use std::path::Path;

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

#[derive(Debug, Deserialize)]
pub struct OrandaConfig {
    pub description: Option<String>,
    pub dist_dir: Option<String>,
    pub static_dir: Option<String>,
    pub homepage: Option<String>,
    pub name: Option<String>,
    pub no_header: Option<bool>,
    pub readme_path: Option<String>,
    pub theme: Option<Theme>,
    pub additional_css: Option<Vec<String>>,
    pub repository: Option<String>,
    pub syntax_theme: Option<SyntaxTheme>,
    pub analytics: Option<Analytics>,
    pub additional_pages: Option<Vec<String>>,
    pub social: Option<Social>,
    pub artifacts: Option<Artifacts>,
    pub version: Option<String>,
    pub logo: Option<String>,
    pub favicon: Option<String>,
    pub path_prefix: Option<String>,
    pub license: Option<String>,
    pub md_book: Option<String>,
}

impl OrandaConfig {
    pub fn load(config_path: &Path) -> Result<Option<OrandaConfig>> {
        let config_path = config_path.to_string_lossy();
        let msg = format!("Loading config at {}", config_path);
        Message::new(MessageType::Info, &msg).print();
        tracing::info!("{}", &msg);
        let config_future = axoasset::load_string(&config_path);
        let config_result = tokio::runtime::Handle::current().block_on(config_future);

        match config_result {
            Ok(config) => {
                let data: OrandaConfig = serde_json::from_str(config.as_str())?;
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
