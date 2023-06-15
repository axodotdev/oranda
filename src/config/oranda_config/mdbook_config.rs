use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyOptExt};

/// Config for us building and integrating your mdbook
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct MdBookConfig {
    /// Path to the mdbook
    ///
    /// If not set we will attempt to auto-detect
    pub path: Option<String>,
    /// Whether to enable the custom oranda/axo theme
    pub theme: Option<bool>,
}

impl ApplyLayer for MdBookConfig {
    fn apply_layer(&mut self, layer: Self) {
        self.path.apply_opt(layer.path);
        self.theme.apply_opt(layer.theme)
    }
}
