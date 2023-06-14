//! Logic for computing the artifacts/installers that each Release contains
//!
//! The main type is [`ReleaseArtifacts`][].
//!
//! Data is added to this type with:
//!
//! * [`ReleaseArtifacts::add_github`][] (in different file)
//! * [`ReleaseArtifacts::add_cargo_dist`][] (in a different file)
//! * [`ReleaseArtifacts::add_inference`][] (in a different file)
//! * [`ReleaseArtifacts::add_package_managers`][]
//!
//! One you've added all the data you want, call [`ReleaseArtifacts::select_installers`][]
//! to compute the final result, which is stored in [`ReleaseArtifacts::targets`][].

use std::collections::{BTreeMap, HashMap};

use indexmap::IndexMap;

use crate::config::artifacts::Artifacts;

use super::artifact_inference::KNOWN_TARGET_TRIPLES;

/// A Target Triple like x86_64-pc-windows-msvc
pub type TargetTriple = String;
/// Borrowed TargetTriple
pub type Targ = str;
/// The name of an application
pub type AppName = String;
/// The name of a file
pub type FileName = String;

/// Info about the artifacts of a Release
#[derive(Debug, Default, Clone)]
pub struct ReleaseArtifacts {
    /// An app to focus on. This enables multi-tenant oranda to filter out
    /// files for other apps in the same Github Release / directory.
    pub(crate) app_name: Option<String>,
    /// Files found in the Release
    files: IndexMap<FileName, File>,
    /// Potential installation methods found in the Release
    installers: Vec<Installer>,
    /// What installers to use for each target, in descending order
    /// (so recommend the first one, potentially show the others in tabs)
    targets: BTreeMap<TargetTriple, Vec<InstallIdx>>,
}

/// A handle to a File (equivalent to a pointer into [`ReleaseArtifacts::files`][])
#[derive(Debug, Copy, Clone)]
pub struct FileIdx(usize);

/// A File we found in the Release
#[derive(Debug, Clone)]
pub struct File {
    /// The name of the file
    pub name: FileName,
    /// The URL it can be downloaded from
    pub download_url: String,
    /// Whether artifact_inference should process this file
    ///
    /// Starts true, but can be set to false by other steps to avoid suggesting an installer twice
    pub infer: bool,
}

/// A handle to an Installer (equivalent to a pointer into [`ReleaseArtifactsInstallers`][])
#[derive(Debug, Copy, Clone)]
pub struct InstallIdx(usize);

/// A potential installer / installation method for this release
#[derive(Debug, Clone)]
pub struct Installer {
    /// A label to give
    pub label: String,
    /// What targets this supports, and how this installer should be preferred on that target
    pub targets: HashMap<TargetTriple, InstallerPreference>,
    /// The way we should suggest this installer
    pub method: InstallMethod,
}

/// How much an installer should be preferred (descending order)
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InstallerPreference {
    /// The best way
    Preferred,
    /// Some kind of "native" installer like a .msi or .dmg
    Native,
    /// Some kind of curl|sh script
    Script,
    /// Any kind of custom/misc/unknown solution
    Custom,
    /// Just a tarball containing the binary
    Archive,
}

/// Different methods of installation recommendation
#[derive(Debug, Clone)]
pub enum InstallMethod {
    /// Download this file
    Download {
        /// The file
        file: FileIdx,
    },
    /// Run this line in your terminal
    Run {
        /// Source for the script
        file: Option<FileIdx>,
        /// Command to copy-paste
        run_hint: String,
    },
}

impl ReleaseArtifacts {
    /// Create a new set of artifacts for a Release
    ///
    /// Optionally filtered down to the given app (for multi-tenant stuff)
    pub fn new(app_name: Option<AppName>) -> Self {
        Self {
            app_name,
            ..Self::default()
        }
    }

    /// Add a file to the list
    pub fn add_file(&mut self, file: File) -> FileIdx {
        let idx = FileIdx(self.files.len());
        let old = self.files.insert(file.name.clone(), file);
        assert!(
            old.is_none(),
            "release had two files with the same name ({})??",
            &self.files[idx.0].name
        );
        idx
    }

    /// Add an installer to the list
    pub fn add_installer(&mut self, installer: Installer) -> InstallIdx {
        let idx = InstallIdx(self.installers.len());
        self.installers.push(installer);
        idx
    }

    /// Get a file
    pub fn file(&self, idx: FileIdx) -> &File {
        self.files.get_index(idx.0).unwrap().1
    }
    /// Get a mutable file
    pub fn file_mut(&mut self, idx: FileIdx) -> &mut File {
        self.files.get_index_mut(idx.0).unwrap().1
    }
    /// Get the handle to a file, given the name
    pub fn file_idx(&self, name: &FileName) -> Option<FileIdx> {
        self.files.get_index_of(name).map(FileIdx)
    }
    /// Get all the handles to files
    pub fn file_indices(&self) -> impl Iterator<Item = FileIdx> {
        (0..self.files.len()).map(FileIdx)
    }
    /// Get an installer
    pub fn installer(&self, idx: InstallIdx) -> &Installer {
        &self.installers[idx.0]
    }
    /// Get all installers
    pub fn installers(&self) -> impl Iterator<Item = (InstallIdx, &Installer)> {
        self.installers
            .iter()
            .enumerate()
            .map(|(idx, ins)| (InstallIdx(idx), ins))
    }
    /// Get all target -> installer mappings
    pub fn targets(&self) -> &BTreeMap<TargetTriple, Vec<InstallIdx>> {
        &self.targets
    }

    /// Add custom package manager values from the config
    pub fn add_package_managers(&mut self, config: &Artifacts) {
        if let Some(package_managers) = &config.package_managers {
            for (label, script) in package_managers {
                let run_hint = format!("```shell\n{script}\n```");
                let installer = Installer {
                    label: label.to_owned(),
                    targets: preference_to_targets(vec![], InstallerPreference::Custom),
                    method: InstallMethod::Run {
                        file: None,
                        run_hint,
                    },
                };
                self.add_installer(installer);
            }
        }
    }

    /// Now that we've added all the data sources, select
    pub fn select_installers(&mut self) {
        for target in KNOWN_TARGET_TRIPLES.iter().copied().flatten().copied() {
            let mut installers = vec![];
            for (idx, installer) in self.installers() {
                if let Some(preference) = installer.targets.get(target) {
                    installers.push((idx, preference));
                }
            }
            installers.sort_by(|(idx_a, pref_a), (idx_b, pref_b)| {
                let installer_a = self.installer(*idx_a);
                let installer_b = self.installer(*idx_b);

                pref_a
                    .cmp(pref_b)
                    .then_with(|| installer_a.label.cmp(&installer_b.label))
            });

            let installers = installers.into_iter().map(|(i, _pref)| i).collect();
            self.targets.insert(target.to_owned(), installers);
        }
    }
}

/// Take an installer preference an uniformly apply it to every given TargetTriple
///
/// If the array is empty, we take this to mean "for all possible target triples"
pub fn preference_to_targets(
    targets: Vec<TargetTriple>,
    preference: InstallerPreference,
) -> HashMap<TargetTriple, InstallerPreference> {
    let targets = if targets.is_empty() {
        KNOWN_TARGET_TRIPLES
            .iter()
            .copied()
            .flatten()
            .copied()
            .map(|t| t.to_owned())
            .collect()
    } else {
        targets
    };

    targets.into_iter().map(|t| (t, preference)).collect()
}
