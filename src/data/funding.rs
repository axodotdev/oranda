use crate::config::Config;
use crate::errors::{OrandaError, Result};
use crate::site::markdown::to_html;
use axoasset::LocalAsset;
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

/// Contents of the FUNDING.yml file. Needs to follow GitHub's spec, since we serialize from it,
/// the most accurate resource for this seems to be here: <https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/displaying-a-sponsor-button-in-your-repository#about-funding-files>
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Funding {
    pub github: Option<Vec<String>>,
    pub patreon: Option<String>,
    pub open_collective: Option<String>,
    pub ko_fi: Option<String>,
    pub tidelift: Option<String>,
    pub community_bridge: Option<String>,
    pub liberapay: Option<String>,
    pub issuehunt: Option<String>,
    pub custom: Option<Vec<String>>,
    /// Content read from the optional Markdown file
    #[serde(skip)]
    pub docs_content: Option<String>,
}

/// An enumeration of different supported funding providers.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FundingType {
    Github,
    Patreon,
    #[serde(rename = "open_collective")]
    OpenCollective,
    KoFi,
    Tidelift,
    #[serde(rename = "community_bridge")]
    CommunityBridge,
    Issuehunt,
    Custom,
}

impl Funding {
    /// Creates a new Funding struct by attempting to read from the FUNDING.yml, and the docs file.
    pub fn new(config: &Config) -> Result<Self> {
        let mut funding = match load_funding_file() {
            Ok(Some(res)) => Ok(parse_response(res)?),
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

fn parse_response(contents: String) -> Result<Funding> {
    let deserialized_map = serde_yaml::from_str(&contents);
    match deserialized_map {
        Ok(yaml) => Ok(yaml),
        Err(e) => Err(OrandaError::GithubFundingParseError {
            details: e.to_string(),
        }),
    }
}
