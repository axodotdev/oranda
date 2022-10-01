use std::fs;
use std::path::Path;

use crate::OrandaError;
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
    pub fn load() -> Result<Option<Options>, OrandaError> {
        if let Some(ptype) = Options::detect() {
            match ptype {
                Type::JavaScript(project) => Ok(Some(project.read()?)),
                Type::Rust(project) => Ok(Some(project.read()?)),
            }
        } else {
            Ok(None)
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
    JavaScript(JavaScript),
}

struct Rust {}
impl Rust {
    fn read(&self) -> Result<Options, OrandaError> {
        let cargo_toml = fs::read_to_string(CARGO_TOML)?;
        let data: Options = toml::from_str(&cargo_toml)?;
        Ok(data)
    }

    fn config() -> &'static Path {
        Path::new(CARGO_TOML)
    }
}

struct JavaScript {}
impl JavaScript {
    fn read(&self) -> Result<Options, OrandaError> {
        let package_json = fs::read_to_string(PACKAGE_JSON)?;
        let data: Options = serde_json::from_str(&package_json)?;
        Ok(data)
    }

    fn config() -> &'static Path {
        Path::new(PACKAGE_JSON)
    }
}
