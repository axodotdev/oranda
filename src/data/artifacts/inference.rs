use camino::Utf8PathBuf;

use super::*;

// Architectures
// const ARCH_X86: &str = "i686";
// const ARCH_X64: &str = "x86_64";
// const ARCH_ARM64: &str = "aarch64";

// OSes
// const OS_WINDOWS: &str = "pc-windows-msvc";
// const OS_LINUX_GNU: &str = "unknown-linux-gnu";
// const OS_LINUX_MUSL: &str = "unknown-linux-musl";
// const OS_MAC: &str = "apple-darwin";

use axoproject::platforms::{KNOWN_LINUX_TARGETS, KNOWN_MAC_TARGETS, KNOWN_WINDOWS_TARGETS};

// Various extensions for known archive formats
const EXTS_FOR_TAR_BZIP2: &[&str] = &[".tar.bz2", ".tb2", ".tbz", ".tbz2", ".tz2"];
const EXTS_FOR_TAR_GZIP: &[&str] = &[".tar.gz", ".taz", ".tgz"];
const EXTS_FOR_TAR_LZIP: &[&str] = &[".tar.lz"];
const EXTS_FOR_TAR_LZMA: &[&str] = &[".tar.lzma", ".tlz"];
const EXTS_FOR_TAR_XZ: &[&str] = &[".tar.xz", ".txz"];
const EXTS_FOR_TAR_COMPRESS: &[&str] = &[".tar.Z", ".tZ", ".taZ"];
const EXTS_FOR_TAR_ZSTD: &[&str] = &[".tar.zst", ".tzst"];
const EXTS_FOR_TAR_BROTLI: &[&str] = &[".tar.br"];
const EXTS_FOR_ZIP: &[&str] = &[".zip"];
const EXTS_FOR_RAR: &[&str] = &[".rar"];
const EXTS_FOR_7ZIP: &[&str] = &[".7z"];

const KNOWN_ARCHIVE_EXTS: &[&[&str]] = &[
    EXTS_FOR_TAR_BZIP2,
    EXTS_FOR_TAR_GZIP,
    EXTS_FOR_TAR_LZIP,
    EXTS_FOR_TAR_LZMA,
    EXTS_FOR_TAR_XZ,
    EXTS_FOR_TAR_COMPRESS,
    EXTS_FOR_TAR_ZSTD,
    EXTS_FOR_TAR_BROTLI,
    EXTS_FOR_ZIP,
    EXTS_FOR_RAR,
    EXTS_FOR_7ZIP,
];

// Various extensions for known "bundle" formats ("native installers")
const EXT_BUNDLE_MSI: &str = ".msi";
const EXT_BUNDLE_APP: &str = ".app";
const EXT_BUNDLE_DMG: &str = ".dmg";
const EXT_BUNDLE_DEB: &str = ".deb";
const EXT_BUNDLE_RPM: &str = ".rpm";
// annoying subtlety: pacman (arch linux) uses .pkg.tar.* files,
// so we need to use "contains" instead of "ends_with" for bundles
const EXT_BUNDLE_PACMAN: &str = ".pkg.tar.";
const EXT_BUNDLE_FLATPAK: &str = ".flatpak";
const EXT_BUNDLE_SNAP: &str = ".snap";

const KNOWN_WINDOWS_BUNDLE_EXTS: &[&str] = &[EXT_BUNDLE_MSI];
const KNOWN_MAC_BUNDLE_EXTS: &[&str] = &[EXT_BUNDLE_APP, EXT_BUNDLE_DMG];
const KNOWN_LINUX_BUNDLE_EXTS: &[&str] = &[
    EXT_BUNDLE_DMG,
    EXT_BUNDLE_DEB,
    EXT_BUNDLE_RPM,
    EXT_BUNDLE_PACMAN,
    EXT_BUNDLE_FLATPAK,
    EXT_BUNDLE_SNAP,
];
const KNOWN_BUNDLE_EXTS: &[&str] = &[
    EXT_BUNDLE_MSI,
    EXT_BUNDLE_APP,
    EXT_BUNDLE_DMG,
    EXT_BUNDLE_DEB,
    EXT_BUNDLE_RPM,
    EXT_BUNDLE_FLATPAK,
    EXT_BUNDLE_SNAP,
    EXT_BUNDLE_PACMAN,
];

