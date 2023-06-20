use std::collections::HashMap;

use axoasset::SourceFile;
use camino::Utf8PathBuf;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::errors::*;
use crate::message::{Message, MessageType};

pub use analytics::AnalyticsConfig;
pub use artifacts::ArtifactsConfig;
pub use funding::FundingConfig;
pub use mdbook_config::MdBookConfig;
pub use social::SocialConfig;
pub use style::{StyleConfig, StyleOpts};

pub mod analytics;
pub mod artifacts;
mod funding;
mod mdbook_config;
mod social;
mod style;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct OrandaConfig {
    pub description: Option<String>,
    pub dist_dir: Option<String>,
    pub static_dir: Option<String>,
    pub homepage: Option<String>,
    pub name: Option<String>,
    pub no_header: Option<bool>,
    pub readme_path: Option<String>,
    pub repository: Option<String>,
    pub analytics: Option<AnalyticsConfig>,
    pub additional_pages: Option<HashMap<String, String>>,
    pub social: Option<SocialConfig>,
    pub artifacts: Option<ArtifactsConfig>,
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
    pub styles: Option<StyleOpts>,
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
#[derive(Deserialize, Debug, JsonSchema)]
#[serde(untagged)]
pub enum BoolOr<T> {
    /// They gave the simple bool
    Bool(bool),
    /// They gave a more interesting value
    Val(T),
}
