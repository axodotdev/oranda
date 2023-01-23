use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
}

pub fn css_class(theme: &Theme) -> &str {
    match theme {
        Theme::Dark => "dark",
        _ => "light",
    }
}
