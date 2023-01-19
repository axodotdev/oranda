use crate::errors::*;
use axohtml::elements::{a, div, script};
use axohtml::types::{Class, SpacedSet};
use axohtml::{html, text};
use cargo_dist_schema::{ArtifactKind, DistManifest};
use serde::Deserialize;

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
    let mut html: Vec<Box<a<String>>> = vec![];
    for release in typed.releases.iter() {
        for artifact in release.artifacts.iter() {
            if let ArtifactKind::ExecutableZip = artifact.kind {
                let mut targets = String::new();
                for targ in artifact.target_triples.iter() {
                    targets.push_str(format!("{} ", targ).as_str());
                }
                let classname: SpacedSet<Class> = "business-button hidden".try_into().unwrap();
                let url = format!(
                    "{}/releases/download/v{}/{}",
                    config.repository.as_ref().unwrap(),
                    config.version.as_ref().unwrap(),
                    artifact.name
                );

                html.extend(
                    html!(<a href=url class=classname data-targets=targets>{text!("Download")}</a>),
                );
            }
        }
    }

    return Ok(Some(
        html!(<div><h3 class="text-center">{text!("Download for your platform")}</h3>{html}</div>),
    ));
}

pub fn get_os_script(config: &Config) -> Result<Box<script<String>>> {
    let detect_os_js = axoasset::copy("src/site/javascript/detect_os.js", &config.dist_dir);

    let path = tokio::runtime::Handle::current().block_on(detect_os_js)?;
    let path_as_string = path.strip_prefix(&config.dist_dir)?.to_string_lossy();
    Ok(html!(<script src=path_as_string />))
}
