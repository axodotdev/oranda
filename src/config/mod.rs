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
}

impl Config {
    pub fn build(config_path: &Utf8PathBuf) -> Result<Config> {
        // Users can have multiple types of configuration or no configuration at all
        //
        // - Project configuration comes from a project manifest file. We currently
        //   support `Cargo.toml` and `package.json`, but could support any manifest
        //   that provided a `name`, `description`, `repository` and `homepage` field.
        //
        // - Custom configuration comes from a `oranda.config.json` file. If this
        //   file exists, it has precedence over project configuration, which means
        //   you could use this file to override fields in your project manifest.
        //   This file can contain all possible public configuration fields.
        //
        // We apply these in layers, with later layers winning over earlier ones.
        //
        // Note that several of these config merges do a seemingly-useless `if`
        // before applying a value. This is intentional to make the code more robust to refactors.
        //
        // If new stages are added or better defaults get introduced, we always
        // want to defer to those values if the layer we're currently applying doesn't have
        // an opinion on that value, which is what "None" in a config is really expressing.
        let mut cfg = Config::default();
        let custom = OrandaConfig::load(config_path)?;
        let project = ProjectConfig::load(None)?;

        cfg.apply_project_layer(project);
        cfg.apply_custom_layer(custom);

        Ok(cfg)
    }

    /// Apply the layer of config we computed from project files
    fn apply_project_layer(&mut self, project: Option<ProjectConfig>) {
        if let Some(project) = project {
            self.name = project.name;
            self.description = project.description;

            if let Some(val) = project.homepage {
                self.homepage = Some(val);
            }
            if let Some(val) = project.repository {
                self.repository = Some(val);
            }
            if let Some(val) = project.version {
                self.version = Some(val);
            }
            if let Some(val) = project.license {
                self.license = Some(val);
            }
            if let Some(val) = project.readme_path {
                self.readme_path = val.to_string();
            }
            if let Some(val) = project.cargo_dist {
                self.artifacts.cargo_dist = Some(val);
            }
        }
    }

    /// Apply the layer of config we computed from oranda.json
    fn apply_custom_layer(&mut self, custom: Option<OrandaConfig>) {
        // Apply the "custom" layer
        if let Some(custom) = custom {
            if let Some(val) = custom.description {
                self.description = val;
            }
            if let Some(val) = custom.dist_dir {
                self.dist_dir = val;
            }
            if let Some(val) = custom.static_dir {
                self.static_dir = val;
            }
            if let Some(val) = custom.homepage {
                self.homepage = Some(val);
            }
            if let Some(val) = custom.name {
                self.name = val;
            }
            if let Some(val) = custom.readme_path {
                self.readme_path = val;
            }
            if let Some(val) = custom.repository {
                self.repository = Some(val);
            }
            if let Some(val) = custom.analytics {
                self.analytics = Some(val);
            }
            if let Some(val) = custom.additional_pages {
                self.additional_pages = Some(val);
            }
            if let Some(val) = custom.social {
                self.social = Some(val);
            }
            if let Some(artifacts) = custom.artifacts {
                // This value gets merged in a more fine-grain matter
                // to allow earlier layers to set some values
                if let Some(val) = artifacts.cargo_dist {
                    self.artifacts.cargo_dist = Some(val);
                }
                if let Some(val) = artifacts.package_managers {
                    self.artifacts.package_managers = Some(val);
                }
            }
            if let Some(val) = custom.styles {
                self.styles = val;
            }
            if let Some(val) = custom.version {
                self.version = Some(val);
            }
            if let Some(val) = custom.logo {
                self.logo = Some(val);
            }
            if let Some(val) = custom.favicon {
                self.favicon = Some(val);
            }
            if let Some(val) = custom.path_prefix {
                self.path_prefix = Some(val);
            }
            if let Some(val) = custom.mdbook {
                self.mdbook = Some(val);
            }
            if let Some(val) = custom.changelog {
                self.changelog = val;
            }
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
        }
    }
}
