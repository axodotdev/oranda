use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::config::theme::Theme;
use crate::errors::*;

static ORANDA_JSON: &str = "./oranda.config.json";

#[derive(Debug, Deserialize)]
pub struct OrandaConfig {
    pub description: Option<String>,
    pub dist_dir: Option<String>,
    pub homepage: Option<String>,
    pub name: Option<String>,
    pub no_header: Option<bool>,
    pub readme_path: Option<String>,
    pub theme: Option<Theme>,
}

impl OrandaConfig {
    pub fn load() -> Result<Option<OrandaConfig>> {
        println!("reading from oranda config...");
        if Path::new(ORANDA_JSON).exists() {
            let oranda_json = fs::read_to_string(ORANDA_JSON)?;
            println!("read json: {:?}", &oranda_json);
            let data: OrandaConfig = serde_json::from_str(&oranda_json)?;
            println!("read data: {:?}", &data);
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }
}
