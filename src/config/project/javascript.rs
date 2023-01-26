use crate::config::ProjectConfig;
use crate::errors::*;
use std::path::{Path, PathBuf};

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
