use axoasset::Asset;
use serde::Deserialize;

use crate::config::ProjectConfig;
use crate::errors::*;
use std::path::{Path, PathBuf};

static PACKAGE_JSON: &str = "./package.json";

/// A package.json file
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct PackageJson {
    /// Name of the package
    pub name: String,
    /// Version of the package
    pub version: Option<String>,
    /// Description of the package
    pub description: String,
    /// Link to the homepage
    pub homepage: Option<String>,
    /// Link to the repository
    pub repository: Option<Repository>,
    /// License of the package
    pub license: Option<String>,
}

/// A link to a repository
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Repository {
    /// Shorthand syntax
    Short(String),
    /// Long form
    Long(LongRepository),
}

/// A link to a repository
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct LongRepository {
    /// The type of link it is
    pub r#type: String,
    /// The url
    pub url: String,
    /// The subdirectory to find the project at
    pub directory: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct JavaScript {}
impl JavaScript {
    pub fn read(&self, project_root: &Option<PathBuf>) -> Result<ProjectConfig> {
        let path = JavaScript::config(project_root);
        let package_json_future = Asset::load_string(path.to_str().unwrap());
        let package_json = tokio::runtime::Handle::current().block_on(package_json_future)?;
        let data: PackageJson = serde_json::from_str(&package_json)?;

        let repository = data.repository.map(|repo| match repo {
            // TODO: process this into a proper URL?
            //
            // It can be things like:
            //
            // * "npm/npm"
            // * "github:user/repo"
            // * "gist:11081aaa281"
            // * "bitbucket:user/repo"
            // * "gitlab:user/repo"
            //
            // Using the same syntax as https://docs.npmjs.com/cli/v7/commands/npm-install
            Repository::Short(repo) => repo,
            Repository::Long(repo) => repo.url,
        });

        Ok(ProjectConfig {
            name: data.name,
            description: data.description,
            homepage: data.homepage,
            repository,
            version: data.version,
            license: data.license,
        })
    }

    pub fn config(project_root: &Option<PathBuf>) -> PathBuf {
        if let Some(root) = project_root {
            Path::new(root).join(PACKAGE_JSON)
        } else {
            Path::new(PACKAGE_JSON).to_path_buf()
        }
    }
}
