use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyOptExt, ApplyValExt};
use crate::site::{markdown::SyntaxTheme, oranda_theme::OrandaTheme};

/// Config related to styling your page
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct StyleConfig {
    pub theme: Option<OrandaTheme>,
    pub syntax_theme: Option<SyntaxTheme>,
    #[serde(default)]
    pub additional_css: Vec<String>,
    pub oranda_css_version: Option<String>,
}

impl ApplyLayer for StyleConfig {
    fn apply_layer(&mut self, layer: Self) {
        self.theme.apply_opt(layer.theme);
        self.syntax_theme.apply_opt(layer.syntax_theme);
        self.oranda_css_version.apply_opt(layer.oranda_css_version);
        self.additional_css.apply_val(Some(layer.additional_css));
    }
}
impl StyleConfig {
    /// Get the theme
    pub fn theme(&self) -> OrandaTheme {
        self.theme.unwrap_or(OrandaTheme::Dark)
    }
    /// Get the syntax_theme
    pub fn syntax_theme(&self) -> SyntaxTheme {
        self.syntax_theme.unwrap_or(SyntaxTheme::MaterialTheme)
    }
}
