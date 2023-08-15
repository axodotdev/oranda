use crate::config::{Config, SocialConfig};
use crate::errors::*;
use crate::site::oranda_theme::OrandaTheme;
use serde::Serialize;

pub mod css;
mod header;
pub mod javascript;
use crate::site::layout::header::get_logo;
use crate::site::{link, page};
use javascript::analytics::Analytics;

#[derive(Serialize, Debug, Default)]
pub struct LayoutContext {
    /// Result of [`OrandaTheme::as_css_classes`][]
    theme: &'static str,
    project_name: String,
    homepage: Option<String>,
    repository: Option<String>,
    favicon_url: Option<String>,
    description: Option<String>,
    oranda_css_path: String,
    has_additional_css: bool,
    logo: Option<String>,
    license: Option<String>,
    additional_pages: Option<Vec<AdditionalPageContext>>,
    artifacts_link: Option<String>,
    mdbook_link: Option<String>,
    funding_link: Option<String>,
    changelog_link: Option<String>,
    has_nav: bool,
    home_link: String,
    path_prefix: Option<String>,
    analytics: Analytics,
    social: SocialConfig,
}

#[derive(Serialize, Debug, Default)]
pub struct AdditionalPageContext {
    path: String,
    name: String,
}

impl LayoutContext {
    pub fn new(config: &Config) -> Result<Self> {
        let css_path =
            css::get_css_link(&config.build.path_prefix, &config.styles.oranda_css_version)?;
        let additional_pages = if config.build.additional_pages.is_empty() {
            None
        } else {
            let mut ret = Vec::new();
            for (name, path) in config.build.additional_pages.iter() {
                if page::source::is_markdown(path) {
                    let file_path = page::source::get_filename_with_dir(path)?;
                    if let Some(path) = file_path {
                        let href = link::generate(&config.build.path_prefix, &format!("{}/", path));
                        ret.push(AdditionalPageContext {
                            name: name.clone(),
                            path: href,
                        });
                    }
                }
            }
            Some(ret)
        };

        let favicon_url = config
            .styles
            .favicon
            .clone()
            .map(|_| link::generate(&config.build.path_prefix, "favicon.ico"));
        let logo = if let Some(logo) = config.styles.logo.as_deref() {
            let path = get_logo(logo, config)?;
            Some(path)
        } else {
            None
        };
        let artifacts_link = if config.components.artifacts_enabled() {
            let link = link::generate(&config.build.path_prefix, "artifacts/");
            Some(link)
        } else {
            None
        };
        let mdbook_link = &config
            .components
            .mdbook
            .as_ref()
            .map(|_| link::generate(&config.build.path_prefix, "book/"));
        let funding_link = &config
            .components
            .funding
            .as_ref()
            .map(|_| link::generate(&config.build.path_prefix, "funding/"));
        let changelog_link = &config
            .components
            .changelog
            .as_ref()
            .map(|_| link::generate(&config.build.path_prefix, "changelog/"));
        let has_nav = additional_pages.is_some()
            || artifacts_link.is_some()
            || mdbook_link.is_some()
            || funding_link.is_some()
            || changelog_link.is_some();
        let home_link = if let Some(path_prefix) = config.build.path_prefix.as_ref() {
            format!("/{}/", path_prefix)
        } else {
            "/".to_string()
        };
        let analytics = Analytics::new(&config.marketing.analytics);

        Ok(Self {
            theme: config.styles.theme.as_css_classes(),
            project_name: config.project.name.clone(),
            homepage: config.project.homepage.clone(),
            repository: config.project.repository.clone(),
            favicon_url,
            description: config.project.description.clone(),
            logo,
            license: config.project.license.clone(),
            oranda_css_path: css_path,
            has_additional_css: !config.styles.additional_css.is_empty(),
            additional_pages,
            artifacts_link,
            mdbook_link: mdbook_link.clone(),
            funding_link: funding_link.clone(),
            changelog_link: changelog_link.clone(),
            has_nav,
            home_link,
            path_prefix: config.build.path_prefix.clone(),
            analytics,
            social: config.marketing.social.clone(),
        })
    }

    /// Generates a new layout context to use for the workspace index page.
    pub fn new_for_workspace_index(workspace_config: &Config) -> Result<Self> {
        let css_path = css::get_css_link(
            &workspace_config.build.path_prefix,
            &workspace_config.styles.oranda_css_version,
        )?;
        Ok(Self {
            project_name: workspace_config.workspace.name.clone().unwrap_or_default(),
            theme: workspace_config.styles.theme.as_css_classes(),
            oranda_css_path: css_path,
            path_prefix: workspace_config.build.path_prefix.clone(),
            ..Default::default()
        })
    }
}

impl OrandaTheme {
    /// Gets the css classes this theme lowers to
    pub fn as_css_classes(&self) -> &'static str {
        match self {
            OrandaTheme::Light => "light",
            OrandaTheme::Dark => "dark",
            OrandaTheme::AxoLight => "axo",
            OrandaTheme::AxoDark => "dark axo",
            OrandaTheme::Hacker => "hacker",
            OrandaTheme::Cupcake => "cupcake",
        }
    }
}
