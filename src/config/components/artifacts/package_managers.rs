use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::config::{ApplyLayer, ApplyValExt};

/// Package managers to display (complete version)
#[derive(Debug, Clone)]
pub struct PackageManagersConfig {
    pub preferred: IndexMap<String, String>,
    pub additional: IndexMap<String, String>,
}
/// Package managers to display
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PackageManagersLayer {
    /// Packages to display in both the install widget and install page
    ///
    /// See docs for the parent "package_managers" field for details
    pub preferred: Option<IndexMap<String, String>>,
    /// Packages to display in just the install page
    ///
    /// See docs for the parent "package_managers" field for details
    pub additional: Option<IndexMap<String, String>>,
}

impl Default for PackageManagersConfig {
    fn default() -> Self {
        PackageManagersConfig {
            preferred: IndexMap::default(),
            additional: IndexMap::default(),
        }
    }
}
impl ApplyLayer for PackageManagersConfig {
    type Layer = PackageManagersLayer;
    fn apply_layer(&mut self, layer: Self::Layer) {
        // This is intentionally written slightly cumbersome to make you update this
        let PackageManagersLayer {
            preferred,
            additional,
        } = layer;
        // In the future these might want to be `extend`
        self.preferred.apply_val(preferred);
        self.additional.apply_val(additional);
    }
}

impl PackageManagersConfig {
    pub fn has(&self, key: &str) -> bool {
        self.preferred.contains_key(key) || self.additional.contains_key(key)
    }
    pub fn has_npm(&self) -> bool {
        self.has("npm") || self.has("npx")
    }
    pub fn is_empty(&self) -> bool {
        self.preferred.is_empty() && self.additional.is_empty()
    }
}
