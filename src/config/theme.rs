use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    Hacker,
    Cupcake,
}

pub fn css_class(theme: &Theme) -> &str {
    match theme {
        Theme::Dark => "dark",
        Theme::Hacker => "hacker",
        Theme::Cupcake => "cupcake",
        _ => "light",
    }
}
