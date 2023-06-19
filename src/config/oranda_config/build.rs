use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyOptExt, ApplyValExt};

/// Config related to styling your page
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BuildConfig {
    #[serde(default = "dist_dir_default")]
    pub dist_dir: String,
    #[serde(default = "static_dir_default")]
    pub static_dir: String,
    pub path_prefix: Option<String>,
}

fn dist_dir_default() -> String {
    "dist".to_string()
}

fn static_dir_default() -> String {
    "static".to_string()
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            dist_dir: dist_dir_default(),
            static_dir: static_dir_default(),
            path_prefix: None,
        }
    }
}

impl ApplyLayer for BuildConfig {
    fn apply_layer(&mut self, layer: Self) {
        self.dist_dir.apply_val(Some(layer.dist_dir));
        self.static_dir.apply_val(Some(layer.static_dir));
        self.path_prefix.apply_opt(layer.path_prefix);
    }
}

impl BuildConfig {
    pub fn new(
        dist_dir: Option<String>,
        static_dir: Option<String>,
        path_prefix: Option<String>,
    ) -> Self {
        Self {
            dist_dir: dist_dir.unwrap_or_default(),
            static_dir: static_dir.unwrap_or_default(),
            path_prefix,
        }
    }
}
