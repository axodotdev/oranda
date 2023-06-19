use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyOptExt, ApplyValExt};

/// Config related to styling your page
#[derive(Debug)]
pub struct BuildConfig {
    pub dist_dir: String,
    pub static_dir: String,
    pub path_prefix: Option<String>,
}

/// User facing options related to styling your page
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BuildOpts {
    pub dist_dir: Option<String>,
    pub static_dir: Option<String>,
    pub path_prefix: Option<String>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            dist_dir: "dist".to_string(),
            static_dir: "static".to_string(),
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
