use axoasset::{Asset, LocalAsset};
use axohtml::elements::{div, span};
use axohtml::{html, text, unsafe_text};
use cargo_dist_schema as cargo_dist;
use chrono::DateTime;

use crate::config::Config;
use crate::data::changelog;
use crate::data::releases::github::GithubRelease;
use crate::errors::*;
use crate::site::link;
use crate::site::markdown;

use crate::data::artifacts;

pub fn get_os(name: &str) -> Option<&str> {
    match name.trim() {
        "x86_64-unknown-linux-gnu" => Some("linux"),
        "x86_64-apple-darwin" => Some("mac"),
        "x86_64-apple-silicon" => Some("mac"),
        "x86_64-pc-windows-msvc" => Some("windows"),
        &_ => None,
    }
}

pub fn fetch_manifest(config: &Config) -> Result<(cargo_dist::DistManifest, String)> {
    if let Some(repo) = &config.repository {
        let latest_dist_release = latest_dist_release(repo)?;
        let url =
            create_download_link(config, "dist-manifest.json", &latest_dist_release.tag_name)?;

        match reqwest::blocking::get(&url)?.error_for_status() {
            Ok(resp) => match resp.json::<cargo_dist::DistManifest>() {
                Ok(manifest) => Ok((manifest, latest_dist_release.published_at)),
                Err(e) => Err(OrandaError::CargoDistManifestParseError { url, details: e }),
            },
            Err(e) => Err(OrandaError::CargoDistManifestFetchError {
                url,
                status_code: e.status().unwrap_or(reqwest::StatusCode::BAD_REQUEST),
            }),
        }
    } else {
        Err(OrandaError::Other(
            "Repository is mandatory for the cargo dist option".to_owned(),
        ))
    }
}

fn latest_dist_release(repo: &str) -> Result<GithubRelease> {
    let releases = changelog::fetch_releases(repo)?;
    for release in releases {
        if release.has_dist_manifest() {
            return Ok(release);
        }
    }
    Err(OrandaError::NoCargoDistReleasesFound {
        repo: repo.to_string(),
    })
}

fn get_installer_path(config: &Config, name: &str, version: &str) -> Result<String> {
    let download_link = create_download_link(config, name, version)?;
    let file_string_future = Asset::load_string(download_link.as_str());
    let file_string = tokio::runtime::Handle::current().block_on(file_string_future)?;
    let file_path = format!("{}.txt", &name);
    LocalAsset::write_new(&file_string, &file_path, &config.dist_dir)?;
    Ok(file_path)
}

fn get_install_hint(
    manifest: &cargo_dist::DistManifest,
    release: &cargo_dist::Release,
    target_triples: &[String],
    config: &Config,
) -> Result<(String, String)> {
    let no_hint_error = OrandaError::Other(
        "There has been an issue getting your install hint, are you using cargo dist?".to_string(),
    );
    let hint = release
        .artifacts
        .iter()
        .map(|artifact_id| &manifest.artifacts[artifact_id])
        .find(|artifact| {
            artifact.install_hint.is_some()
                && artifact
                    .target_triples
                    .iter()
                    .any(|h| target_triples.iter().any(|item| item == h))
        });

    if let Some(current_hint) = hint {
        if let (Some(install_hint), Some(name)) = (&current_hint.install_hint, &current_hint.name) {
            let file_path = get_installer_path(config, name, &release.app_version)?;
            Ok((install_hint.to_string(), file_path))
        } else {
            Err(no_hint_error)
        }
    } else {
        Err(no_hint_error)
    }
}

