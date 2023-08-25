use camino::Utf8PathBuf;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::config::{ApplyLayer, ApplyOptExt};
use crate::data::funding::FundingType;
use crate::errors::*;

/// Config for displaying funding information on your page (complete version)
#[derive(Debug, Clone)]
pub struct FundingConfig {
    pub preferred_funding: Option<FundingType>,
    pub yml_path: Option<String>,
    pub md_path: Option<String>,
}
/// Settings for displaying funding information on your page
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FundingLayer {
    /// A funding method to make larger/focused to encourage over all others
    pub preferred_funding: Option<FundingType>,
    /// A path to a github-format FUNDING.yml file
    ///
    /// We parse this out to get a list of funding sources.
    ///
    /// By default we try to find this at "./.github/FUNDING.yml"
    pub yml_path: Option<String>,
    /// A relative path to a freeform github-flavor markdown file
    /// whose contents will be included on your funding page.
    ///
    /// By default we try to find this at "./funding.md"
    pub md_path: Option<String>,
}

impl Default for FundingConfig {
    fn default() -> Self {
        FundingConfig {
            preferred_funding: None,
            yml_path: None,
            md_path: None,
        }
    }
}
impl ApplyLayer for FundingConfig {
    type Layer = FundingLayer;
    fn apply_layer(&mut self, layer: Self::Layer) {
        // This is intentionally written slightly cumbersome to make you update this
        let FundingLayer {
            preferred_funding,
            yml_path,
            md_path,
        } = layer;
        self.preferred_funding.apply_opt(preferred_funding);
        self.yml_path.apply_opt(yml_path);
        self.md_path.apply_opt(md_path);
    }
}

impl FundingConfig {
    /// If we have a FUNDING.yml file, try to find it. If we fail, we disable funding support.
    pub fn find_paths(config: &mut Option<Self>, start_dir: &Path) -> Result<()> {
        // If this is None, we were force-disabled and shouldn't auto-detect
        let Some(this) = config else { return Ok(()) };

        // Try to auto-detect the FUNDING.yml if not specified
        if this.yml_path.is_none() {
            let default_yml_path =
                Utf8PathBuf::from(format!("{}/.github/FUNDING.yml", start_dir.display()));
            if default_yml_path.exists() {
                this.yml_path = Some(default_yml_path.to_string());
            }
        }
        // Try to auto-detect funding.md if not specified
        if this.md_path.is_none() {
            let default_md_path = Utf8PathBuf::from(format!("{}/funding.md", start_dir.display()));
            if default_md_path.exists() {
                this.md_path = Some(default_md_path.to_string());
            }
        }

        // This is intentionally written slightly cumbersome to make you update this
        let FundingConfig {
            preferred_funding,
            yml_path,
            md_path,
        } = this;
        let cant_find_files = yml_path.is_none() && md_path.is_none();
        let has_user_config = preferred_funding.is_some();
        if cant_find_files {
            // The config is unusable.
            //
            // * If the user customized stuff, error out because they clearly wanted this to work
            // * Otherwise, just disable the feature
            if has_user_config {
                return Err(OrandaError::FundingConfigInvalid);
            } else {
                *config = None;
            }
        }
        Ok(())
    }
}
