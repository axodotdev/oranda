use crate::config::Config;
use crate::errors::*;
use crate::site::html::build_common_html;
use axohtml::elements::{a, div, script};
use axohtml::types::{Class, SpacedSet};
use axohtml::{html, text};
use cargo_dist_schema::{ArtifactKind, DistManifest};
use serde::Deserialize;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ArtifactSystem {
    Windows,
    Windows64,
    WindowsArm,

    Mac,
    MacPpc,
    Mac32,
    MacSilicon,

    Linux,
    LinuxUbuntu,
    LinuxDebian,
    LinuxMandriva,
    LinuxRedhat,
    LinuxFedora,
    LinuxSuse,
    LinuxGentoo,

    Ios,
    Android,

    Freebsd,
}

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

    let mut html: Vec<Box<a<String>>> = vec![];
    for release in typed.releases.iter() {
        for artifact in release.artifacts.iter() {
            if let ArtifactKind::ExecutableZip = artifact.kind {
                let mut targets = String::new();
                for targ in artifact.target_triples.iter() {
                    targets.push_str(format!("{} ", targ).as_str());
                }
                let classname: SpacedSet<Class> = "block hidden".try_into().unwrap();
                let url = format!(
                    "{}/releases/download/v{}/{}",
                    config.repository.as_ref().unwrap(),
                    config.version.as_ref().unwrap(),
                    artifact.name
                );

                html.extend(
                    html!(<a href=url class=classname data-targets=targets><button class="business-button primary">{text!("Download")}</button></a>),
                );
            }
        }
    }

    build_artifacts_html(config)?;
    return Ok(Some(html!(
    <div class="artifacts">
        <h3 class="text-center">{text!("Download for your platform")}</h3>{html}
        <a href="/artifacts.html" class="download-all">{text!("View all downloads")}</a>
    </div>
    )));
}

pub fn get_os_script(config: &Config) -> Result<Box<script<String>>> {
    let detect_os_js = axoasset::copy("src/site/javascript/detect_os.js", &config.dist_dir);

    let path = tokio::runtime::Handle::current().block_on(detect_os_js)?;
    let path_as_string = path.strip_prefix(&config.dist_dir)?.to_string_lossy();
    Ok(html!(<script src=path_as_string />))
}

pub fn build_artifacts_html(config: &Config) -> Result<()> {
    let content = html!(
        <div>
            <h1>{text!("All downloads")}</h1>
        </div>
    );
    let doc = build_common_html(config, content)?;
    let html_path = format!("{}/artifacts.html", &config.dist_dir);

    let mut html_file = File::create(html_path)?;
    html_file.write_all(doc.as_bytes())?;
    Ok(())
}
