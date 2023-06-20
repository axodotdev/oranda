use camino::Utf8PathBuf;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyOptExt};
use crate::data::funding::FundingType;
use crate::errors::*;

/// Config for displaying funding information on your page
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct FundingConfig {
    pub preferred_funding: Option<FundingType>,
    pub yml_path: Option<String>,
    pub md_path: Option<String>,
}

impl ApplyLayer for FundingConfig {
    fn apply_layer(&mut self, layer: Self) {
        self.preferred_funding.apply_opt(layer.preferred_funding);
        self.yml_path.apply_opt(layer.yml_path);
        self.md_path.apply_opt(layer.md_path);
    }
}

impl FundingConfig {
    // see if we can find a FUNDING.yml, if so update the config path

    /// If we have a FUNDING.yml file, try to find it. If we fail, we disable funding support.
    pub fn find_paths(real_this: &mut Option<Self>) -> Result<()> {
        let Some(this) = real_this else {
            return Ok(())
        };

        // Try to auto-detect the FUNDING.yml if not specified
        if this.yml_path.is_none() {
            let default_yml_path = Utf8PathBuf::from("./.github/FUNDING.yml");
            if default_yml_path.exists() {
                this.yml_path = Some(default_yml_path.to_string());
            }
        }
        // Try to auto-detect funding.md if not specified
        if this.md_path.is_none() {
            let default_md_path = Utf8PathBuf::from("./funding.md");
            if default_md_path.exists() {
                this.md_path = Some(default_md_path.to_string());
            }
        }

        let missing_important_things = this.yml_path.is_none() && this.md_path.is_none();
        let customized_other_things = this.preferred_funding.is_some();
        if missing_important_things {
            // The config is unusable.
            //
            // * If the user customized stuff, error out because they clearly wanted this to work
            // * Otherwise, just disable the feature
            if customized_other_things {
                return Err(OrandaError::FundingConfigInvalid);
            } else {
                *real_this = None;
            }
        }
        Ok(())
    }
}
