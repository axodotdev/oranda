use axoasset::SourceFile;
use cargo_dist_schema::DistManifest;
use chrono::DateTime;
use serde::Serialize;

use crate::config::ArtifactsConfig;
use crate::data::{cargo_dist, github::GithubRelease, GithubRepo};
use crate::errors::*;

use super::artifacts::ReleaseArtifacts;

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Debug, Clone)]
pub enum ReleaseSource {
    Github(GithubRelease),
    CurrentState(CurrentStateRelease),
}

#[derive(Serialize, Debug, Clone)]
pub struct CurrentStateRelease {
    pub version: Option<String>,
    pub date: Option<String>,
    pub prerelease: bool,
}

impl ReleaseSource {
    /// Get a freeform string that identifies a version/release.
    ///
    /// Can be things like "0.1.0", "v0.1.0", or "css-v0.1.0".
    pub fn version_tag(&self) -> &str {
        match self {
            ReleaseSource::Github(src) => &src.tag_name,
            ReleaseSource::CurrentState(src) => src.version.as_deref().unwrap_or("current"),
        }
    }

    /// Whether this is a prerelease
    pub fn is_prerelease(&self) -> bool {
        match self {
            ReleaseSource::Github(src) => src.prerelease,
            ReleaseSource::CurrentState(src) => src.prerelease,
        }
    }

    /// The date this was published (can be anything, but we do optionally try to parse/format it)
    pub fn date(&self) -> Option<&str> {
        match self {
            ReleaseSource::Github(src) => Some(src.published_at.as_str()),
            ReleaseSource::CurrentState(src) => src.date.as_deref(),
        }
    }

    /// Get a pretty formatted version of the date
    pub fn formatted_date(&self) -> Option<String> {
        self.date().map(|date| {
            if let Ok(parsed_date) = DateTime::parse_from_rfc3339(date) {
                parsed_date.format("%b %e %Y at %R UTC").to_string()
            } else {
                date.to_owned()
            }
        })
    }

    /// The display name of the release
    pub fn name(&self) -> Option<&str> {
        match self {
            ReleaseSource::Github(src) => src.name.as_deref(),
            ReleaseSource::CurrentState(_src) => None,
        }
    }

    /// Get the body of the release (notes/description)
    pub(crate) fn body(&self) -> Option<&str> {
        match self {
            ReleaseSource::Github(src) => src.body.as_deref(),
            ReleaseSource::CurrentState(_src) => None,
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Release {
    pub manifest: Option<DistManifest>,
    pub source: ReleaseSource,
    pub artifacts: ReleaseArtifacts,
}

impl Release {
    pub async fn new(
        source: ReleaseSource,
        repo: Option<&GithubRepo>,
        artifacts_config: Option<&ArtifactsConfig>,
    ) -> Result<Self> {
        // If artifacts are disabled then bail out, because all this code is just artifacts stuff
        let Some(artifacts_config) = artifacts_config else {
            return Ok(Self { manifest: None, source, artifacts: ReleaseArtifacts::new(None)} )
        };

        let manifest = if let (ReleaseSource::Github(gh_release), Some(repo)) = (&source, repo) {
            if artifacts_config.cargo_dist {
                Self::fetch_manifest(gh_release, repo).await?
            } else {
                None
            }
        } else {
            // FIXME: warn if cargo-dist enabled?
            None
        };

        // Compute the artifacts for this release
        //
        // In the future with multi-tenant oranda support, this None
        // can be replaced with the name of the app we want to focus in on
        let mut artifacts = ReleaseArtifacts::new(None);

        // Add data from various sources
        if let ReleaseSource::Github(gh_release) = &source {
            artifacts.add_github(gh_release);
        }
        if let Some(manifest) = &manifest {
            artifacts.add_cargo_dist(manifest);
        }
        artifacts.add_package_managers(artifacts_config);
        artifacts.add_inference();

        // Compute the final result
        artifacts.select_installers(artifacts_config);

        Ok(Self {
            manifest,
            source,
            artifacts,
        })
    }

    /// Gets whether any platform has actual targets to suggest
    pub fn has_installers(&self) -> bool {
        !self.artifacts.installers_by_target().is_empty()
    }

    async fn fetch_manifest(
        gh_release: &GithubRelease,
        repo: &GithubRepo,
    ) -> Result<Option<DistManifest>> {
        let tag = &gh_release.tag_name;
        if gh_release.has_dist_manifest() {
            let request = octolotl::request::ReleaseAsset::new(
                &repo.owner,
                &repo.name,
                tag,
                cargo_dist::MANIFEST_FILENAME,
            );
            let response = octolotl::Request::send(&request, true)
                .await?
                .error_for_status()?;

            Ok(Self::parse_response(response, tag).await?)
        } else {
            Ok(None)
        }
    }

    async fn parse_response(
        response: reqwest::Response,
        tag: &str,
    ) -> Result<Option<DistManifest>> {
        let res = response.text().await?;
        let src = SourceFile::new("dist-manifest.json", res);
        Ok(match src.deserialize_json::<DistManifest>() {
            Ok(manifest) => Some(manifest),
            Err(e) => {
                // Try partially parsing the manifest to get schema version info
                let info = cargo_dist_schema::check_version(src.contents());
                if let Some(info) = info {
                    if info.format.unsupported() {
                        // Don't mention it -- nothing's wrong, it's just too old
                    } else {
                        let schema_version = info.version.to_string();
                        let parser_version = cargo_dist_schema::SELF_VERSION.to_owned();
                        let tag = tag.to_owned();
                        let err = OrandaError::CargoDistManifestPartial {
                            schema_version,
                            parser_version,
                            tag,
                            details: e,
                        };
                        let report = miette::Report::new(err);
                        eprintln!("{report:?}");
                    }
                } else {
                    let tag = tag.to_owned();
                    let err = OrandaError::CargoDistManifestMalformed { tag, details: e };
                    let report = miette::Report::new(err);
                    eprintln!("{report:?}");
                }
                None
            }
        })
    }
}
