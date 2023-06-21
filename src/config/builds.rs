use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::Deserialize;

use super::{ApplyLayer, ApplyOptExt, ApplyValExt};

#[derive(Debug)]
/// Information about how the pages should be built
pub struct BuildConfig {
    /// Relative path to the dir where build output should be placed
    pub dist_dir: String,
    /// Relative path to a dir full of extra static content
    pub static_dir: String,
    /// A path fragment to prepend before URLs
    ///
    /// This allows things like hosting a static site at `axodotdev.github.io/my_project/`
    pub path_prefix: Option<String>,
    /// Additional pages that should be included in the top level nav.
    ///
    /// This is a map from page-label to relative paths to pages.
    ///
    /// We use IndexMap to respect the order the user provided.
    pub additional_pages: IndexMap<String, String>,
}
#[derive(Debug, Deserialize, JsonSchema)]
/// Information about how the pages should be built
pub struct BuildLayer {
    /// Relative path to the dir where build output should be placed
    pub dist_dir: Option<String>,
    /// Relative path to a dir full of extra static content
    pub static_dir: Option<String>,
    /// A path fragment to prepend before URLs
    ///
    /// This allows things like hosting a static site at `axodotdev.github.io/my_project/`
    pub path_prefix: Option<String>,
    /// Additional pages that should be included in the top level nav.
    ///
    /// This is a map from page-label to relative paths to pages.
    ///
    /// We use IndexMap to respect the order the user provided.
    pub additional_pages: Option<IndexMap<String, String>>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        BuildConfig {
            dist_dir: "public".to_owned(),
            static_dir: "static".to_owned(),
            path_prefix: None,
            additional_pages: Default::default(),
        }
    }
}
impl ApplyLayer for BuildConfig {
    type Layer = BuildLayer;
    fn apply_layer(&mut self, layer: Self::Layer) {
        // This is intentionally written slightly cumbersome to make you update this
        let BuildLayer {
            dist_dir,
            static_dir,
            path_prefix,
            additional_pages,
        } = layer;
        self.dist_dir.apply_val(dist_dir);
        self.static_dir.apply_val(static_dir);
        self.path_prefix.apply_opt(path_prefix);
        // In the future this might want to be `extend`
        self.additional_pages.apply_val(additional_pages);
    }
}
