use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyOptExt};
use crate::site::{markdown::SyntaxTheme, oranda_theme::OrandaTheme};

use super::ApplyValExt;

pub const ORANDA_CSS_TAG: &str = "css-v0.0.7";

/// Config related to styling your page (complete version)
#[derive(Debug)]
pub struct StyleConfig {
    pub theme: OrandaTheme,
    pub syntax_theme: SyntaxTheme,
    pub additional_css: Vec<String>,
    pub oranda_css_version: String,
    pub logo: Option<String>,
    pub favicon: Option<String>,
}
/// Config related to styling your page (partial version used by oranda.json)
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StyleLayer {
    pub theme: Option<OrandaTheme>,
    pub syntax_theme: Option<SyntaxTheme>,
    pub additional_css: Option<Vec<String>>,
    pub oranda_css_version: Option<String>,
    pub logo: Option<String>,
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
