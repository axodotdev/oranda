use crate::config::Config;
use crate::errors::*;
use crate::site::html::build_common_html;
use axohtml::elements::{a, div, script, tr};
use axohtml::{html, text, unsafe_text};
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

fn get_kind_string(kind: &ArtifactKind) -> String {
    match kind {
        ArtifactKind::ExecutableZip => String::from("Executable Zip"),
        ArtifactKind::Symbols => String::from("Symbols"),
        ArtifactKind::Installer => String::from("Installer"),
        _ => String::from("Unknown"),
    }
}

fn create_download_link(config: &Config, name: &String) -> String {
    if let (Some(repo), Some(version)) = (&config.repository, &config.version) {
        format!("{}/releases/download/v{}/{}", repo, version, name)
    } else {
        String::new()
    }
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

    let url = create_download_link(config, &String::from("dist-manifest.json"));

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
                let url = create_download_link(config, &artifact.name);

                html.extend(
                    html!(<a href=url class="block hidden" data-targets=targets><button class="business-button primary">{text!("Download")}</button></a>),
                );
            }
        }
    }

    build_artifacts_html(config, typed)?;
    Ok(Some(html!(
    <div class="artifacts">
        {html}
        <a href="/artifacts.html" class="download-all">{text!("View all downloads")}</a>
    </div>
    )))
}

pub fn get_os_script(config: &Config) -> Result<Box<script<String>>> {
    let detect_os_js = axoasset::copy("src/site/javascript/detect_os.js", &config.dist_dir);

    let path = tokio::runtime::Handle::current().block_on(detect_os_js)?;
    let path_as_string = path.strip_prefix(&config.dist_dir)?.to_string_lossy();
    Ok(html!(<script src=path_as_string />))
}

// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
fn create_content(table: Vec<Box<tr<String>>>) -> Box<div<String>> {
    html!(
    <div>
        <h1>{text!("All downloads")}</h1>
        <table>
            <tr>
                <th>{text!("Name")}</th>
                <th>{text!("Kind")}</th>
                <th>{text!("Download")}</th>
            </tr>
            <tbody>
                {table}
            </tbody>
        </table>
    </div>
    )
}

pub fn build_artifacts_html(config: &Config, manifest: &DistManifest) -> Result<()> {
    let mut table = vec![];
    for release in manifest.releases.iter() {
        for artifact in release.artifacts.iter() {
            let name = &artifact.name;
            let url = create_download_link(config, name);
            let _kind = get_kind_string(&artifact.kind);
            table.push(html!({
                unsafe_text!(
                    "  <tr><td>{name}</td>
                <td>{kind}</td>
                <td><a href=url>Download</a></td>         </tr>"
                )
            }));
        }
    }
    let doc = build_common_html(config, create_content(table))?;
    let html_path = format!("{}/artifacts.html", &config.dist_dir);

    let mut html_file = File::create(html_path)?;
    html_file.write_all(doc.as_bytes())?;
    Ok(())
}
