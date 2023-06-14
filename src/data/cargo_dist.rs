use axoasset::{Asset, LocalAsset};
use camino::Utf8PathBuf;
pub use cargo_dist_schema::{Artifact, ArtifactKind, DistManifest, Release};

use crate::config::Config;
use crate::data::github::GithubRelease;
use crate::errors::*;

use super::artifacts::{
    preference_to_targets, InstallMethod, Installer, InstallerPreference, ReleaseArtifacts,
};

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
        LocalAsset::write_new(&file_string, &full_file_path)?;
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

impl ReleaseArtifacts {
    /// Incorporate data from cargo-dist into the ReleaseArtifacts
    pub fn add_cargo_dist(&mut self, manifest: &DistManifest) {
        // NOTE: this code currently assumes `self.files` has already been populated
        // by e.g. calling `add_github` or whatever future system for discovering artifacts.
        // If the manifest refers to files that don't exist, they will be skipped.

        for app in &manifest.releases {
            // If we're trying to restrict to a specific app, ignore releases of other ones
            // (future-proofing for multi-tenant oranda work)
            if let Some(app_name) = &self.app_name {
                if app_name != &app.app_name {
                    continue;
                }
            }

            for (id, artifact) in manifest.artifacts_for_release(app) {
                let method;
                let preference;
                let file = artifact.name.as_ref().and_then(|n| self.file_idx(n));
                match artifact.kind {
                    ArtifactKind::ExecutableZip => {
                        // Skip this if the file is somehow missing
                        let Some(file) = file else {
                            continue;
                        };
                        method = InstallMethod::Download { file };
                        preference = InstallerPreference::Archive;
                    }
                    ArtifactKind::Installer => {
                        // If this is missing then this is an information-only installer, with no actual file
                        // (e.g. just a string saying "install with `npm -i my-app`"). As of this writing
                        // this is not a thing that cargo-dist *can* produce, but this field is optional
                        // precisely so that it can support this in the future. So if we pretend it's a thing
                        // now, we'll Just Work if it ever does become a thing.
                        if let Some(install_hint) = &artifact.install_hint {
                            // If there's an install-hint, assume this is something we're telling them to run
                            method = InstallMethod::Run {
                                file,
                                run_hint: install_hint.clone(),
                            };
                            preference = InstallerPreference::Script;
                        } else if let Some(file) = file {
                            // If there's no install-hint, but there is a proper file name, just suggest downloading it
                            // while assuming this is some kind of custom installer
                            method = InstallMethod::Download { file };
                            preference = InstallerPreference::Custom;
                        } else {
                            // Must be some new cargo-dist thing we don't understand, move along
                            continue;
                        };
                    }
                    _ => {
                        // We don't care about these *yet*
                        // (notably skipped: Symbols, Checksum)
                        continue;
                    }
                };
                // Use the installer's description as a label, otherwise use the id..?
                let label = artifact
                    .description
                    .clone()
                    .unwrap_or_else(|| id.to_owned());
                let targets = preference_to_targets(artifact.target_triples.clone(), preference);
                let installer = Installer {
                    label,
                    targets,
                    method,
                };
                self.add_installer(installer);

                // If this is a proper file, disable inference of this file's properties
                if let Some(file) = file {
                    self.file_mut(file).infer = false;
                }
            }
        }
    }
}