// Various extensions for
const EXT_SCRIPT_SHELL: &str = ".sh";
const EXT_SCRIPT_POWERSHELL: &str = ".ps1";
// FIXME: could add windows' .bat..? or is that more like a bundle?

const KNOWN_WINDOWS_SCRIPT_EXTS: &[&str] = &[EXT_SCRIPT_POWERSHELL];
const KNOWN_UNIX_SCRIPT_EXTS: &[&str] = &[EXT_SCRIPT_SHELL];
pub(crate) const KNOWN_SCRIPT_EXTS: &[&str] = &[EXT_SCRIPT_SHELL, EXT_SCRIPT_POWERSHELL];

impl ReleaseArtifacts {
    /// Infer installers/artifacts based solely on file names
    pub fn add_inference(&mut self) {
        // Gotta clone this upfront to avoid borrowing stuff
        let app_name = self.app_name.clone();
        for file_idx in self.file_indices() {
            let file = self.file_mut(file_idx);
            // Skip this
            if !file.infer {
                continue;
            }
            if let Some(app_name) = &app_name {
                // If we're trying to restrict to a specific app, ignore files that don't contain
                // the app name (future-proofing for multi-tenant oranda work)
                if !file.name.contains(app_name) {
                    continue;
                }
            }

            // Search for target triples in the file name
            let mut targets = vec![];
            for target in KNOWN_TARGET_TRIPLES.iter().copied().flatten().copied() {
                if file.name.contains(target) {
                    targets.push(target.to_owned());
                }
            }

            let label;
            let description = String::new();
            let method;
            let preference;

            // Try to detect what kind of file this is
            if file.name.contains("install")
                && KNOWN_SCRIPT_EXTS.iter().any(|ext| file.name.ends_with(ext))
            {
                // Looks like an installer script! Recommend a ~curl|sh for it.
                //
                // If this script doesn't have targets, infer them
                if targets.is_empty() {
                    targets = infer_targets_for_script(file);
                }
                let run_hint = infer_run_hint_for_script(file);
                label = infer_label_for_script(file);
                preference = InstallerPreference::Script;
                method = InstallMethod::Run {
                    file: Some(file_idx),
                    run_hint,
                };
            } else if KNOWN_BUNDLE_EXTS.iter().any(|ext| file.name.ends_with(ext)) {
                // Looks like an installer bundle! Recommend a download.
                //
                // NOTE: the above check is intentionally "contains" and not "ends_with" because
                // arch packages are .pkg.tar.* and that's really annoying to handle.
                //
                // If this bundle doesn't have targets, infer them
                if targets.is_empty() {
                    targets = infer_targets_for_bundle(file);
                }
                label = infer_label_for_bundle(file);
                preference = InstallerPreference::Native;
                method = InstallMethod::Download { file: file_idx };
            } else if KNOWN_ARCHIVE_EXTS
                .iter()
                .copied()
                .flatten()
                .any(|ext| file.name.ends_with(ext))
            {
                // Looks like this is an archive containing a binary! Recommend a download.
                // Skip anything without a target triple, because we can't use it otherwise,
                // and it might just be something like a source dump.
                if targets.is_empty() {
                    continue;
                }
                label = infer_label_for_archive(file);
                preference = InstallerPreference::Archive;
                method = InstallMethod::Download { file: file_idx };
            } else {
                // Nothing we recognize
                continue;
            }

            let targets = preference_to_targets(targets, preference);
            let installer = Installer {
                label,
                description,
                targets,
                method,
                display: DisplayPreference::Preferred,
            };
            self.add_installer(installer);
        }
    }
}

