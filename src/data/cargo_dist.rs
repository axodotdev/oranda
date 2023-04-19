use axoasset::{Asset, LocalAsset};
pub use cargo_dist_schema::{Artifact, ArtifactKind, DistManifest, Release};

use crate::config::Config;
use crate::data::github::GithubRelease;
use crate::errors::*;

pub fn get_os(name: &str) -> Option<&str> {
    match name.trim() {
        "x86_64-unknown-linux-gnu" => Some("linux"),
        "x86_64-apple-darwin" => Some("mac"),
        "aarch64-apple-darwin" => Some("arm mac"),
        "x86_64-pc-windows-msvc" => Some("windows"),
        &_ => None,
    }
}

pub struct DistRelease {
    pub manifest: DistManifest,
    pub publish_date: String,
}

pub fn fetch_release(config: &Config) -> Result<DistRelease> {
    if let Some(repo) = &config.repository {
        let latest_dist_release = fetch_latest_github_release(repo)?;
        let url = download_link(config, "dist-manifest.json", &latest_dist_release.tag_name)?;

        match reqwest::blocking::get(&url)?.error_for_status() {
            Ok(resp) => match resp.json::<DistManifest>() {
                Ok(manifest) => Ok(DistRelease {
                    manifest,
                    publish_date: latest_dist_release.published_at,
                }),
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

fn fetch_latest_github_release(repo: &str) -> Result<GithubRelease> {
    let releases = GithubRelease::fetch_all(repo)?;
    for release in releases {
        if release.has_dist_manifest() {
            return Ok(release);
        }
    }
    Err(OrandaError::NoCargoDistReleasesFound {
        repo: repo.to_string(),
    })
}

pub fn write_installer_source(config: &Config, name: &str, version: &str) -> Result<String> {
    let download_link = download_link(config, name, version)?;
    let file_string_future = Asset::load_string(download_link.as_str());
    let file_string = tokio::runtime::Handle::current().block_on(file_string_future)?;
    let file_path = format!("{}.txt", &name);
    LocalAsset::write_new(&file_string, &file_path, &config.dist_dir)?;
    Ok(file_path)
}

pub fn get_install_hint(
    manifest: &DistManifest,
    release: &Release,
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
            let file_path = write_installer_source(config, name, &release.app_version)?;
            Ok((install_hint.to_string(), file_path))
        } else {
            Err(no_hint_error)
        }
    } else {
        Err(no_hint_error)
    }
}

pub fn get_kind_string(kind: &ArtifactKind) -> String {
    match kind {
        ArtifactKind::ExecutableZip => String::from("Executable Zip"),
        ArtifactKind::Symbols => String::from("Symbols"),
        ArtifactKind::Installer => String::from("Installer"),
        _ => String::from("Unknown"),
    }
}

pub fn download_link(config: &Config, name: &str, version: &str) -> Result<String> {
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
