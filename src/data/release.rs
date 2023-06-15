use axoasset::SourceFile;
use cargo_dist_schema::DistManifest;
use serde::{Deserialize, Serialize};

use crate::data::{cargo_dist, github::GithubRelease, GithubRepo};
use crate::errors::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Release {
    pub manifest: Option<DistManifest>,
    pub source: GithubRelease,
}

impl Release {
    pub async fn new(
        gh_release: GithubRelease,
        repo: &GithubRepo,
        cargo_dist: bool,
    ) -> Result<Self> {
        let manifest = if cargo_dist {
            Self::fetch_manifest(&gh_release, repo).await?
        } else {
            None
        };
        Ok(Self {
            manifest,
            source: gh_release,
        })
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
