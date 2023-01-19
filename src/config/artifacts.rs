use axohtml::elements::div;
use axohtml::{html, text};
use cargo_dist_schema::DistManifest;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use std::path::PathBuf;

use crate::errors::*;

use super::Config;

#[derive(Debug, Deserialize)]
pub struct Artifacts {
    pub cargo_dist: bool,
}

pub fn create_artifacts_tabs(config: &Config) -> Result<Option<Box<div<String>>>> {
    let Some(Artifacts { cargo_dist: true }) = &config.artifacts else {
        return Ok(None);
      };

    if config.repository.is_none() || config.version.is_none() {
        return Err(OrandaError::Other(String::from(
            "The repository and version are required for cargo_dist",
        )));
    }

    let url = format!(
        "{}/releases/download/v{}/dist-manifest.json",
        config.repository.as_ref().unwrap(),
        config.version.as_ref().unwrap()
    );

    let resp = reqwest::blocking::get(url);

    let Ok(resp) = resp else {
        return Err(OrandaError::Other(String::from(
            "The repository and version configurations are required for cargo_dist",
        )));
      };

    let typed = &resp.json::<DistManifest>()?;

    println!("{:?}", typed.releases);
    let mut html: Vec<Box<div<String>>> = vec![];
    for release in typed.releases.iter() {
        for artifact in release.artifacts.iter() {
            html.extend(html!(<div>{text!(&artifact.name)}</div>));
        }
    }

    return Ok(Some(
        html!(<div><h2 class="text-center">{text!("Download for your platform")}</h2>{html}</div>),
    ));
}
