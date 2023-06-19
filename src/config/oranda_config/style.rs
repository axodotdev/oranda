use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyValExt};
use crate::site::{markdown::SyntaxTheme, oranda_theme::OrandaTheme};

pub const LATEST_ORANDA_CSS: &str = "0.0.5";

/// Config related to styling your page
#[derive(Debug)]
pub struct StyleConfig {
    pub theme: OrandaTheme,
    pub syntax_theme: SyntaxTheme,
    pub additional_css: Vec<String>,
    pub oranda_css_version: String,
}

/// User-facing options related to styling your page
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct StyleOpts {
    pub theme: Option<OrandaTheme>,
    pub syntax_theme: Option<SyntaxTheme>,
    #[serde(default)]
    pub additional_css: Vec<String>,
    pub oranda_css_version: Option<String>,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            theme: OrandaTheme::Dark,
            syntax_theme: SyntaxTheme::MaterialTheme,
            additional_css: vec![],
            oranda_css_version: LATEST_ORANDA_CSS.to_string(),
        }
    }
}

impl ApplyLayer for StyleConfig {
    fn apply_layer(&mut self, layer: Self) {
        self.theme.apply_val(Some(layer.theme));
        self.syntax_theme.apply_val(Some(layer.syntax_theme));
        self.oranda_css_version.apply_val(Some(layer.oranda_css_version));
        self.additional_css.apply_val(Some(layer.additional_css));
    }
}
