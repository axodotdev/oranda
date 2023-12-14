use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::config::{ApplyLayer, ApplyOptExt};
use crate::site::{markdown::SyntaxTheme, oranda_theme::OrandaTheme};

use super::ApplyValExt;

pub const ORANDA_CSS_TAG: &str = "v0.6.0-prerelease.2";

/// Config related to styling your page (complete version)
#[derive(Debug, Clone)]
pub struct StyleConfig {
    pub theme: OrandaTheme,
    pub syntax_theme: SyntaxTheme,
    pub additional_css: Vec<String>,
    pub oranda_css_version: String,
    pub logo: Option<String>,
    pub favicon: Option<String>,
}
/// Settings for styling your page
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StyleLayer {
    /// The builtin oranda theme to use for all your pages
    ///
    /// If using oranda's mdbook integration this will also restyle your mdbook
    /// (assuming we made an equivalent mdbook theme).
    ///
    /// Default is "dark"
    pub theme: Option<OrandaTheme>,
    /// The builtin syntax highlighting theme to use for all your pages
    ///
    /// WARNING: this feature is currently non-functional, only the default works!
    ///
    /// Default is "MaterialTheme"
    syntax_theme: Option<SyntaxTheme>,
    /// A list of relative paths to extra css files to include in all your pages
    pub additional_css: Option<Vec<String>>,
    /// A way to force oranda to use a different archived version of its builtin css
    ///
    /// The value is the git-tag of the release on the oranda repo to fetch oranda.css from.
    ///
    /// Example: "css-v0.0.7"
    pub oranda_css_version: Option<String>,
    /// A relative path or URL to an image to use as the logo of your project
    pub logo: Option<String>,
    /// A relative path or URL to an image to use as the favicon of your site
    pub favicon: Option<String>,
}

impl Default for StyleConfig {
    fn default() -> Self {
        StyleConfig {
            theme: OrandaTheme::Dark,
            syntax_theme: SyntaxTheme::MaterialTheme,
            additional_css: vec![],
            oranda_css_version: ORANDA_CSS_TAG.to_owned(),
            logo: None,
            favicon: None,
        }
    }
}
impl ApplyLayer for StyleConfig {
    type Layer = StyleLayer;
    fn apply_layer(&mut self, layer: Self::Layer) {
        // This is intentionally written slightly cumbersome to make you update this
        let StyleLayer {
            theme,
            syntax_theme,
            additional_css,
            oranda_css_version,
            logo,
            favicon,
        } = layer;

        self.theme.apply_val(theme);
        self.syntax_theme.apply_val(syntax_theme);
        self.oranda_css_version.apply_val(oranda_css_version);
        // In the future this might want to be `extend`
        self.additional_css.apply_val(additional_css);
        self.logo.apply_opt(logo);
        self.favicon.apply_opt(favicon);
    }
}
