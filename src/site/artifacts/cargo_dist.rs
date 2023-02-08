use std::path::PathBuf;

use crate::config::Config;
use crate::errors::*;
use crate::site::link;
use crate::site::markdown::syntax_highlight;
use axohtml::dom::UnsafeTextNode;
use axohtml::elements::{div, li, span};
use axohtml::{html, text, unsafe_text};
use cargo_dist_schema::{Artifact, ArtifactKind, DistManifest};

pub fn get_os(name: &str) -> Option<&str> {
    match name.trim() {
        "x86_64-unknown-linux-gnu" => Some("linux"),
        "x86_64-apple-darwin" => Some("mac"),
        "x86_64-apple-silicon" => Some("mac"),
        "x86_64-pc-windows-msvc" => Some("windows"),
        &_ => None,
    }
}

pub fn fetch_manifest(config: &Config) -> std::result::Result<DistManifest, reqwest::Error> {
    let url = create_download_link(config, &String::from("dist-manifest.json"));
    let resp = reqwest::blocking::get(url)?;

    resp.json::<DistManifest>()
}

fn get_installer_path(config: &Config, name: &String) -> Result<String> {
    let download_link = create_download_link(config, &name);
    let file_string_future = axoasset::load_string(download_link.as_str());
    let file_string = tokio::runtime::Handle::current().block_on(file_string_future)?;
    let file_path = format!("{}.txt", &name);
    let asset = axoasset::local::LocalAsset::new(
        format!("{}/{}", &config.dist_dir, &file_path).as_str(),
        file_string.as_bytes().to_vec(),
    );
    asset.write(&config.dist_dir)?;
    Ok(file_path)
}

fn get_install_hint(
    artifacts: &[Artifact],
    target_triples: &[String],
    config: &Config,
) -> Result<(String, String)> {
    let no_hint_error = OrandaError::Other(
        "There has been an issue getting your install hint, are you using cargo dist?".to_string(),
    );
    let hint = artifacts.iter().find(|artifact| {
        artifact.install_hint.is_some()
            && artifact
                .target_triples
                .iter()
                .any(|h| target_triples.iter().any(|item| item == h))
    });

    if let Some(current_hint) = hint {
        if let Some(install_hint) = &current_hint.install_hint {
            let file_path = get_installer_path(&config, &current_hint.name)?;
            Ok((String::from(install_hint), file_path))
        } else {
            Err(no_hint_error)
        }
    } else {
        Err(no_hint_error)
    }
}

pub fn get_install_hint_code(
    artifacts: &[Artifact],
    target_triples: &[String],
    config: &Config,
) -> Result<String> {
    let install_hint = get_install_hint(artifacts, target_triples, config)?;

    let highlighted_code =
        syntax_highlight(Some("sh"), install_hint.0.as_str(), &config.syntax_theme);
    match highlighted_code {
        Ok(code) => Ok(code),
        Err(_) => Ok(format!(
            "<code class='text-center break-all'>{}</code>",
            install_hint.0
        )),
    }
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

pub fn build(config: &Config) -> Result<Box<div<String>>> {
    if config.repository.is_none() || config.version.is_none() {
        return Err(OrandaError::Other(String::from(
            "The repository and version are required for cargo_dist",
        )));
    }
    let downloads_href = link::generate(&config.path_prefix, String::from("artifacts.html"));
    let typed = fetch_manifest(config)?;

    let mut html: Vec<Box<div<String>>> = vec![];
    for release in typed.releases.iter() {
        for artifact in release.artifacts.iter() {
            if let ArtifactKind::ExecutableZip = artifact.kind {
                let mut targets = String::new();
                for targ in artifact.target_triples.iter() {
                    targets.push_str(format!("{} ", targ).as_str());
                }
                let install_code =
                    get_install_hint_code(&release.artifacts, &artifact.target_triples, config)?;
                let detect_text = match get_os(targets.as_str()) {
                    Some(os) => format!("We have detected you are on {}, are we wrong?", os),
                    None => String::from("We couldn't detect the system you are using."),
                };

                // axohtml does not support SVG for now
                let copy_icon:  Box<UnsafeTextNode<String>> = unsafe_text!("<svg stroke='currentColor' fill='currentColor' stroke-width='0' viewBox='0 0 20 20' height='1em' width='1em' xmlns='http://www.w3.org/2000/svg'><path d='M8 2a1 1 0 000 2h2a1 1 0 100-2H8z'></path><path d='M3 5a2 2 0 012-2 3 3 0 003 3h2a3 3 0 003-3 2 2 0 012 2v6h-4.586l1.293-1.293a1 1 0 00-1.414-1.414l-3 3a1 1 0 000 1.414l3 3a1 1 0 001.414-1.414L10.414 13H15v3a2 2 0 01-2 2H5a2 2 0 01-2-2V5zM15 11h2a1 1 0 110 2h-2v-2z'></path></svg>");
                let hint = get_install_hint(&release.artifacts, &artifact.target_triples, config)?;

                html.extend(html!(
                    <div class="hidden target artifact-header" data-targets=&targets>
                        <h4 class="text-center">{text!("Install")}</h4>
                        <div class="install-code-wrapper">
                            {unsafe_text!(install_code)}
                            <button data-copy={hint.0} class="business-button primary copy-clipboard-button button">{copy_icon}</button>
                            <a class="business-button primary button" href=(hint.1)>
                                {text!("Source")}
                            </a>
                        </div>
                        <div>
                            <span class="text-center detect">
                                {text!(detect_text)}
                            </span>
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

pub fn build_table(manifest: DistManifest, config: &Config) -> Box<div<String>> {
    let mut table = vec![];
    for release in manifest.releases.iter() {
        for artifact in release.artifacts.iter() {
            let name = &artifact.name;
            let url = create_download_link(config, name);
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

    create_table_content(table)
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
pub fn build_list(manifest: &DistManifest, config: &Config) -> Result<Vec<Box<li<String>>>> {
    let mut list = vec![];
    for release in manifest.releases.iter() {
        for artifact in release.artifacts.iter() {
            if let ArtifactKind::ExecutableZip = artifact.kind {
                let mut targets = String::new();
                for targ in artifact.target_triples.iter() {
                    targets.push_str(format!("{} ", targ).as_str());
                }
                let install_code =
                    get_install_hint_code(&release.artifacts, &artifact.target_triples, config)?;
                list.extend(html!(
                    <li class="list-none">
                        <h5>{text!(targets)}</h5>
                        {unsafe_text!(install_code)}
                    </li>
                ))
            }
        }
    }

    Ok(list)
}
