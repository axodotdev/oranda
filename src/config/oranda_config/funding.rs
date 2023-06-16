use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyOptExt};
use crate::data::funding::FundingType;

/// Config for displaying funding information on your page
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct FundingConfig {
    pub preferred_funding: Option<FundingType>,
}

impl ApplyLayer for FundingConfig {
    fn apply_layer(&mut self, layer: Self) {
        self.preferred_funding.apply_opt(layer.preferred_funding);
    }
}
