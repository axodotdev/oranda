use crate::config::Config;
use crate::data::github::GithubRepo;
use crate::errors::{OrandaError, Result};
use base64::engine::general_purpose;
use base64::Engine;
use serde::{Deserialize, Serialize};

/// Contents of the HTTP response. Serialized from JSON.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct FundingResponse {
    pub name: String,
    pub content: String,
}

/// Contents of the FUNDING.yml file. Needs to follow GitHub's spec, since we serialize from it,
/// the most accurate resource for this seems to be here: https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/displaying-a-sponsor-button-in-your-repository#about-funding-files
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Funding {
    pub github: Option<Vec<String>>,
    pub patreon: Option<String>,
    pub open_collective: Option<String>,
    pub ko_fi: Option<String>,
    pub tidelift: Option<String>,
    pub community_bridge: Option<String>,
    pub liberapay: Option<String>,
    pub issuehunt: Option<String>,
    pub lfx_crowdfunding: Option<String>,
    pub custom: Option<Vec<String>>,
}

impl Funding {
    /// Creates a new Funding struct by pulling from the GitHub repository specified in the
    /// configuration. Assumes that a repository is set (i.e. unwraps on the repo config key),
    /// so check for existence beforehand.
    pub fn new(config: &Config) -> Result<Self> {
        let repo = GithubRepo::from_url(&config.repository.clone().unwrap())?;
        match repo.fetch_funding_yaml() {
            Ok(Some(res)) => Ok(Self::parse_response(res)?),
            Ok(None) => Ok(Self::default()),
            Err(e) => Err(e),
        }
    }

    fn parse_response(res: FundingResponse) -> Result<Self> {
        let string_yaml = &general_purpose::STANDARD
            .decode(res.content.replace('\n', ""))
            .unwrap();
        match std::str::from_utf8(string_yaml) {
            Ok(parsed_yaml) => {
                let deserialized_map = serde_yaml::from_str(parsed_yaml);
                match deserialized_map {
                    Ok(yaml) => Ok(yaml),
                    Err(e) => Err(OrandaError::GithubFundingParseError {
                        details: e.to_string(),
                    }),
                }
            }
            Err(e) => Err(OrandaError::GithubFundingParseError {
                details: e.to_string(),
            }),
        }
    }
}
