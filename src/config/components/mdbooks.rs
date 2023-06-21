use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyOptExt, ApplyValExt};

/// Config for us building and integrating your mdbook
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MdBookConfig {
    /// Path to the mdbook
    ///
    /// If not set we will attempt to auto-detect
    pub path: Option<String>,
    /// Whether to enable the custom oranda/axo theme
    pub theme: bool,
}

/// Config for us building and integrating your mdbook
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct MdBookLayer {
    /// Path to the mdbook
    ///
    /// If not set we will attempt to auto-detect
    pub path: Option<String>,
    /// Whether to enable the custom oranda/axo theme
    pub theme: Option<bool>,
}

impl Default for MdBookConfig {
    fn default() -> Self {
        MdBookConfig {
            path: None,
            theme: true,
        }
    }
}
impl ApplyLayer for MdBookConfig {
    type Layer = MdBookLayer;
    fn apply_layer(&mut self, layer: Self::Layer) {
        // This is intentionally written slightly cumbersome to make you update this
        let MdBookLayer { path, theme } = layer;
        self.path.apply_opt(path);
        self.theme.apply_val(theme);
    }
}
