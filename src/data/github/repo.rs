use crate::errors::*;

use crate::site::funding::FundingResponse;
use miette::{miette, IntoDiagnostic};
use reqwest::blocking::Response;
use reqwest::header::USER_AGENT;
use reqwest::StatusCode;
use url::Url;

/// Represents a GitHub repository that we can query things about.
#[derive(Debug, Clone)]
pub struct GithubRepo {
    /// The repository owner.
    pub owner: String,
    /// The repository name.
    pub name: String,
}

impl GithubRepo {
    /// Constructs a new Github repository from a "owner/name" string. Notably, this does not check
    /// whether the repo actually exists.
    pub fn from_url(repo_url: &str) -> Result<Self> {
        let binding =
            Url::parse(repo_url)
                .into_diagnostic()
                .map_err(|e| OrandaError::RepoParseError {
                    repo: repo_url.to_string(),
                    details: e,
                })?;
        let segment_list = binding.path_segments().map(|c| c.collect::<Vec<_>>());
        if let Some(segments) = segment_list {
            if segments.len() == 2 {
                return Ok(Self {
                    owner: segments[0].to_string(),
                    name: segments[1].to_string(),
                });
            }
        }
        Err(OrandaError::RepoParseError {
            repo: binding.to_string(),
            details: miette!("This URL is not structured the expected way, expected more segments"),
        })
    }

    /// Fetches the FUNDING.yml file from Github. Returns `Ok(Some(FundingResponse))` if one exists, and
    /// `Ok(None)` if it doesn't.
    pub fn fetch_funding_yaml(&self) -> Result<Option<FundingResponse>> {
        let proxy_url = format!(
            "https://octolotl.axodotdev.host/funding/{}/{}",
            self.owner, self.name
        );
        let github_url = format!(
            "https://api.github.com/repos/{}/{}/contents/.github/FUNDING.yml",
            self.owner, self.name
        );

        let res = match Self::fetch(&proxy_url)?.error_for_status() {
            Ok(res) => Ok(res),
            Err(_) => Self::fetch(&github_url)?.error_for_status(),
        };

        match res?.error_for_status() {
            Ok(r) => Ok(Some(r.json()?)),
            Err(e) if e.status().is_some() && e.status().unwrap() == StatusCode::NOT_FOUND => {
                Ok(None)
            }
            Err(e) => Err(OrandaError::GithubFundingFetchError { details: e }),
        }
    }

    /// FIXME: Replace this function with a call to the octolotl lib
    fn fetch(url: &str) -> Result<Response> {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let user_agent = format!("oranda-{}", VERSION);

        Ok(reqwest::blocking::Client::new()
            .get(url)
            .header(USER_AGENT, user_agent)
            .send()?)
    }
}
