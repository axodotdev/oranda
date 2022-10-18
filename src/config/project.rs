use std::fs;
use std::path::{Path, PathBuf};

#[cfg(test)]
use assert_fs::fixture::{FileWriteStr, PathChild};

use serde::Deserialize;

use crate::errors::*;

static CARGO_TOML: &str = "Cargo.toml";
static PACKAGE_JSON: &str = "package.json";

#[derive(Debug, Deserialize, PartialEq)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
    pub homepage: Option<String>,
}

impl ProjectConfig {
    pub fn load(project_root: Option<PathBuf>) -> Result<Option<ProjectConfig>> {
        if let Some(ptype) = ProjectConfig::detect(&project_root) {
            match ptype {
                Type::JavaScript(project) => Ok(Some(project.read(&project_root)?)),
                Type::Rust(project) => Ok(Some(project.read(&project_root)?)),
            }
        } else {
            Ok(None)
        }
    }

    fn detect(project_root: &Option<PathBuf>) -> Option<Type> {
        if Rust::config(&project_root).exists() {
            Some(Type::Rust(Rust {}))
        } else if JavaScript::config(&project_root).exists() {
            Some(Type::JavaScript(JavaScript {}))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
enum Type {
    Rust(Rust),
    JavaScript(JavaScript),
}

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: ProjectConfig,
}

#[derive(Debug, PartialEq)]
struct Rust {}
impl Rust {
    fn read(&self, project_root: &Option<PathBuf>) -> Result<ProjectConfig> {
        let cargo_toml = fs::read_to_string(Rust::config(&project_root))?;
        let data: CargoToml = toml::from_str(&cargo_toml)?;
        Ok(data.package)
    }

    fn config(project_root: &Option<PathBuf>) -> PathBuf {
        if let Some(root) = project_root {
            Path::new(root).join(CARGO_TOML)
        } else {
            Path::new(CARGO_TOML).to_path_buf()
        }
    }
}

#[derive(Debug, PartialEq)]
struct JavaScript {}
impl JavaScript {
    fn read(&self, project_root: &Option<PathBuf>) -> Result<ProjectConfig> {
        let package_json = fs::read_to_string(JavaScript::config(&project_root))?;
        let data: ProjectConfig = serde_json::from_str(&package_json)?;
        Ok(data)
    }

    fn config(project_root: &Option<PathBuf>) -> PathBuf {
        if let Some(root) = project_root {
            Path::new(root).join(PACKAGE_JSON)
        } else {
            Path::new(PACKAGE_JSON).to_path_buf()
        }
    }
}

#[test]
fn it_detects_a_rust_project() {
    let tempdir = assert_fs::TempDir::new().expect("failed creating tempdir");
    let cargo_toml = tempdir.child("Cargo.toml");
    cargo_toml
        .write_str(
            r#"
[package]
name = "axo"
description = ">o_o<"
    "#,
        )
        .expect("failed to write package_json");

    assert_eq!(
        ProjectConfig::detect(&Some(tempdir.path().to_path_buf())),
        Some(Type::Rust(Rust {}))
    );
    tempdir
        .close()
        .expect("could not successfully delete temporary directory");
}

#[test]
fn it_loads_a_rust_project_config() {
    let tempdir = assert_fs::TempDir::new().expect("failed creating tempdir");
    let cargo_toml = tempdir.child("Cargo.toml");
    cargo_toml
        .write_str(
            r#"
[package]
name = "axo"
description = ">o_o<"
    "#,
        )
        .expect("failed to write package_json");
    let config = ProjectConfig::load(Some(tempdir.path().to_path_buf()))
        .expect("failed to load Cargo.toml")
        .unwrap();

    assert_eq!(config.name, "axo");
    tempdir
        .close()
        .expect("could not successfully delete temporary directory");
}

#[test]
fn it_detects_a_js_project() {
    let tempdir = assert_fs::TempDir::new().expect("failed creating tempdir");
    let package_json = tempdir.child("package.json");
    package_json
        .write_str(
            r#"
{
    "name": "axo",
    "description": ">o_o<"
}
    "#,
        )
        .expect("failed to write package_json");

    assert_eq!(
        ProjectConfig::detect(&Some(tempdir.path().to_path_buf())),
        Some(Type::JavaScript(JavaScript {}))
    );
    tempdir
        .close()
        .expect("could not successfully delete temporary directory");
}

#[test]
fn it_loads_a_js_project_config() {
    let tempdir = assert_fs::TempDir::new().expect("failed creating tempdir");
    let package_json = tempdir.child("package.json");
    package_json
        .write_str(
            r#"
{
    "name": "axo",
    "description": ">o_o<"
}
    "#,
        )
        .expect("failed to write package_json");

    let config = ProjectConfig::load(Some(tempdir.path().to_path_buf()))
        .expect("failed to load package.json")
        .unwrap();

    assert_eq!(config.name, "axo");
    assert_eq!(config.description, ">o_o<");
    assert_eq!(config.homepage, None);
    tempdir
        .close()
        .expect("could not successfully delete temporary directory");
}
