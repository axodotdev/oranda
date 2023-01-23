use std::fs;
use std::path::{Path, PathBuf};

#[cfg(test)]
use assert_fs::fixture::{FileWriteStr, PathChild};

use crate::config::{project::ProjectType, ProjectConfig};
use crate::errors::*;

static PACKAGE_JSON: &str = "./package.json";

#[derive(Debug, Eq, PartialEq)]
pub struct JavaScript {}
impl JavaScript {
    pub fn read(&self, project_root: &Option<PathBuf>) -> Result<ProjectConfig> {
        let package_json = fs::read_to_string(JavaScript::config(project_root))?;
        let data: ProjectConfig = serde_json::from_str(&package_json)?;
        Ok(data)
    }

    pub fn config(project_root: &Option<PathBuf>) -> PathBuf {
        if let Some(root) = project_root {
            Path::new(root).join(PACKAGE_JSON)
        } else {
            Path::new(PACKAGE_JSON).to_path_buf()
        }
    }
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
        Some(ProjectType::JavaScript(JavaScript {}))
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
