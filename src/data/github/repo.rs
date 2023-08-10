use std::fmt;

use crate::data::GithubRelease;
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

impl fmt::Display for GithubRepo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}/{})", self.owner, self.name)
    }
}

#[derive(Debug)]
enum GithubRepoInput {
    Url(String),
    Ssh(String),
}

impl GithubRepoInput {
    fn new(repo_string: String) -> Result<Self> {
        // Handle git+https just the same as https
        if repo_string.starts_with("https") || repo_string.starts_with("git+https") {
            Ok(Self::Url(repo_string))
        } else if repo_string.starts_with("git@") {
            Ok(Self::Ssh(repo_string))
        } else {
            let err = OrandaError::UnknownRepoStyle { url: repo_string };
            Err(err)
        }
    }

    fn parse(self) -> Result<GithubRepo> {
        match self {
            Self::Url(s) => Ok(Self::parse_url(s)?),
            Self::Ssh(s) => Ok(Self::parse_ssh(s)?),
        }
    }

    fn parse_url(repo_string: String) -> Result<GithubRepo> {
        let parsed = Url::parse(&repo_string).into_diagnostic().map_err(|e| {
            OrandaError::RepoParseError {
                repo: repo_string.to_string(),
                details: e,
            }
        })?;
        if parsed.domain() != Some("github.com") {
            return Err(OrandaError::RepoParseError {
                repo: repo_string,
                details: miette!("For now, we can only detect releases (artifacts.auto: true) for github repository urls.")
            });
        }
        let segment_list = parsed.path_segments().map(|c| c.collect::<Vec<_>>());
        if let Some(segments) = segment_list {
            if segments.len() >= 2 {
                let owner = segments[0].to_string();
                let name = Self::remove_git_suffix(segments[1].to_string());
                let rest_is_empty = segments.iter().skip(2).all(|s| s.trim().is_empty());
                if rest_is_empty {
                    return Ok(GithubRepo { owner, name });
                }
            }
        }
        Err(OrandaError::RepoParseError {
                    repo: repo_string,
                    details: miette!("We found a repo url but we had trouble parsing it. Please make sure it's entered correctly. This may be an error, and if so you should file an issue."),
                })
    }

    fn parse_ssh(repo_string: String) -> Result<GithubRepo> {
        let core = Self::remove_git_suffix(Self::remove_git_prefix(repo_string.clone())?);
        let segments: Vec<&str> = core.split('/').collect();
        if !segments.is_empty() && segments.len() >= 2 {
            let owner = segments[0].to_string();
            let name = Self::remove_git_suffix(segments[1].to_string());
            let rest_is_empty = segments.iter().skip(2).all(|s| s.trim().is_empty());
            if rest_is_empty {
                return Ok(GithubRepo { owner, name });
            }
        }
        Err(OrandaError::RepoParseError {
            repo: repo_string,
            details: miette!("We found a repo url but we had trouble parsing it. Please make sure it's entered correctly. This may be an error, and if so you should file an issue."),
        })
    }

    fn remove_git_prefix(s: String) -> Result<String> {
        let prefix = "git@github.com:";
        if s.starts_with(prefix) {
            Ok(s.replace(prefix, ""))
        } else {
            Err(OrandaError::RepoParseError {
                repo: s,
                details: miette!("For now, we can only detect releases (artifacts.auto: true) for github repository urls.")
            })
        }
    }

    fn remove_git_suffix(s: String) -> String {
        let suffix = ".git";
        if s.ends_with(suffix) {
            s.replace(suffix, "")
        } else {
            s
        }
    }
}

impl GithubRepo {
    /// Constructs a new Github repository from a "owner/name" string. Notably, this does not check
    /// whether the repo actually exists.
    pub fn from_url(repo_url: &str) -> Result<Self> {
        GithubRepoInput::new(repo_url.to_string())?.parse()
    }

    pub fn has_releases(&self) -> Result<bool> {
        if let Ok(releases) =
            tokio::runtime::Handle::current().block_on(GithubRelease::fetch_all(self))
        {
            if releases.is_empty() {
                Ok(false)
            } else {
                Ok(true)
            }
        } else {
            let warning = OrandaError::ReleasesCheckFailed {
                repo: self.to_string(),
            };
            eprintln!("{:?}", miette::Report::new(warning));
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_an_https_repo_string() {
        let input = "https://github.com/axodotdev/oranda";
        let actual_owner = "axodotdev";
        let actual_name = "oranda";
        let parsed = GithubRepo::from_url(input).unwrap();
        assert_eq!(parsed.owner, actual_owner);
        assert_eq!(parsed.name, actual_name);
    }

    #[test]
    fn it_parses_an_https_repo_string_with_dot_git() {
        let input = "https://github.com/axodotdev/oranda.git";
        let actual_owner = "axodotdev";
        let actual_name = "oranda";
        let parsed = GithubRepo::from_url(input).unwrap();
        assert_eq!(parsed.owner, actual_owner);
        assert_eq!(parsed.name, actual_name);
    }

    #[test]
    fn it_parses_an_ssh_repo_string() {
        let input = "git@github.com:axodotdev/oranda.git";
        let actual_owner = "axodotdev";
        let actual_name = "oranda";
        let parsed = GithubRepo::from_url(input).unwrap();
        assert_eq!(parsed.owner, actual_owner);
        assert_eq!(parsed.name, actual_name);
    }
}
