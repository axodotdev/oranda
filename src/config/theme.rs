use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    #[serde(alias = "axo_light")]
    AxoLight,
    #[serde(alias = "axo_dark")]
    AxoDark,
    Hacker,
    Cupcake,
}

pub fn css_class(theme: &Theme) -> &'static str {
    match theme {
        Theme::Dark => "dark",
        Theme::AxoLight => "axo",
        Theme::AxoDark => "dark axo",
        Theme::Hacker => "hacker",
        Theme::Cupcake => "cupcake",
        _ => "light",
    }
}
