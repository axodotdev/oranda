use crate::config::ProjectConfig;
use crate::errors::*;
use std::path::{Path, PathBuf};

#[cfg(test)]
use assert_fs::fixture::{FileWriteStr, PathChild};

#[cfg(test)]
use crate::config::project::Type;

#[cfg(test)]
use crate::tests::TEST_RUNTIME;

static PACKAGE_JSON: &str = "./package.json";

#[derive(Debug, Eq, PartialEq)]
pub struct JavaScript {}
impl JavaScript {
    pub fn read(&self, project_root: &Option<PathBuf>) -> Result<ProjectConfig> {
        let path = JavaScript::config(project_root);
        let package_json_future = axoasset::load_string(path.to_str().unwrap());
        let package_json = tokio::runtime::Handle::current().block_on(package_json_future)?;
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
        Some(Type::JavaScript(JavaScript {}))
    );
    tempdir
        .close()
        .expect("could not successfully delete temporary directory");
}

#[test]
fn it_loads_a_js_project_config() {
    let _guard = TEST_RUNTIME.enter();
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
