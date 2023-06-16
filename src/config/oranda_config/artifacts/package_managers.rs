use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyOptExt};

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct PackageManagersConfig {
    #[serde(default)]
    pub preferred: Option<IndexMap<String, String>>,
    #[serde(default)]
    pub additional: Option<IndexMap<String, String>>,
}

impl ApplyLayer for PackageManagersConfig {
    fn apply_layer(&mut self, layer: Self) {
        self.preferred.apply_opt(layer.preferred);
        self.additional.apply_opt(layer.additional);
    }
}

impl PackageManagersConfig {
    pub fn has_some(&self) -> bool {
        self.preferred.is_some() || self.additional.is_some()
    }
}
