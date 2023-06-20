use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyValExt};
use crate::site::{markdown::SyntaxTheme, oranda_theme::OrandaTheme};

const ORANDA_CSS_TAG: &str = "css-v0.0.7";

/// User facing options related to styling your page
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StyleOpts {
    pub theme: Option<OrandaTheme>,
    pub syntax_theme: Option<SyntaxTheme>,
    pub additional_css: Option<Vec<String>>,
    pub oranda_css_tag: Option<String>,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            theme: OrandaTheme::Light,
            syntax_theme: SyntaxTheme::MaterialTheme,
            additional_css: vec![],
            oranda_css_tag: ORANDA_CSS_TAG.to_string(),
        }
    }
}

/// Config related to styling your page
#[derive(Debug)]
pub struct StyleConfig {
    pub theme: OrandaTheme,
    pub syntax_theme: SyntaxTheme,
    pub additional_css: Vec<String>,
    pub oranda_css_tag: String,
}

impl ApplyLayer for StyleConfig {
    fn apply_layer(&mut self, layer: StyleOpts) {
        self.theme.apply_val(layer.theme);
        self.syntax_theme.apply_val(layer.syntax_theme);
        self.oranda_css_tag.apply_val(layer.oranda_css_tag);
        self.additional_css.apply_val(layer.additional_css);
    }
}
