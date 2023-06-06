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

#[derive(Debug, Deserialize)]
pub struct Social {
    pub image: Option<String>,
    pub image_alt: Option<String>,
    pub twitter_account: Option<String>,
}

impl Social {
    /// Merge this value with another layer of itself, preferring the new layer
    pub fn apply_layer(&mut self, layer: Self) {
        if let Some(val) = layer.image {
            self.image = Some(val);
        }
        if let Some(val) = layer.image_alt {
            self.image_alt = Some(val);
        }
        if let Some(val) = layer.twitter_account {
            self.twitter_account = Some(val);
        }
    }
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

impl MdBookConfig {
    /// Merge this value with another layer of itself, preferring the new layer
    pub fn apply_layer(&mut self, layer: Self) {
        if let Some(val) = layer.path {
            self.path = Some(val);
        }
        if let Some(val) = layer.theme {
            self.theme = Some(val);
        }
    }
}

/// Config related to styling your page
#[derive(Debug, Default, Deserialize)]
pub struct StyleConfig {
    pub theme: Option<Theme>,
    pub syntax_theme: Option<SyntaxTheme>,
    #[serde(default)]
    pub additional_css: Vec<String>,
    pub oranda_css_version: Option<String>,
}

impl StyleConfig {
    /// Merge this value with another layer of itself, preferring the new layer
    pub fn apply_layer(&mut self, layer: Self) {
        if let Some(val) = layer.theme {
            self.theme = Some(val);
        }
        if let Some(val) = layer.syntax_theme {
            self.syntax_theme = Some(val);
        }
        if let Some(val) = layer.oranda_css_version {
            self.oranda_css_version = Some(val);
        }
        self.additional_css.extend(layer.additional_css);
    }
    /// Get the theme
    pub fn theme(&self) -> Theme {
        self.theme.unwrap_or(Theme::Dark)
    }
    /// Get the syntax_theme
    pub fn syntax_theme(&self) -> SyntaxTheme {
        self.syntax_theme.unwrap_or(SyntaxTheme::MaterialTheme)
    }
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
