use axohtml::elements::div;
use axohtml::{html, text};
use serde::Deserialize;

use super::Config;

#[derive(Debug, Deserialize)]
pub struct Artifacts {
    pub cargo_dist: bool,
}

pub fn create_artifacts_tabs(config: &Config) -> Option<Box<div<String>>> {
    let Some(Artifacts { cargo_dist: true }) = &config.artifacts else {
        return None;
      };

    // let cargo_toml = fs::read_to_string(Rust::config(project_root))?;
    // let data: CargoToml = toml::from_str(&cargo_toml)?;

    return Some(html!(<div>{text!("lol")}</div>));
}
