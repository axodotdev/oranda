use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyOptExt};
use crate::data::funding::FundingType;

/// Config for displaying funding information on your page
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct FundingConfig {
    pub preferred_funding: Option<FundingType>,
    pub path: Option<String>,
}

impl ApplyLayer for FundingConfig {
    fn apply_layer(&mut self, layer: Self) {
        self.preferred_funding.apply_opt(layer.preferred_funding);
        self.path.apply_opt(layer.path);
    }
}

impl FundingConfig {
    /// If we have a FUNDING.yml file, try to find it. If we fail, we disable funding support.
    fn find(&self) {
        if self.path.is_none() {
            // see if we can find a FUNDING.yml, if so update the config path
            let funding_yml = Utf8PathBuf::from("./.github/FUNDING.yml");
            let funding_md = Utf8PathBuf::from("./funding.md");

            if funding_yml.exists() {
                funding_cfg.path = Some(funding_path.to_string());
                return;
            } else if self.preferred.is_some() {
                // throw an error because there's no funding_yml
            } else if funding_md.exists() {
                // return and the fundign page will just be funding.md
                return;
            }

            // if all else fails
            self.funding = None;
        }
    }
}
