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
        "x86_64-unknown-linux-gnu" => Some("x64 Linux"),
        "x86_64-apple-darwin" => Some("x64 macOS"),
        "aarch64-apple-darwin" => Some("arm64 macOS"),
        "x86_64-pc-windows-msvc" => Some("x64 Windows"),
        &_ => None,
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
                let label;
                let method;
                let preference;
                let file = artifact.name.as_ref().and_then(|n| self.file_idx(n));

                // If this artifact has a checksum, register it
                let checksum_file = artifact.checksum.as_ref().and_then(|n| self.file_idx(n));
                if let Some(file) = file {
                    self.file_mut(file).checksum_file = checksum_file;
                }

                match artifact.kind {
                    ArtifactKind::ExecutableZip => {
                        // Skip this if the file is somehow missing
                        let Some(file) = file else {
                            continue;
                        };
                        label = if id.ends_with(".zip") {
                            "zip".to_owned()
                        } else {
                            "tarball".to_owned()
                        };
                        method = InstallMethod::Download { file };
                        preference = InstallerPreference::Archive;
                    }
                    ArtifactKind::Installer => {
                        if let Some(install_hint) = &artifact.install_hint {
                            // If there's an install-hint, assume this is something we're telling them to run
                            //
                            // Special hack: demote npm-packages, which cargo-dist presents kind of weird
                            let file = if id.contains("npm-package") {
                                preference = InstallerPreference::Custom;
                                None
                            } else {
                                preference = InstallerPreference::Script;
                                file
                            };

                            method = InstallMethod::Run {
                                file,
                                run_hint: install_hint.clone(),
                            };
                        } else if let Some(file) = file {
                            // If there's no install-hint, but there is a proper file name, just suggest downloading it
                            // while assuming this is some kind of custom installer
                            method = InstallMethod::Download { file };
                            preference = InstallerPreference::Custom;
                        } else {
                            // Must be some new cargo-dist thing we don't understand, move along
                            continue;
                        };
                        label = if id.ends_with(".sh") {
                            "shell".to_owned()
                        } else if id.ends_with(".ps1") {
                            "powershell".to_owned()
                        } else if id.contains("npm-package") {
                            "npm".to_owned()
                        } else {
                            Utf8PathBuf::from(id).extension().unwrap_or(id).to_owned()
                        };
                    }
                    _ => {
                        // We don't care about these *yet*
                        // (notably skipped: Symbols, Checksum)
                        continue;
                    }
                };
                let targets = preference_to_targets(artifact.target_triples.clone(), preference);
                let installer = Installer {
                    label,
                    description: artifact.description.clone().unwrap_or_default(),
                    targets,
                    method,
                    ignore: false,
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
