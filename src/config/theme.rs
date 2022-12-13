use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    Axo,
}

pub fn css_class(theme: &Theme) -> String {
    match theme {
        Theme::Dark => String::from("dark"),
        Theme::Axo => String::from("axo"),
        _ => String::from(""),
    }
}
