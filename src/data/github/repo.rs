use crate::errors::*;

use miette::{miette, IntoDiagnostic};
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
}