pub fn get_install_hint_code(
    manifest: &cargo_dist::DistManifest,
    release: &cargo_dist::Release,
    target_triples: &[String],
    config: &Config,
) -> Result<String> {
    let install_hint = get_install_hint(manifest, release, target_triples, config)?;

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

fn get_kind_string(kind: &cargo_dist::ArtifactKind) -> String {
    match kind {
        cargo_dist::ArtifactKind::ExecutableZip => String::from("Executable Zip"),
        cargo_dist::ArtifactKind::Symbols => String::from("Symbols"),
        cargo_dist::ArtifactKind::Installer => String::from("Installer"),
        _ => String::from("Unknown"),
    }
}

fn build_install_block(
    config: &Config,
    manifest: &cargo_dist::DistManifest,
    release: &cargo_dist::Release,
    artifact: &cargo_dist::Artifact,
) -> Result<Box<div<String>>> {
    let install_code = get_install_hint_code(manifest, release, &artifact.target_triples, config)?;

    let copy_icon = artifacts::get_copyicon();
    let hint = get_install_hint(manifest, release, &artifact.target_triples, config)?;

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

pub fn build(config: &Config) -> Result<Box<div<String>>> {
    if config.repository.is_none() || config.version.is_none() {
        return Err(OrandaError::Other(String::from(
            "The repository and version are required for cargo_dist",
        )));
    }
    let downloads_href = link::generate(&config.path_prefix, "artifacts.html");
    let (typed, published_at) = fetch_manifest(config)?;

    let mut html: Vec<Box<div<String>>> = vec![];
    for release in typed.releases.iter() {
        for artifact_id in release.artifacts.iter() {
            let artifact = &typed.artifacts[artifact_id];
            if let cargo_dist::ArtifactKind::ExecutableZip = artifact.kind {
                let mut targets = String::new();
                for targ in artifact.target_triples.iter() {
                    targets.push_str(format!("{} ", targ).as_str());
                }
                let detect_html = match get_os(targets.as_str()) {
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
                let install_code_block = build_install_block(config, &typed, release, artifact);
                let title = format!("Install v{}", release.app_version);
                let formatted_date = match DateTime::parse_from_rfc3339(published_at.as_str()) {
                    Ok(date) => date.format("%b %e %Y at %R UTC").to_string(),
                    Err(_) => published_at.to_owned(),
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

pub fn build_table(
    manifest: cargo_dist::DistManifest,
    config: &Config,
) -> Result<Box<div<String>>> {
    let mut table = vec![];
    for release in manifest.releases.iter() {
        for artifact_id in release.artifacts.iter() {
            let artifact = &manifest.artifacts[artifact_id];
            if let Some(name) = artifact.name.clone() {
                let url = create_download_link(config, &name, &release.app_version)?;
                let kind = get_kind_string(&artifact.kind);
                let targets: &String = &artifact.target_triples.clone().into_iter().collect();
                table.extend(vec![
                    html!(<span>{text!(name)}</span>),
                    html!(<span>{text!(kind)}</span>),
                    html!(<span>{text!(targets)}</span>),
                    html!(<span><a href=url>{text!("Download")}</a></span>),
                ]);
            }
        }
    }

    Ok(create_table_content(table))
}

// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
fn create_table_content(table: Vec<Box<span<String>>>) -> Box<div<String>> {
    html!(
    <div>
        <h3>{text!("Downloads")}</h3>
        <div class="table">
            <span class="th">
                {text!("Name")}
            </span>
            <span class="th">
                {text!("Kind")}
            </span>
            <span class="th">
            {text!("Target")}
            </span>
            <span class="th">
                {text!("Download")}
            </span>
            {table}
        </div>
    </div>
    )
}
// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
pub fn build_list(
    manifest: &cargo_dist::DistManifest,
    config: &Config,
) -> Result<Box<div<String>>> {
    let mut list = vec![];
    for release in manifest.releases.iter() {
        for artifact_id in release.artifacts.iter() {
            let artifact = &manifest.artifacts[artifact_id];
            if let cargo_dist::ArtifactKind::ExecutableZip = artifact.kind {
                let mut targets = String::new();
                for targ in artifact.target_triples.iter() {
                    targets.push_str(format!("{} ", targ).as_str());
                }

                let title = match artifact.description.clone() {
                    Some(desc) => desc,
                    None => match get_os(targets.as_str()) {
                        Some(os) => String::from(os),
                        None => targets,
                    },
                };
                let install_code_block = build_install_block(config, manifest, release, artifact);
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

fn create_download_link(config: &Config, name: &str, version: &str) -> Result<String> {
    if let Some(repo) = &config.repository {
        let version_to_use = if version.contains('v') {
            version.split('v').collect::<Vec<&str>>()[1]
        } else {
            version
        };
        Ok(format!(
            "{}/releases/download/v{}/{}",
            repo, version_to_use, name
        ))
    } else {
        Err(OrandaError::Other(
            "Repository is mandatory for the cargo dist option".to_owned(),
        ))
    }
}
