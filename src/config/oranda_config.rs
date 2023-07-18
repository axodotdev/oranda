use axoasset::SourceFile;
use camino::Utf8PathBuf;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::errors::*;
use crate::message::{Message, MessageType};

use super::{BuildLayer, ComponentLayer, MarketingLayer, ProjectLayer, StyleLayer, WorkspaceLayer};

/// Configuration for `oranda` (typically stored in oranda.json)
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OrandaLayer {
    /// Info about the project/application you're making a site for
    ///
    /// All of these values should automatically be sourced from your Cargo.toml or package.json
    /// whenever possible. You should only need to set these if you want to override the value.
    pub project: Option<ProjectLayer>,
    /// Settings for the build/output of the site
    pub build: Option<BuildLayer>,
    /// Settings for social/marketing/analytics
    pub marketing: Option<MarketingLayer>,
    /// Settings for themes/styles of the site
    pub styles: Option<StyleLayer>,
    /// Additional optional components
    pub components: Option<ComponentLayer>,
    /// Workspace configuration
    pub workspace: Option<WorkspaceLayer>,
    /// Field that text-editors can use to fetch the schema for this struct
    ///
    /// We never use this, but we don't want to error out if its set.
    #[serde(rename = "$schema")]
    pub _schema: Option<String>,
}

impl OrandaLayer {
    pub fn load(config_path: &Utf8PathBuf) -> Result<Option<OrandaLayer>> {
        let config_result = SourceFile::load_local(config_path.as_path());

        match config_result {
            Ok(config) => {
                let data: OrandaLayer = config.deserialize_json()?;
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
