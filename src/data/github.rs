use crate::errors::*;

use miette::{miette, IntoDiagnostic};
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubReleaseAsset {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub label: String,
    pub content_type: String,
    pub state: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}

impl GithubRelease {
    pub fn fetch_all(repo: &str) -> Result<Vec<GithubRelease>> {
        let repo_parsed = match Url::parse(repo).into_diagnostic() {
            Ok(parsed) => Ok(parsed),
            Err(e) => Err(OrandaError::RepoParseError {
                repo: repo.to_string(),
                details: e,
            }),
        };
        let binding = repo_parsed?;
        let parts = binding.path_segments().map(|c| c.collect::<Vec<_>>());
        if let Some(url_parts) = parts {
            let url = format!(
                "https://octolotl.axodotdev.host/releases/{}/{}",
                url_parts[0], url_parts[1]
            );
            const VERSION: &str = env!("CARGO_PKG_VERSION");
            let header = format!("oranda-{}", VERSION);

            let releases_response = reqwest::blocking::Client::new()
                .get(url)
                .header(USER_AGENT, header)
                .send()?;

            Self::parse_response(releases_response)
        } else {
            Err(OrandaError::RepoParseError {
                repo: binding.to_string(),
                details: miette!(
                    "This URL is not structured the expected way, expected more segments"
                ),
            })
        }
    }

    fn parse_response(response: reqwest::blocking::Response) -> Result<Vec<GithubRelease>> {
        match response.error_for_status() {
            Ok(r) => match r.json() {
                Ok(releases) => Ok(releases),
                Err(e) => Err(OrandaError::GithubReleaseParseError { details: e }),
            },
            Err(e) => Err(OrandaError::GithubReleasesFetchError { details: e }),
        }
    }

    pub fn has_dist_manifest(&self) -> bool {
        for asset in &self.assets {
            if asset.name == "dist-manifest.json" {
                return true;
            }
        }
        false
    }
}
