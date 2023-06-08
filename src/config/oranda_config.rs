use std::collections::HashMap;

use axoasset::SourceFile;
use camino::Utf8PathBuf;
use serde::Deserialize;

use crate::config::analytics::Analytics;
use crate::config::theme::Theme;
use crate::errors::*;
use crate::message::{Message, MessageType};
use crate::site::markdown::SyntaxTheme;

use crate::config::artifacts::Artifacts;
use crate::data::funding::FundingType;

#[derive(Debug, Deserialize)]
pub struct Social {
    pub image: Option<String>,
    pub image_alt: Option<String>,
    pub twitter_account: Option<String>,
}

/// Config for us building and integrating your mdbook
#[derive(Debug, Default, Deserialize)]
pub struct MdBookConfig {
    /// Path to the mdbook
    ///
    /// If not set we will attempt to auto-detect
    pub path: Option<String>,
    /// Whether to enable the custom oranda/axo theme
    pub theme: Option<bool>,
}

/// Config related to styling your page
#[derive(Debug, Deserialize)]
pub struct StyleConfig {
    pub theme: Theme,
    pub syntax_theme: SyntaxTheme,
    pub additional_css: Vec<String>,
    pub oranda_css_version: Option<String>,
}

impl Default for StyleConfig {
    fn default() -> Self {
        StyleConfig {
            theme: Theme::Dark,
            additional_css: vec![],
            syntax_theme: SyntaxTheme::MaterialTheme,
            oranda_css_version: None,
        }
    }
}

/// Config for displaying funding information on your page
#[derive(Debug, Default, Deserialize)]
pub struct FundingConfig {
    pub preferred_funding: Option<FundingType>,
}

#[derive(Debug, Deserialize)]
pub struct OrandaConfig {
    pub description: Option<String>,
    pub dist_dir: Option<String>,
    pub static_dir: Option<String>,
    pub homepage: Option<String>,
    pub name: Option<String>,
    pub no_header: Option<bool>,
    pub readme_path: Option<String>,
    pub repository: Option<String>,
    pub analytics: Option<Analytics>,
    pub additional_pages: Option<HashMap<String, String>>,
    pub social: Option<Social>,
    pub artifacts: Option<Artifacts>,
    pub version: Option<String>,
    pub logo: Option<String>,
    pub favicon: Option<String>,
    pub path_prefix: Option<String>,
    pub license: Option<String>,
    /// Config for mdbook
    ///
    /// We allow this to be set to just `false` to give the user
    /// an easy way to force-disable any errant autodetection.
    /// Setting it to `true` is allowed but equivalent to `None`.
    #[serde(alias = "md_book")]
    pub mdbook: Option<BoolOr<MdBookConfig>>,
    pub changelog: Option<bool>,
    pub styles: Option<StyleConfig>,
    pub funding: Option<BoolOr<FundingConfig>>,
}

impl OrandaConfig {
    pub fn load(config_path: &Utf8PathBuf) -> Result<Option<OrandaConfig>> {
        let msg = format!("Loading config at {}", config_path);
        Message::new(MessageType::Info, &msg).print();
        tracing::info!("{}", &msg);
        let config_result = SourceFile::load_local(config_path.as_path());

        match config_result {
            Ok(config) => {
                let data: OrandaConfig = config.deserialize_json()?;
                tracing::debug!("{:?}", data);
                Ok(Some(data))
            }
            Err(_) => {
                Message::new(MessageType::Info, "No config found, using default values").print();
                Ok(None)
            }
        }
    }
}

/// A value or just a boolean
///
/// This allows us to have a simple yes/no version of a config while still
/// allowing for a more advanced version to exist.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum BoolOr<T> {
    /// They gave the simple bool
    Bool(bool),
    /// They gave a more interesting value
    Val(T),
}
