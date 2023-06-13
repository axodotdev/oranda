use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    Hacker,
    Cupcake,
}

pub fn css_class(theme: &Theme) -> &'static str {
    match theme {
        Theme::Dark => "dark",
        Theme::Hacker => "hacker",
        Theme::Cupcake => "cupcake",
        _ => "light",
    }
}