/// Given a file that appears to be a "bundle" but doesn't specify a target,
/// infer the targets it applies to
fn infer_targets_for_bundle(file: &File) -> Vec<TargetTriple> {
    let mut targets = vec![];
    if KNOWN_WINDOWS_BUNDLE_EXTS
        .iter()
        .any(|ext| file.name.contains(ext))
    {
        targets.extend(KNOWN_WINDOWS_TARGETS.iter().copied().map(|t| t.to_owned()));
    }
    if KNOWN_MAC_BUNDLE_EXTS
        .iter()
        .any(|ext| file.name.contains(ext))
    {
        targets.extend(KNOWN_MAC_TARGETS.iter().copied().map(|t| t.to_owned()));
    }
    if KNOWN_LINUX_BUNDLE_EXTS
        .iter()
        .any(|ext| file.name.contains(ext))
    {
        targets.extend(
            KNOWN_LINUX_TARGETS
                .iter()
                .copied()
                .flatten()
                .copied()
                .map(|t| t.to_owned()),
        );
    }
    targets
}

/// Given a file that appears to be a "script" but doesn't specify a target, infer the targets it applies to
fn infer_targets_for_script(file: &File) -> Vec<TargetTriple> {
    let mut targets = vec![];
    if KNOWN_WINDOWS_SCRIPT_EXTS
        .iter()
        .any(|ext| file.name.contains(ext))
    {
        targets.extend(KNOWN_WINDOWS_TARGETS.iter().copied().map(|t| t.to_owned()));
    }
    if KNOWN_UNIX_SCRIPT_EXTS
        .iter()
        .any(|ext| file.name.contains(ext))
    {
        targets.extend(
            KNOWN_LINUX_TARGETS
                .iter()
                .copied()
                .flatten()
                .copied()
                .map(|t| t.to_owned()),
        );
        targets.extend(KNOWN_MAC_TARGETS.iter().copied().map(|t| t.to_owned()));
    }
    targets
}

/// Infer the command to curl|sh a script
fn infer_run_hint_for_script(file: &File) -> String {
    if file.name.ends_with(EXT_SCRIPT_SHELL) {
        format!(
            "curl --proto '=https' --tlsv1.2 -LsSf {} | sh",
            file.download_url
        )
    } else if file.name.ends_with(EXT_SCRIPT_POWERSHELL) {
        format!("irm {} | iex", file.download_url)
    } else {
        unimplemented!(
            "Looks like someone added a new kind of script but didn't add a run hint for it?"
        );
    }
}

/// Infer the label for a bundle
fn infer_label_for_bundle(file: &File) -> String {
    // For now just use the extension
    Utf8PathBuf::from(&file.name)
        .extension()
        .expect("we determined a file was a bundle based on its extension, but it had none?")
        .to_owned()
}

/// Infer the label for a tarball/zip
fn infer_label_for_archive(file: &File) -> String {
    // For now just use the extension
    if EXTS_FOR_RAR.iter().any(|ext| file.name.ends_with(ext)) {
        "rar".to_owned()
    } else if EXTS_FOR_7ZIP.iter().any(|ext| file.name.ends_with(ext)) {
        "7zip".to_owned()
    } else if EXTS_FOR_ZIP.iter().any(|ext| file.name.ends_with(ext)) {
        "zip".to_owned()
    } else {
        "tarball".to_owned()
    }
}

/// Infer the label to curl|sh a script
fn infer_label_for_script(file: &File) -> String {
    if file.name.ends_with(EXT_SCRIPT_POWERSHELL) {
        "powershell".to_owned()
    } else if file.name.ends_with(EXT_SCRIPT_SHELL) {
        "shell".to_owned()
    } else {
        Utf8PathBuf::from(&file.name)
            .extension()
            .expect("we determined a file was a script based on its extension, but it had none?")
            .to_owned()
    }
}
