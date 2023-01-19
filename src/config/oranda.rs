use std::path::PathBuf;

use serde::Deserialize;

use crate::config::analytics::Analytics;
use crate::config::theme::Theme;
use crate::errors::*;
use crate::site::markdown::syntax_highlight::syntax_themes::SyntaxThemes;

#[derive(Debug, Deserialize)]
pub struct Social {
    pub image: Option<String>,
    pub image_alt: Option<String>,
    pub twitter_account: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OrandaConfig {
    pub description: Option<String>,
    pub dist_dir: Option<String>,
    pub homepage: Option<String>,
    pub name: Option<String>,
    pub no_header: Option<bool>,
    pub readme_path: Option<String>,
    pub theme: Option<Theme>,
    pub remote_styles: Option<Vec<String>>,
    pub additional_css: Option<String>,
    pub repository: Option<String>,
    pub syntax_theme: Option<SyntaxThemes>,
    pub analytics: Option<Analytics>,
    pub additional_pages: Option<Vec<String>>,
    pub social: Option<Social>,
    pub logo: Option<String>,
    pub favicon: Option<String>,
}

impl OrandaConfig {
    pub fn load(config_path: &PathBuf) -> Result<Option<OrandaConfig>> {
        println!("reading from oranda config...");
        let config_future = axoasset::load_string(config_path.to_str().unwrap());

        let config = tokio::runtime::Handle::current().block_on(config_future)?;
        let data: OrandaConfig = serde_json::from_str(config.as_str())?;
        println!("read data: {:?}", &data);
        Ok(Some(data))
    }
}
