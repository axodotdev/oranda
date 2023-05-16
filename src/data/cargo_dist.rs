use axoasset::{Asset, LocalAsset};
use camino::Utf8PathBuf;
pub use cargo_dist_schema::{Artifact, ArtifactKind, DistManifest, Release};

use crate::config::Config;
use crate::data::github::GithubRelease;
use crate::errors::*;

pub const MANIFEST_FILENAME: &str = "dist-manifest.json";

#[derive(Clone, Debug)]
pub struct DistRelease {
    pub manifest: DistManifest,
    pub source: GithubRelease,
}

pub fn get_os(name: &str) -> Option<&str> {
    match name.trim() {
        "x86_64-unknown-linux-gnu" => Some("linux"),
        "x86_64-apple-darwin" => Some("mac"),
        "aarch64-apple-darwin" => Some("arm mac"),
        "x86_64-pc-windows-msvc" => Some("windows"),
        &_ => None,
    }
}

/// Make the source of an installer script available on the server
pub fn write_installer_source(config: &Config, name: &str, version: &str) -> Result<String> {
    let file_path = format!("{}.txt", &name);
    let full_file_path = Utf8PathBuf::from(&config.dist_dir).join(&file_path);
    if !full_file_path.exists() {
        let download_link = download_link(config, name, version)?;
        let file_string_future = Asset::load_string(download_link.as_str());
        let file_string = tokio::runtime::Handle::current().block_on(file_string_future)?;
        LocalAsset::write_new(&file_string, &file_path, &config.dist_dir)?;
    }
    Ok(file_path)
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
