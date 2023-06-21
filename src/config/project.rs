use schemars::JsonSchema;
use serde::Deserialize;

use super::{ApplyLayer, ApplyOptExt, ApplyValExt};

/// Information about the project (complete version)
#[derive(Debug)]
pub struct ProjectConfig {
    /// Name of the project
    pub name: String,
    /// Current version of the project(?)
    pub version: Option<String>,
    /// Brief description of the project
    pub description: Option<String>,
    /// URL to the homepage of the project
    pub homepage: Option<String>,
    /// URL to the repository of the project
    ///
    /// If this is of the form `https://github.com/$USER/$PROJECT/` we can
    /// enable more advanced Github support
    pub repository: Option<String>,
    /// Relative path to the README of this project
    ///
    /// This is non-Optional because the README is the core thing we always require
    pub readme_path: String,
    /// License of the project (probably SPDX format)
    pub license: Option<String>,
}

/// Information about the project (partial version used by oranda.json)
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ProjectLayer {
    /// Name of the project
    pub name: Option<String>,
    /// Current version of the project(?)
    pub version: Option<String>,
    /// Brief description of the project
    pub description: Option<String>,
    /// URL to the homepage of the project
    pub homepage: Option<String>,
    /// URL to the repository of the project
    ///
    /// If this is of the form `https://github.com/$USER/$PROJECT/` we can
    /// enable more advanced Github support
    pub repository: Option<String>,
    /// Relative path to the README of this project
    pub readme_path: Option<String>,
    /// License of the project (probably SPDX format)
    pub license: Option<String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        ProjectConfig {
            name: "My Oranda Project".to_owned(),
            version: None,
            description: None,
            homepage: None,
            repository: None,
            readme_path: "README.md".to_owned(),
            license: None,
        }
    }
}

impl ApplyLayer for ProjectConfig {
    type Layer = ProjectLayer;
    fn apply_layer(&mut self, layer: Self::Layer) {
        // This is intentionally written slightly cumbersome to make you update this
        let ProjectLayer {
            name,
            version,
            description,
            homepage,
            repository,
            readme_path,
            license,
        } = layer;

        // Always overwrite
        self.name.apply_val(name);
        self.version.apply_opt(version);
        self.description.apply_opt(description);
        self.homepage.apply_opt(homepage);
        self.repository.apply_opt(repository);
        self.readme_path.apply_val(readme_path);
        self.license.apply_opt(license);
    }
}
