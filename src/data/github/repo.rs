use crate::errors::*;

use miette::{miette, IntoDiagnostic};
use url::Url;

#[derive(Debug, Clone)]
pub struct GithubRepo {
    pub owner: String,
    pub name: String,
}

impl GithubRepo {
    pub fn from(repo_url: &str) -> Result<Self> {
        let repo_parsed = match Url::parse(repo_url).into_diagnostic() {
            Ok(parsed) => Ok(parsed),
            Err(e) => Err(OrandaError::RepoParseError {
                repo: repo_url.to_string(),
                details: e,
            }),
        };
        let binding = repo_parsed?;
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
