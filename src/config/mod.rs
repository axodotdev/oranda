pub mod artifacts;
mod oranda_config;
pub mod project;
pub mod theme;

use artifacts::Artifacts;
pub use oranda_config::{MdBookConfig, StyleConfig};
pub mod analytics;
use crate::errors::*;
use analytics::Analytics;
use camino::Utf8PathBuf;
use oranda_config::{OrandaConfig, Social};
use project::ProjectConfig;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Config {
    pub description: String,
    pub dist_dir: String,
    pub homepage: Option<String>,
    pub static_dir: String,
    pub name: String,
    pub no_header: bool,
    pub readme_path: String,
    pub repository: Option<String>,
    pub analytics: Option<Analytics>,
    pub additional_pages: Option<HashMap<String, String>>,
    pub social: Option<Social>,
    pub artifacts: Artifacts,
    pub version: Option<String>,
    pub logo: Option<String>,
    pub favicon: Option<String>,
    pub path_prefix: Option<String>,
    pub license: Option<String>,
    /// The config for using mdbook
    pub mdbook: Option<MdBookConfig>,
    pub styles: StyleConfig,
    pub changelog: bool,
    pub funding: bool,
}

impl Config {
    pub fn build(config_path: &Utf8PathBuf) -> Result<Config> {
        //Users can have multiple types of configuration or no configuration at all
        //
        //- Project configuration comes from a project manifest file. We currently
        //  support `Cargo.toml` and `package.json`, but could support any manifest
        //  that provided a `name`, `description`, `repository` and `homepage` field.
        //
        //- Custom configuration comes from a `oranda.config.json` file. If this
        //  file exists, it has precedence over project configuration, which means
        //  you could use this file to override fields in your project manifest.
        //  This file can contain all possible public configuration fields.
        let default = Config::default();
        let custom = OrandaConfig::load(config_path)?;
        let project = ProjectConfig::load(None)?;

        // if there is no oranda.config file present...
        if custom.is_none() {
            // but there is a project manifest file
            if let Some(project) = project {
                // return a merge of the default and project config
                return Ok(Config {
                    description: project.description,
                    homepage: project.homepage,
                    name: project.name,
                    repository: project.repository,
                    version: project.version,
                    license: project.license,
                    ..Default::default()
                });
            } else {
                // otherwise return the default
                return Ok(default);
            }
        }

        // if there is an oranda.config file
        if let Some(custom) = custom {
            // but there is not project manifest
            if project.is_none() {
                //return a merge of custom config and default config
                return Ok(Config {
                    description: custom.description.unwrap_or(default.description),
                    dist_dir: custom.dist_dir.unwrap_or(default.dist_dir),
                    static_dir: custom.static_dir.unwrap_or(default.static_dir),
                    homepage: Self::project_override(custom.homepage, None, default.homepage),
                    name: custom.name.unwrap_or(default.name),
                    no_header: custom.no_header.unwrap_or(default.no_header),
                    readme_path: custom.readme_path.unwrap_or(default.readme_path),
                    repository: Self::project_override(custom.repository, None, default.repository),
                    analytics: custom.analytics,
                    additional_pages: custom.additional_pages,
                    social: custom.social,
                    artifacts: custom.artifacts.unwrap_or(default.artifacts),
                    styles: custom.styles.unwrap_or(default.styles),
                    version: None,
                    license: None,
                    logo: custom.logo,
                    favicon: custom.favicon,
                    path_prefix: custom.path_prefix,
                    mdbook: None,
                    changelog: custom.changelog.unwrap_or(default.changelog),
                    funding: custom.funding.unwrap_or(default.funding),
                });
            // otherwise both oranda config and project manifest exists
            } else if let Some(project) = project {
                // so return a merge of custom > project > default
                return Ok(Config {
                    description: custom.description.unwrap_or(project.description),
                    dist_dir: custom.dist_dir.unwrap_or(default.dist_dir),
                    static_dir: custom.static_dir.unwrap_or(default.static_dir),
                    homepage: Self::project_override(
                        custom.homepage,
                        project.homepage,
                        default.homepage,
                    ),
                    name: custom.name.unwrap_or(project.name),
                    no_header: custom.no_header.unwrap_or(default.no_header),
                    readme_path: custom.readme_path.unwrap_or(default.readme_path),
                    repository: Self::project_override(
                        custom.repository,
                        project.repository,
                        default.repository,
                    ),
                    analytics: custom.analytics,
                    additional_pages: custom.additional_pages,
                    social: custom.social,
                    artifacts: custom.artifacts.unwrap_or(default.artifacts),
                    styles: custom.styles.unwrap_or(default.styles),
                    version: custom.version.or(project.version),
                    license: custom.license.or(project.license),
                    logo: custom.logo,
                    favicon: custom.favicon,
                    path_prefix: custom.path_prefix,
                    mdbook: custom.mdbook,
                    changelog: custom.changelog.unwrap_or(default.changelog),
                    funding: custom.funding.unwrap_or(default.funding),
                });
            }
        }

        Err(OrandaError::Other(String::from(
            "Your config is a bag of bees. Not today, Satan",
        )))
    }

    pub fn project_override(
        custom: Option<String>,
        project: Option<String>,
        default: Option<String>,
    ) -> Option<String> {
        if custom.is_some() {
            custom
        } else if project.is_some() {
            project
        } else {
            default
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            description: String::new(),
            dist_dir: String::from("public"),
            homepage: None,
            name: String::from("My Axo project"),
            no_header: false,
            readme_path: String::from("README.md"),
            repository: None,
            analytics: None,
            additional_pages: None,
            social: None,
            artifacts: Artifacts::default(),
            styles: StyleConfig::default(),
            version: None,
            license: None,
            logo: None,
            favicon: None,
            path_prefix: None,
            static_dir: String::from("static"),
            mdbook: None,
            changelog: false,
            funding: false,
        }
    }
}
