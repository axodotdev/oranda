use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{ApplyLayer, ApplyOptExt, ApplyValExt};

/// Information about the project (complete version)
#[derive(Debug, Clone)]
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

/// Info about the project/application you're making a site for
///
/// All of these values should automatically be sourced from your Cargo.toml or package.json
/// whenever possible. You should only need to set these if you want to override the value.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectLayer {
    /// Name of the project
    ///
    /// This is used for the top-level heading on your site, as well as the title and footer.
    ///
    /// This is automatically sourced from your Cargo.toml or package.json.
    pub name: Option<String>,
    /// Current version of the project
    ///
    /// This is used as a last-resort fallback when referring to the current release
    /// of your project. If using richer integrations like our support for GitHub Releases,
    /// we'll prefer that source and complete ignore this field.
    ///
    /// This is automatically sourced from your Cargo.toml or package.json.
    pub version: Option<String>,
    /// Brief description of the project
    ///
    /// This is used as metadata for things like preview links to your site.
    ///
    /// This is automatically sourced from your Cargo.toml or package.json.
    pub description: Option<String>,
    /// URL to the homepage of the project
    ///
    /// This is used as metadata for things like preview links to your site.
    ///
    /// This is automatically sourced from your Cargo.toml or package.json.
    pub homepage: Option<String>,
    /// URL to the repository of the project
    ///
    /// This is used for a lot of things and super important for enabling richer
    /// integrations. If this is of the form `https://github.com/$USER/$PROJECT/` we can
    /// enable more advanced GitHub support like reading your GitHub Releases to generate
    /// pages for every release and doing platform-autodetected installation options.
    ///
    /// This is automatically sourced from your Cargo.toml or package.json, although
    /// we might not handle the more complicated formats that package.json supports.
    pub repository: Option<String>,
    /// Relative path to the README of this project
    ///
    /// This will become the body of your site's front page.
    ///
    /// By default we will just check for a README.md in the current working directory.
    ///
    /// This is automatically sourced from your Cargo.toml. To the best of our
    /// knowledge there is no equivalent field in package.json.
    pub readme_path: Option<String>,
    /// License of the project
    ///
    /// This is displayed in the footer of your pages.
    ///
    /// This is automatically sourced from your Cargo.toml or package.json.
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
