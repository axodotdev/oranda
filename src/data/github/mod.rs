use crate::errors::*;

use axoasset::SourceFile;
use serde::{Deserialize, Serialize};

mod repo;
pub use repo::GithubRepo;

/// From the GitHub Rest API
/// as documented here: <https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GithubRelease {
    pub url: String,
    pub assets_url: String,
    pub html_url: String,
    pub id: i64,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<GithubReleaseAsset>,
    pub tarball_url: String,
    pub zipball_url: String,
    pub body: Option<String>,
}

/// From the GitHub Rest API
/// as documented here: <https://docs.github.com/en/rest/releases/assets?apiVersion=2022-11-28>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GithubReleaseAsset {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub label: Option<String>,
    pub content_type: String,
    pub state: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}

impl GithubRelease {
    pub async fn fetch_all(repo: &GithubRepo) -> Result<Vec<GithubRelease>> {
        let request = octolotl::request::Releases::new(&repo.owner, &repo.name);
        match octolotl::Request::send(&request, true).await {
            Ok(r) => {
                let res: serde_json::Value = serde_json::from_str(&r.text().await?)?;
                let pretty_response = serde_json::to_string_pretty(&res)?;
                Ok(
                    SourceFile::new("", pretty_response)
                        .deserialize_json::<Vec<GithubRelease>>()?,
                )
            }
            Err(e) => Err(OrandaError::GithubReleasesFetchError { details: e }),
        }
    }

    pub fn has_dist_manifest(&self) -> bool {
        self.assets.iter().any(|a| a.name == "dist-manifest.json")
    }

    pub fn asset_url<'a>(&'a self, asset_name: &'a str) -> Option<&'a str> {
        for asset in &self.assets {
            if asset.name == asset_name {
                return Some(&asset.browser_download_url);
            }
        }
        None
    }
}
