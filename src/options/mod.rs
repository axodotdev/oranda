mod project;
pub mod theme;

use theme::Theme;

#[derive(Debug)]
pub struct Options {
    pub description: String,
    pub dist_dir: String,
    pub homepage: Option<String>,
    pub name: String,
    pub no_header: bool,
    pub readme_path: String,
    pub theme: Theme,
}

impl Options {
    pub fn build() -> Options {
        let default = Options::default();
        if let Some(popts) = project::Options::load() {
            Options {
                description: popts.description,
                dist_dir: default.dist_dir,
                homepage: popts.homepage,
                name: popts.name,
                no_header: default.no_header,
                readme_path: default.readme_path,
                theme: default.theme,
            }
        } else {
            default
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            description: String::from("Queen triggerfish viperfish trench lightfish flying gurnard candlefish; Atlantic cod North American freshwater catfish four-eyed fish zebra lionfish worm eel."),
            dist_dir: String::from("public"),
            homepage: None,
            name: String::from("My Axo project"),
            no_header: false,
            readme_path: String::from("README.md"),
            theme: Theme::Light,
        }
    }
}
