use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    #[serde(rename = "axo-dark")]
    AxoDark,
    #[serde(rename = "axo-light")]
    AxoLight,
}

pub fn css_class(theme: &Theme) -> &str {
    match theme {
        Theme::Dark => "dark",
        Theme::AxoDark => "axo dark",
        Theme::AxoLight => "axo light",
        _ => "light",
    }
}
