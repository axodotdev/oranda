use axohtml::elements::{div, script};
use axohtml::{html, text};
use cargo_dist_schema::{ArtifactKind, DistManifest};
use serde::Deserialize;

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
            if let ArtifactKind::ExecutableZip = artifact.kind {
                html.extend(html!(<div>{text!(&artifact.name)}</div>));
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
