use crate::config::ProjectConfig;
use crate::errors::*;
use serde::Deserialize;
use std::path::{Path, PathBuf};

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
