pub mod artifacts;
mod oranda;
pub mod project;
pub mod theme;
use self::artifacts::Artifacts;
pub mod analytics;
use self::analytics::Analytics;
use self::oranda::{OrandaConfig, Social};
use crate::errors::*;
use crate::site::markdown::SyntaxTheme;
use project::ProjectConfig;
use std::collections::HashMap;
use std::path::Path;

use theme::Theme;

#[derive(Debug)]

pub struct Config {
    pub description: String,
    pub dist_dir: String,
    pub homepage: Option<String>,
    pub static_dir: String,
    pub name: String,
    pub no_header: bool,
    pub readme_path: String,
    pub theme: Theme,
    pub additional_css: Vec<String>,
    pub repository: Option<String>,
    pub syntax_theme: SyntaxTheme,
    pub analytics: Option<Analytics>,
    pub additional_pages: Option<HashMap<String, String>>,
    pub social: Option<Social>,
    pub artifacts: Option<Artifacts>,
    pub version: Option<String>,
    pub logo: Option<String>,
    pub favicon: Option<String>,
    pub path_prefix: Option<String>,
    pub license: Option<String>,
    pub md_book: Option<String>,
    pub changelog: bool,
    pub funding: bool,
}

impl Config {
    pub fn build(config_path: &Path) -> Result<Config> {
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
                    theme: custom.theme.unwrap_or(default.theme),
                    additional_css: custom.additional_css.unwrap_or(default.additional_css),
                    repository: Self::project_override(custom.repository, None, default.repository),
                    syntax_theme: custom.syntax_theme.unwrap_or(default.syntax_theme),
                    analytics: custom.analytics,
                    additional_pages: custom.additional_pages,
                    social: custom.social,
                    artifacts: custom.artifacts,
                    version: None,
                    license: None,
                    logo: custom.logo,
                    favicon: custom.favicon,
                    path_prefix: custom.path_prefix,
                    md_book: custom.md_book,
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
                    theme: custom.theme.unwrap_or(default.theme),
                    additional_css: custom.additional_css.unwrap_or(default.additional_css),
                    repository: Self::project_override(
                        custom.repository,
                        project.repository,
                        default.repository,
                    ),
                    syntax_theme: custom.syntax_theme.unwrap_or(default.syntax_theme),
                    analytics: custom.analytics,
                    additional_pages: custom.additional_pages,
                    social: custom.social,
                    artifacts: custom.artifacts,
                    version: custom.version.or(project.version),
                    license: custom.license.or(project.license),
                    logo: custom.logo,
                    favicon: custom.favicon,
                    path_prefix: custom.path_prefix,
                    md_book: custom.md_book,
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
            theme: Theme::Dark,
            additional_css: vec![],
            repository: None,
            syntax_theme: SyntaxTheme::MaterialTheme,
            analytics: None,
            additional_pages: None,
            social: None,
            artifacts: None,
            version: None,
            license: None,
            logo: None,
            favicon: None,
            path_prefix: None,
            static_dir: String::from("static"),
            md_book: None,
            changelog: false,
            funding: false,
        }
    }
}
