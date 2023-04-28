use axohtml::elements::div;
use axohtml::{html, text, unsafe_text};
use cargo_dist_schema::{Artifact as DistArtifact, DistManifest, Release as DistApp};
use chrono::DateTime;

use crate::config::Config;
use crate::data::cargo_dist::{self, DistRelease};
use crate::errors::*;
use crate::site::{icons, link, markdown};

struct InstallerData {
    app: DistApp,
    manifest: DistManifest,
    artifact: DistArtifact,
}

pub fn build_header(latest_release: &DistRelease, config: &Config) -> Result<Box<div<String>>> {
    let downloads_href = link::generate(&config.path_prefix, "artifacts.html");

    let mut html: Vec<Box<div<String>>> = vec![];
    let manifest = &latest_release.manifest;
    for app in manifest.releases.iter() {
        for artifact_id in app.artifacts.iter() {
            let artifact = &manifest.artifacts[artifact_id];
            if let cargo_dist::ArtifactKind::ExecutableZip = artifact.kind {
                let mut targets = String::new();
                for targ in artifact.target_triples.iter() {
                    targets.push_str(format!("{} ", targ).as_str());
                }
                let detect_html = match cargo_dist::get_os(targets.as_str()) {
                    Some(os) => {
                        html!(
                            <span class="detect">{text!("We have detected you are on ")}
                                <span class="detected-os">{text!(os)}</span>
                            {text!(", are we wrong?")}
                            </span>)
                    }
                    None => {
                        html!(<span class="detect">{text!("We couldn't detect the system you are using.")}</span>)
                    }
                };
                let data = InstallerData {
                    app: app.to_owned(),
                    manifest: manifest.to_owned(),
                    artifact: artifact.to_owned(),
                };
                let install_code_block = build_install_block(&data, config);
                let title = format!("Install v{}", app.app_version);
                let formatted_date =
                    match DateTime::parse_from_rfc3339(&latest_release.source.published_at) {
                        Ok(date) => date.format("%b %e %Y at %R UTC").to_string(),
                        Err(_) => latest_release.source.published_at.to_owned(),
                    };
                let date_published = format!("Published at {}", formatted_date);
                html.extend(html!(
                    <div class="hidden target artifact-header" data-targets=&targets>
                        <h4>{text!(title)}</h4>
                        <div>
                            <small class="published-date">{text!(date_published)}</small>
                        </div>
                        {install_code_block}
                        <div>
                            {detect_html}
                            <a href=&downloads_href>{text!("View all installation options")}</a>
                        </div>
                    </div>
                ));
            }
        }
    }

    Ok(html!(
    <div class="artifacts">
        {html}
        <a href=&downloads_href class="hidden backup-download business-button primary">{text!("View installation options")}</a>
    </div>
    ))
}

fn build_install_block(data: &InstallerData, config: &Config) -> Result<Box<div<String>>> {
    // If there's an installer that covers that, prefer it
    if let Ok(install_block) = build_install_block_for_installer(data, config) {
        return Ok(install_block);
    }

    // Otherwise, just link the artifact
    let name = &data.artifact.name.as_ref().unwrap();
    let url = cargo_dist::download_link(config, name, &data.app.app_version)?;
    Ok(html!(
        <div class="install-code-wrapper">
            <a href=url>{text!("Download {}", name)}</a>
        </div>
    ))
}

/// Tries to recommend an installer that installs the given artifact
fn build_install_block_for_installer(
    data: &InstallerData,
    config: &Config,
) -> Result<Box<div<String>>> {
    let install_code = build_install_hint_code(data, config)?;

    let copy_icon = icons::copy();
    let hint = get_install_hint(data, config)?;

    Ok(html!(
        <div class="install-code-wrapper">
            {unsafe_text!(install_code)}
            <button
                data-copy={hint.0}
                class="button primary copy-clipboard-button button">
                {copy_icon}
            </button>
            <a class="button primary button" href=(hint.1)>
                {text!("Source")}
            </a>
        </div>
    ))
}

// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
pub fn build_list(release: &DistRelease, config: &Config) -> Result<Box<div<String>>> {
    let mut list = vec![];
    let manifest = &release.manifest;
    for app in manifest.releases.iter() {
        for artifact_id in app.artifacts.iter() {
            let artifact = &manifest.artifacts[artifact_id];
            if let cargo_dist::ArtifactKind::ExecutableZip = artifact.kind {
                let mut targets = String::new();
                for targ in artifact.target_triples.iter() {
                    targets.push_str(format!("{} ", targ).as_str());
                }

                let title = match artifact.description.clone() {
                    Some(desc) => desc,
                    None => match cargo_dist::get_os(targets.as_str()) {
                        Some(os) => String::from(os),
                        None => targets,
                    },
                };
                let data = InstallerData {
                    app: app.to_owned(),
                    artifact: artifact.to_owned(),
                    manifest: manifest.to_owned(),
                };
                let install_code_block = build_install_block(&data, config);
                list.extend(html!(
                    <li class="list-none">
                        <h5 class="capitalize">{text!(title)}</h5>
                        {install_code_block}
                    </li>
                ))
            }
        }
    }

    Ok(html!(
    <div>
        <h3>{text!("Install via script")}</h3>
        <ul>
            {list}
        </ul>
    </div>
    ))
}

fn build_install_hint_code(data: &InstallerData, config: &Config) -> Result<String> {
    let install_hint = get_install_hint(data, config)?;

    let highlighted_code =
        markdown::syntax_highlight(Some("sh"), &install_hint.0, &config.syntax_theme);
    match highlighted_code {
        Ok(code) => Ok(code),
        Err(_) => Ok(format!(
            "<code class='inline-code'>{}</code>",
            install_hint.0
        )),
    }
}

fn get_install_hint(data: &InstallerData, config: &Config) -> Result<(String, String)> {
    let no_hint_error = OrandaError::Other(
        "There has been an issue getting your install hint, are you using cargo dist?".to_string(),
    );
    let hint = data
        .app
        .artifacts
        .iter()
        .map(|artifact_id| &data.manifest.artifacts[artifact_id])
        .find(|artifact| {
            artifact.install_hint.is_some()
                && artifact
                    .target_triples
                    .iter()
                    .any(|h| data.artifact.target_triples.iter().any(|item| item == h))
        });

    if let Some(current_hint) = hint {
        if let (Some(install_hint), Some(name)) = (&current_hint.install_hint, &current_hint.name) {
            let file_path =
                cargo_dist::write_installer_source(config, name, &data.app.app_version)?;
            Ok((install_hint.to_string(), file_path))
        } else {
            Err(no_hint_error)
        }
    } else {
        Err(no_hint_error)
    }
}
