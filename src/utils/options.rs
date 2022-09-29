use std::fs::{self};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use twelf::config;
pub struct Downloads {}

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum Theme {
    light,
    dark,
}

#[config]
#[derive(Debug, Eq, PartialEq)]
pub struct Options {
    // Your Readme.md name
    pub file: Option<String>,
    pub dist: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    // pub logo: String,
    // pub shareCard: String,
    pub no_header: Option<bool>,
    pub theme: Option<Theme>,
}

impl Default for Options {
    fn default() -> Self {
        fn get_nested_toml(value: &Value, key: String) -> String {
            let empty_default = Value::String("".to_string());

            value["package"]
                .get(key)
                .unwrap_or_else(|| &empty_default)
                .to_string()
        }

        let mut name = String::new();
        let mut description = String::new();
        let mut homepage = String::new();

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

pub fn create_parsed_options(options: Options) -> Options {
    let defaults = Options::default();
    Options {
        file: options.file.or(defaults.file),
        dist: options.dist.or(defaults.dist),
        no_header: options.no_header.or(defaults.no_header),
        description: options.description.or(defaults.description),
        name: options.name.or(defaults.name),
        theme: options.theme.or(defaults.theme),
        homepage: options.homepage.or(defaults.homepage),
    }
}
