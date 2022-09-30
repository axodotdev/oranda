use std::fs;
use std::path::Path;

use serde::Deserialize;

static CARGO_TOML: &'static str = "Cargo.toml";
static PACKAGE_JSON: &'static str = "package.json";

#[derive(Debug, Deserialize)]
pub struct Options {
    pub name: String,
    pub description: String,
    pub homepage: Option<String>,
}

impl Options {
    pub fn load() -> Option<Options> {
        if let Some(ptype) = Options::detect() {
            match ptype {
                Type::Rust(project) => Some(project.read()),
                Type::JavaScript(project) => Some(project.read()),
            }
        } else {
            None
        }
    }
    
    fn detect() -> Option<Type> {
        if Rust::config().exists() {
            Some(Type::Rust(Rust {}))
        } else if JavaScript::config().exists() {
            Some(Type::JavaScript(JavaScript {}))
        } else {
            None
        }
    }
}

enum Type {
    Rust(Rust),
    JavaScript(JavaScript)
}

struct Rust {}
impl Rust {
    fn read(&self) -> Options {
        let cargo_toml = fs::read_to_string(CARGO_TOML).unwrap();
        toml::from_str(&cargo_toml).unwrap()
    }

    fn config() -> &'static Path {
        Path::new(CARGO_TOML)
    }
}

struct JavaScript {}
impl JavaScript {
    fn read(&self) -> Options {
        let package_json = fs::read_to_string(PACKAGE_JSON).unwrap();
        serde_json::from_str(&package_json).unwrap()
    }

    fn config() -> &'static Path {
        Path::new(PACKAGE_JSON)
    }
}

