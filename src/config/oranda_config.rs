use axoasset::SourceFile;
use camino::Utf8PathBuf;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::errors::*;
use crate::message::{Message, MessageType};

use super::{BuildLayer, ComponentLayer, MarketingLayer, ProjectLayer, StyleLayer};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct OrandaConfig {
    /// Info about the project/application
    pub project: Option<ProjectLayer>,
    /// Info about the build/output
    pub build: Option<BuildLayer>,
    /// Info about social/marketing/analytics
    pub marketing: Option<MarketingLayer>,
    /// Info about layout/themes
    pub styles: Option<StyleLayer>,
    /// Additional optional components
    pub components: Option<ComponentLayer>,
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
