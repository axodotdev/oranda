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
    pub project: ProjectLayer,
    /// Info about the build/output
    pub build: BuildLayer,
    /// Info about social/marketing/analytics
    pub marketing: MarketingLayer,
    /// Info about layout/themes
    pub styles: StyleLayer,
    /// Additional optional components
    pub components: ComponentLayer,
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
