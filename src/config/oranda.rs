use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::config::analytics::Analytics;
use crate::config::theme::Theme;
use crate::errors::*;
use crate::message::{Message, MessageType};
use crate::site::markdown::syntax_highlight::syntax_themes::SyntaxThemes;

#[derive(Debug, Deserialize, Serialize)]
pub struct Social {
    pub image: Option<String>,
    pub image_alt: Option<String>,
    pub twitter_account: Option<String>,
}

static ORANDA_JSON: &str = "./oranda.json";

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
    pub fn load() -> Result<Option<OrandaConfig>> {
        if Path::new(ORANDA_JSON).exists() {
            Message::new(MessageType::Info, "Found oranda config...").print_and_log();
            let oranda_json = fs::read_to_string(ORANDA_JSON)?;
            let data: OrandaConfig = serde_json::from_str(&oranda_json)?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }
}
