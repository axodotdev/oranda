mod oranda;
mod project;
pub mod theme;
use self::oranda::OrandaConfig;
use crate::errors::*;
use crate::site::markdown::syntax_highlight::syntax_themes::SyntaxThemes;
use project::ProjectConfig;

use theme::Theme;

#[derive(Debug)]
pub struct Config {
    pub description: String,
    pub dist_dir: String,
    pub homepage: Option<String>,
    pub name: String,
    pub no_header: bool,
    pub readme_path: String,
    pub theme: Theme,
    pub remote_styles: Vec<String>,
    pub additional_css: String,
    pub repository: Option<String>,
    pub syntax_theme: SyntaxThemes,
    pub additional_pages: Option<Vec<String>>,
}

impl Config {
    pub fn build() -> Result<Config> {
        //Users can have multiple types of configuration or no configuration at all
        //
        //- Project configuration comes from a project manifest file. We currently
        //  support `Cargo.toml` and `package.json`, but could support any manifest
        //  that provided a `name`, `description`, and `homepage` field.
        //
        //- Custom configuration comes from a `oranda.config.json` file. If this
        //  file exists, it has precedence over project configuration, which means
        //  you could use this file to override fields in your project manifest.
        //  This file can contain all possible public configuration fields.
        let default = Config::default();
        let custom = OrandaConfig::load()?;
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
                    homepage: Self::homepage(custom.homepage, None, default.homepage),
                    name: custom.name.unwrap_or(default.name),
                    no_header: custom.no_header.unwrap_or(default.no_header),
                    readme_path: custom.readme_path.unwrap_or(default.readme_path),
                    theme: custom.theme.unwrap_or(default.theme),
                    remote_styles: custom.remote_styles.unwrap_or(default.remote_styles),
                    additional_css: custom.additional_css.unwrap_or(default.additional_css),
                    repository: custom.repository,
                    syntax_theme: custom.syntax_theme.unwrap_or(default.syntax_theme),
                    additional_pages: custom.additional_pages,
                });
            // otherwise both oranda config and project manifest exists
            } else if let Some(project) = project {
                // so return a merge of custom > project > default
                return Ok(Config {
                    description: custom.description.unwrap_or(project.description),
                    dist_dir: custom.dist_dir.unwrap_or(default.dist_dir),
                    homepage: Self::homepage(custom.homepage, project.homepage, default.homepage),
                    name: custom.name.unwrap_or(project.name),
                    no_header: custom.no_header.unwrap_or(default.no_header),
                    readme_path: custom.readme_path.unwrap_or(default.readme_path),
                    theme: custom.theme.unwrap_or(default.theme),
                    remote_styles: custom.remote_styles.unwrap_or(default.remote_styles),
                    additional_css: custom.additional_css.unwrap_or(default.additional_css),
                    repository: custom.repository,
                    syntax_theme: custom.syntax_theme.unwrap_or(default.syntax_theme),
                    additional_pages: custom.additional_pages,
                });
            }
        }
        Err(OrandaError::Other(String::from(
            "Your config is a bag of bees. Not today, Satan",
        )))
    }

    pub fn homepage(
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
            description: String::from(""),
            dist_dir: String::from("public"),
            homepage: None,
            name: String::from("My Axo project"),
            no_header: false,
            readme_path: String::from("README.md"),
            theme: Theme::Light,
            remote_styles: vec![],
            additional_css: String::from(""),
            repository: None,
            syntax_theme: SyntaxThemes::MaterialTheme,
            additional_pages: None,
        }
    }
}
