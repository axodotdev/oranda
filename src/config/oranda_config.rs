use axoasset::SourceFile;
use camino::Utf8PathBuf;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::errors::*;

use super::{BuildLayer, ComponentLayer, MarketingLayer, ProjectLayer, StyleLayer, WorkspaceLayer};

/// Configuration for `oranda` (typically stored in oranda.json)
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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
        let mut config_path = config_path.to_owned();
        if config_path.extension() == Some("json") {
            if config_path.exists() {
                let config = SourceFile::load_local(config_path.as_path())?;
                return Ok(Some(config.deserialize_json()?));
            } else {
                // Temporary hack
                config_path.set_extension("toml");
            }
        }
        if !config_path.exists() {
            tracing::debug!("No config found, using default values");
            return Ok(None);
        }
        if config_path.extension() == Some("toml") {
            tracing::warn!("!!!Using toml config!!!!");
            let config = SourceFile::load_local(config_path.as_path())?;
            return Ok(Some(config.deserialize_toml()?));
        }

        tracing::debug!("No config found, using default values");
        Ok(None)
    }
}
