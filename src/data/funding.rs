use crate::config::{Config, FundingConfig};
use crate::errors::{OrandaError, Result};
use crate::site::markdown::to_html;
use axoasset::LocalAsset;
use camino::{Utf8Path, Utf8PathBuf};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Funding data-struct.
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Funding {
    /// Contents of the FUNDING.yml file.
    pub content: HashMap<FundingType, FundingContent>,
    /// Content read from the optional Markdown file
    pub docs_content: Option<String>,
}

/// An enumeration of different supported funding providers. Represents the "key" portion of a
/// funding.yml entry.
#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FundingType {
    Github,
    Patreon,
    OpenCollective,
    KoFi,
    Tidelift,
    CommunityBridge,
    Issuehunt,
    Liberapay,
    Custom,
}

/// An enum expressing the different types of values that an entry in FUNDING.yml can have.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum FundingContent {
    One(String),
    Multiple(Vec<String>),
}

impl Funding {
    /// Creates a new Funding struct by attempting to read from the FUNDING.yml, and the docs file.
    pub fn new(config: &Config) -> Result<Self> {
        let path_to_funding = Self::path(&config.funding);
        let mut funding = match LocalAsset::load_string(&path_to_funding) {
            Ok(res) => {
                let parsed_response = parse_response(res)?;
                Self {
                    content: parsed_response,
                    docs_content: None,
                }
            }
            Err(e) => {
                let warning = OrandaError::FundingLoadFailed {
                    path: path_to_funding,
                    details: e,
                };
                eprintln!("{:?}", miette::Report::new(warning));
                Self::default()
            }
        };
        if let Ok(res) = LocalAsset::load_string("funding.md") {
            let html = to_html(&res, &config.styles.syntax_theme())?;
            funding.docs_content = Some(html);
        }

        Ok(funding)
    }

    fn path(config: &Option<FundingConfig>) -> Utf8PathBuf {
        const FUNDING_FILENAME: &str = "FUNDING.yml";
        const FUNDING_FILEPATH: &str = "./github";

        if let Some(cfg) = config {
            if let Some(path) = &cfg.path {
                return Utf8Path::new(&path).into();
            }
        }
        Utf8Path::new(FUNDING_FILEPATH).join(FUNDING_FILENAME)
    }
}

fn parse_response(contents: String) -> Result<HashMap<FundingType, FundingContent>> {
    let deserialized_map = serde_yaml::from_str(&contents);
    match deserialized_map {
        Ok(yaml) => Ok(yaml),
        Err(e) => Err(OrandaError::GithubFundingParseError {
            details: e.to_string(),
        }),
    }
}
