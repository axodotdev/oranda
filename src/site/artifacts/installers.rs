use axohtml::elements::div;
use axohtml::{html, text, unsafe_text};
use chrono::DateTime;

use crate::config::Config;
use crate::data::cargo_dist;
use crate::errors::*;
use crate::site::{icons, link, markdown};

pub fn build_header(config: &Config) -> Result<Box<div<String>>> {
    if config.repository.is_none() || config.version.is_none() {
        return Err(OrandaError::Other(String::from(
            "The repository and version are required for cargo_dist",
        )));
    }
    let downloads_href = link::generate(&config.path_prefix, "artifacts.html");
    let gh_distrelease = cargo_dist::fetch_release(config)?;

    let mut html: Vec<Box<div<String>>> = vec![];
    for release in gh_distrelease.manifest.releases.iter() {
        for artifact_id in release.artifacts.iter() {
            let artifact = &gh_distrelease.manifest.artifacts[artifact_id];
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
                let install_code_block =
                    build_install_block(config, &gh_distrelease.manifest, release, artifact);
                let title = format!("Install v{}", release.app_version);
                let formatted_date =
                    match DateTime::parse_from_rfc3339(gh_distrelease.publish_date.as_str()) {
                        Ok(date) => date.format("%b %e %Y at %R UTC").to_string(),
                        Err(_) => gh_distrelease.publish_date.to_owned(),
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

fn build_install_block(
    config: &Config,
    manifest: &cargo_dist::DistManifest,
    release: &cargo_dist::Release,
    artifact: &cargo_dist::Artifact,
) -> Result<Box<div<String>>> {
    // If there's an installer that covers that, prefer it
    if let Ok(val) = build_install_block_for_installer(config, manifest, release, artifact) {
        return Ok(val);
    }

    // Otherwise, just link the artifact
    let url = cargo_dist::download_link(
        config,
        artifact.name.as_ref().unwrap(),
        &release.app_version,
    )?;
    Ok(html!(
        <div class="install-code-wrapper">
            <a href=url>{text!("Download {}", artifact.name.as_ref().unwrap())}</a>
        </div>
    ))
}

/// Tries to recommend an installer that installs the given artifact
fn build_install_block_for_installer(
    config: &Config,
    manifest: &cargo_dist::DistManifest,
    release: &cargo_dist::Release,
    artifact: &cargo_dist::Artifact,
) -> Result<Box<div<String>>> {
    let install_code =
        build_install_hint_code(manifest, release, &artifact.target_triples, config)?;

    let copy_icon = icons::copy();
    let hint = cargo_dist::get_install_hint(manifest, release, &artifact.target_triples, config)?;

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
pub fn build_list(
    dist_release: &cargo_dist::DistRelease,
    config: &Config,
) -> Result<Box<div<String>>> {
    let mut list = vec![];
    for release in dist_release.manifest.releases.iter() {
        for artifact_id in release.artifacts.iter() {
            let artifact = &dist_release.manifest.artifacts[artifact_id];
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
                let install_code_block =
                    build_install_block(config, &dist_release.manifest, release, artifact);
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

pub fn build_install_hint_code(
    manifest: &cargo_dist::DistManifest,
    release: &cargo_dist::Release,
    target_triples: &[String],
    config: &Config,
) -> Result<String> {
    let install_hint = cargo_dist::get_install_hint(manifest, release, target_triples, config)?;

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
