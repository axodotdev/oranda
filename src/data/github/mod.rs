use crate::errors::*;

use reqwest::{blocking::Response, header::USER_AGENT};
use serde::{Deserialize, Serialize};

mod repo;
pub use repo::GithubRepo;

/// From the GitHub Rest API
/// as documented here: https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28
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
/// as documented here: https://docs.github.com/en/rest/releases/assets?apiVersion=2022-11-28
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub fn fetch_all(repo: &GithubRepo) -> Result<Vec<GithubRelease>> {
        let (proxy_url, github_url) = Self::build_urls(repo);

        let releases_response = match Self::fetch(&proxy_url)?.error_for_status() {
            Ok(res) => Ok(res),
            // if the proxy fails, fallback to github
            Err(_) => Self::fetch(&github_url)?.error_for_status(),
        };

        Self::parse_response(releases_response?)
    }

    fn build_urls(repo: &GithubRepo) -> (String, String) {
        let proxy_url = format!(
            "https://octolotl.axodotdev.host/releases/{}/{}",
            &repo.owner, &repo.name
        );
        let github_url = format!(
            "https://api.github.com/repos/{}/{}/releases",
            &repo.owner, &repo.name
        );
        (proxy_url, github_url)
    }

    fn fetch(url: &str) -> Result<Response> {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let user_agent = format!("oranda-{}", VERSION);

        Ok(reqwest::blocking::Client::new()
            .get(url)
            .header(USER_AGENT, user_agent)
            .send()?)
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
