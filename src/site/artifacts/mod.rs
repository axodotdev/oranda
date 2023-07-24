use std::collections::HashMap;

use crate::config::Config;
use crate::data::artifacts::{File, FileIdx, InstallMethod, InstallerIdx, TargetTriple};
use crate::data::{Context, Release};
use crate::errors::*;

use crate::data::artifacts::inference::triple_to_display_name;
use crate::site::javascript;
use serde::Serialize;

/// A list of downloadable files.
///
/// The inner Vec is a list of supported platforms (display name).
type DownloadableFiles = Vec<(FileIdx, File, Vec<String>)>;
/// A map from TargetTriples to Installers that support that platform
///
/// In theory this should be BTreeMap or IndexMap but something in the pipeline from here to
/// jinja seems to be forcing a sorting so it's deterministic..? Can't find docs for this.
type Platforms = HashMap<TargetTriple, Vec<InstallerIdx>>;

#[derive(Serialize, Debug)]
pub struct ArtifactsContext {
    tag: String,
    formatted_date: Option<String>,
    platforms_with_downloads: Platforms,
    downloadable_files: DownloadableFiles,
    release: Release,
    os_script: String,
    has_checksum_files: bool,
}

pub fn template_context(context: &Context, config: &Config) -> Result<Option<ArtifactsContext>> {
    let Some(release) = context.latest() else {
        return Ok(None);
    };
    let os_script = javascript::build_os_script_path(&config.build.path_prefix);
    let platforms_with_downloads = filter_platforms(release);

    let mut downloadable_files: Vec<_> = release
        .artifacts
        .installers()
        .filter_map(|(_, installer)| {
            let InstallMethod::Download { file } = installer.method else {
                return None;
            };
            Some((
                file,
                release.artifacts.file(file).clone(),
                installer
                    .targets
                    .keys()
                    .map(|s| {
                        triple_to_display_name(s)
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| s.clone())
                    })
                    .collect::<Vec<_>>(),
            ))
        })
        .collect();
    downloadable_files.sort_by_key(|(_, f, _)| f.name.clone());

    if downloadable_files.is_empty() {
        tracing::warn!("You seem to have release automation set up, but we didn't detect any releases. The install page and associated widget will be empty. To disable this, set `artifacts: false`");
    }
    let has_checksum_files = downloadable_files
        .iter()
        .any(|(_, f, _)| f.checksum_file.is_some());

    Ok(Some(ArtifactsContext {
        tag: release.source.version_tag().to_string(),
        formatted_date: release.source.formatted_date(),
        platforms_with_downloads,
        release: release.to_owned(),
        downloadable_files,
        os_script,
        has_checksum_files,
    }))
}

/// Only grab platforms that we can actually provide downloadable files for.
pub fn filter_platforms(release: &Release) -> Platforms {
    // First try to select platforms with downloadable artifacts
    let mut platforms = HashMap::new();
    for (target, installer) in release.artifacts.installers_by_target().iter() {
        let has_valid_installer = installer.iter().any(|i| {
            let installer = release.artifacts.installer(i.to_owned());
            matches!(installer.method, InstallMethod::Download { file: _ })
                || is_core_target(target)
        });
        if has_valid_installer {
            platforms.insert(target.clone(), installer.to_vec());
        }
    }

    // If that produces non-empty results, great!
    if !platforms.is_empty() {
        // Quick-and-dirty hack: if there's x64 macos stuff but no arm64 macos stuff,
        // pretend that x64 stuff is arm64, and hope the user has rosetta2 setup.
        //
        // This won't work if the user has never setup rosetta2 and the app is a CLI,
        // because apple's auto-installer for that stuff only works for "real" apps
        // (.app?).
        //
        // Eventually this should be replaced with a more robust notion of "nearby platforms"
        // as described in https://github.com/axodotdev/cargo-dist/issues/202
        if let Some(entries) = platforms.get("x86_64-apple-darwin") {
            if !platforms.contains_key("aarch64-apple-darwin") {
                let entries = entries.clone();
                platforms.insert("aarch64-apple-darwin".to_owned(), entries);
            }
        }
        return platforms;
    }

    // Otherwise, only show things that are on every platform
    let mut universal_installers = vec![];
    if let Some((_, installers)) = release.artifacts.installers_by_target().iter().next() {
        for installer in installers {
            if release
                .artifacts
                .installers_by_target()
                .iter()
                .all(|(_, installers)| installers.contains(installer))
            {
                universal_installers.push(*installer);
            }
        }
    }
    if !universal_installers.is_empty() {
        let mut platforms = Platforms::default();
        platforms.insert("all".to_owned(), universal_installers);
        return platforms;
    }

    // Otherwise it's empty, oh well
    Platforms::default()
}

/// Check if a target belongs to the "big four" sets of targets:
/// - Linux x64
/// - macOS ARM
/// - macOS x64
/// - Windows x64
fn is_core_target(target: &TargetTriple) -> bool {
    target == "x86_64-pc-windows-msvc"
        || target == "aarch64-apple-darwin"
        || target == "x86_64-apple-darwin"
        || target == "x86_64-unknown-linux-gnu"
}
