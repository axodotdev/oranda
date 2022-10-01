#[derive(Debug)]
pub enum Theme {
    Light,
    Dark,
}

pub fn css_class(theme: &Theme) -> String {
    match theme {
        Theme::Dark => String::from("dark"),
        _ => String::from(""),
    }
}
