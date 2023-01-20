use std::path::{Path, PathBuf};

use serde::Deserialize;

#[cfg(test)]
use crate::config::project::Type;
#[cfg(test)]
use crate::initialize_tokio_runtime;
#[cfg(test)]
use assert_fs::fixture::{FileWriteStr, PathChild};

use crate::config::ProjectConfig;
use crate::errors::*;

static CARGO_TOML: &str = "./Cargo.toml";

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: ProjectConfig,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Rust {}
impl Rust {
    pub fn read(&self, project_root: &Option<PathBuf>) -> Result<ProjectConfig> {
        let path = Rust::config(project_root);
        let cargo_toml_future = axoasset::load_string(path.to_str().unwrap());
        let cargo_toml = tokio::runtime::Handle::current().block_on(cargo_toml_future)?;
        let data: CargoToml = toml::from_str(&cargo_toml)?;
        Ok(data.package)
    }

    pub fn config(project_root: &Option<PathBuf>) -> PathBuf {
        if let Some(root) = project_root {
            Path::new(root).join(CARGO_TOML)
        } else {
            Path::new(CARGO_TOML).to_path_buf()
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
    let _guard = initialize_tokio_runtime().enter();
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
