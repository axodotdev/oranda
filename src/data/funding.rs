use crate::config::Config;
use crate::errors::{OrandaError, Result};
use crate::site::markdown::to_html;
use axoasset::LocalAsset;
use camino::Utf8PathBuf;
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
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
        let mut funding = match load_funding_file() {
            Ok(Some(res)) => {
                let parsed_response = parse_response(res)?;
                Ok(Self {
                    content: parsed_response,
                    docs_content: None,
                })
            }
            Ok(None) => Ok(Self::default()),
            Err(e) => Err(e),
        }?;
        if let Ok(Some(res)) = load_funding_docs() {
            let html = to_html(&res, &config.styles.syntax_theme)?;
            funding.docs_content = Some(html);
        }

        Ok(funding)
    }
}

/// Loads the FUNDING.yml file from the local file system. Returns
/// `Ok(Some(String))` if the file was found, and `Ok(None)` if it
/// wasn't.
pub fn load_funding_file() -> Result<Option<String>> {
    load_generic_file(".github/FUNDING.yml")
}

/// Loads a `funding.md` file from the root directory, to serve as documentation
/// for the generated funding page. Returns the same as the above function.
pub fn load_funding_docs() -> Result<Option<String>> {
    // FIXME: Do we want this file to be FUNDING.md?
    load_generic_file("funding.md")
}

fn load_generic_file(path: &str) -> Result<Option<String>> {
    let path = Utf8PathBuf::from(path);
    if path.exists() {
        Ok(Some(LocalAsset::load_string(path)?))
    } else {
        Ok(None)
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
