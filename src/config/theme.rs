use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    Hacker,
}

pub fn css_class(theme: &Theme) -> &str {
    match theme {
        Theme::Dark => "dark",
        Theme::Hacker => "hacker",
        _ => "light",
    }
}
