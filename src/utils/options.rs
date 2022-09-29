use twelf::config;
pub struct Downloads {}

// enum Theme {
//     Light,
//     Dark,
// }

#[config]
pub struct Options {
    // Your Readme.md name
    pub file: Option<String>,
    pub dist: Option<String>,
    // pub name: String,
    // pub logo: String,
    // pub shareCard: String,
    // pub homepage: String,
    // pub noHeader: bool,
    // pub theme: Theme,
    // pub description: String,
}

pub struct OptionsFilled {
    // Your Readme.md name
    pub file: String,
    pub dist: String,
    // pub name: String,
    // pub logo: String,
    // pub shareCard: String,
    // pub homepage: String,
    // pub noHeader: bool,
    // pub theme: Theme,
    // pub description: String,
}
