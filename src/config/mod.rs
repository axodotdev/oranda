mod project;
pub mod theme;

use crate::errors::*;
use project::ProjectConfig;
use theme::Theme;

#[derive(Debug)]
pub struct Config {
    pub description: String,
    pub dist_dir: String,
    pub homepage: Option<String>,
    pub name: String,
    pub no_header: bool,
    pub readme_path: String,
    pub theme: Theme,
}

impl Config {
    pub fn build() -> Result<Config> {
        let default = Config::default();
        if let Ok(Some(popts)) = ProjectConfig::load() {
            Ok(Config {
                description: popts.description,
                dist_dir: default.dist_dir,
                homepage: popts.homepage,
                name: popts.name,
                no_header: default.no_header,
                readme_path: default.readme_path,
                theme: default.theme,
            })
        } else {
            Ok(default)
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
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
