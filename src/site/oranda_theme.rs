use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum OrandaTheme {
    Light,
    Dark,
    #[serde(alias = "axo_light")]
    AxoLight,
    #[serde(alias = "axo_dark")]
    AxoDark,
    Hacker,
    Cupcake,
}

impl OrandaTheme {
    pub fn css_class(theme: &OrandaTheme) -> &'static str {
        match theme {
            OrandaTheme::Dark => "dark",
            OrandaTheme::AxoLight => "axo",
            OrandaTheme::AxoDark => "dark axo",
            OrandaTheme::Hacker => "hacker",
            OrandaTheme::Cupcake => "cupcake",
            _ => "light",
        }
    }
}
