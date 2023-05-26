use axoasset::SourceFile;
use cargo_dist_schema::DistManifest;

use crate::data::{cargo_dist, github::GithubRelease, GithubRepo};
use crate::errors::*;
use crate::message::{Message, MessageType};

#[derive(Clone, Debug)]
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
        let res: serde_json::Value = serde_json::from_str(&response.text().await?)?;
        let pretty_response = serde_json::to_string_pretty(&res)?;
        Ok(
            match SourceFile::new("", pretty_response).deserialize_json::<DistManifest>() {
                Ok(manifest) => Some(manifest),
                Err(e) => {
                    let msg = format!("Failed to parse dist-manifest for release {tag}.\nDetails:{e}\n\nSkipping...");
                    Message::new(MessageType::Warning, &msg).print();
                    None
                }
            },
        )
    }
}
