use std::fs::{self};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;
pub struct Downloads {}

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum Theme {
    light,
    dark,
}

enum ProjectConfig {
    CargoToml,
    PackageJson
}

#[derive(Debug, Eq, PartialEq)]
pub struct Options {
    // Your Readme.md name
    pub file: String,
    pub dist: String,
    pub name: String,
    pub description: String,
    pub homepage: String,
    // pub logo: String,
    // pub shareCard: String,
    pub no_header: bool,
    pub theme: Theme,
    pub project_config: <Option<ProjectConfig>>,
}

impl Options {
    pub fn build() -> Options {}
    pub fn parse(&self) -> Self {
        let defaults = Options::default();
        Options {
            file: self.file.or(defaults.file),
            dist: self.dist.or(defaults.dist),
            no_header: self.no_header.or(defaults.no_header),
            description: self.description.or(defaults.description),
            name: self.name.or(defaults.name),
            theme: self.theme.or(defaults.theme),
            homepage: self.homepage.or(defaults.homepage),
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        fn get_nested_toml(value: &Value, key: String) -> String {
            let empty_default = Value::String("".to_string());

            value["package"]
                .get(key)
                .unwrap_or(&empty_default)
                .to_string()
        }

        let mut name = String::new();
        let mut description = String::new();
        let mut homepage = String::new();
        let oranda_config_file = ".oranda.config.json";
        let cargo_file = "Cargo.toml";
        let package_json_file = "package.json";
        if Path::new(cargo_file).exists() {
            let file = fs::read_to_string(cargo_file).unwrap();

            let value: Value = toml::from_str(&file).unwrap();

            name = get_nested_toml(&value, "name".to_string());
            description = get_nested_toml(&value, "description".to_string());
            homepage = get_nested_toml(&value, "homepage".to_string());
        };

        if Path::new(package_json_file).exists() {
            let file = fs::read_to_string(package_json_file).unwrap();
            let value: serde_json::Value = serde_json::from_str(&file).unwrap();
            name = value["name"].to_string();
            description = value["description"].to_string();
            homepage = value["homepage"].to_string();
        };

        Options {
            file: Some("Readme".to_string()),
            dist: Some("public".to_string()),
            no_header: Some(false),
            name: Some(name),
            description: Some(description),
            homepage: Some(homepage),
            theme: Some(Theme::light),
        }
    }
}
