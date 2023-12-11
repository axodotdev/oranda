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
//! to compute the final result, which is stored in [`ReleaseArtifacts::installers_by_target`][].

use std::collections::{BTreeMap, HashMap};

use axoasset::{LocalAsset, RemoteAsset};
use axoproject::platforms::KNOWN_TARGET_TRIPLES;
use camino::Utf8PathBuf;
use indexmap::IndexMap;
use serde::{Serialize, Serializer};

use crate::config::ArtifactsConfig;
use crate::config::Config;
use crate::errors::*;

use inference::KNOWN_SCRIPT_EXTS;

pub mod inference;

/// A Target Triple like x86_64-pc-windows-msvc
pub type TargetTriple = String;
/// Borrowed TargetTriple
pub type Targ = str;
/// The name of an application
pub type AppName = String;
/// The name of a file
pub type FileName = String;

/// Info about the artifacts of a Release
#[derive(Debug, Default, Clone, Serialize)]
pub struct ReleaseArtifacts {
    /// An app to focus on. This enables multi-tenant oranda to filter out
    /// files for other apps in the same Github Release / directory.
    #[serde(skip)]
    pub(crate) app_name: Option<String>,
    /// Files found in the Release
    #[serde(serialize_with = "flatten_files")]
    files: IndexMap<FileName, File>,
    /// Potential installation methods found in the Release
    installers: Vec<Installer>,
    /// What installers to use for each target, in descending order
    /// (so recommend the first one, potentially show the others in tabs)
    targets: BTreeMap<TargetTriple, Vec<InstallerIdx>>,
}

/// A handle to a File (equivalent to a pointer into `ReleaseArtifacts::files`)
#[derive(Debug, Copy, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileIdx(usize);

/// A File we found in the Release
#[derive(Debug, Clone, Serialize)]
pub struct File {
    /// The name of the file
    pub name: FileName,
    /// The URL it can be downloaded from
    pub download_url: String,
    /// Link (path) to view the source of the file
    pub view_path: Option<String>,
    /// A file containing checksums for this one
    pub checksum_file: Option<FileIdx>,
    /// Whether artifact_inference should process this file
    ///
    /// Starts true, but can be set to false by other steps to avoid suggesting an installer twice
    #[serde(skip)]
    pub infer: bool,
}

/// A handle to an Installer (equivalent to a pointer into [`ReleaseArtifacts::installers`][])
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize)]
pub struct InstallerIdx(pub usize);

/// A potential installer / installation method for this release
#[derive(Debug, Clone, Serialize)]
pub struct Installer {
    /// A brief label for things identifying the installer
    pub label: String,
    /// A longer description of the installer
    pub description: String,
    /// Which application this installer belongs to. Optional because it's only needed if we have multiple
    /// similar install methods for different apps (e.g. two "sh" installer scripts)
    pub app_name: Option<String>,
    /// What targets this supports, and how this installer should be preferred on that target
    #[serde(skip)]
    pub targets: HashMap<TargetTriple, InstallerPreference>,
    /// The way we should suggest this installer
    pub method: InstallMethod,
    /// Whether this installer should be ignored by select_installers
    /// (if true, the installer is effectively deleted, but we want to keep indices stable)
    #[serde(skip)]
    pub display: DisplayPreference,
}

/// How much an installer should be preferred (descending order)
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
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

/// Where to show the installer
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum DisplayPreference {
    /// Show everywhere
    Preferred,
    /// Show only on the install page
    Additional,
    /// Hide it
    Hidden,
}

/// Different methods of installation recommendation
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
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
    pub fn add_installer(&mut self, installer: Installer) -> InstallerIdx {
        let idx = InstallerIdx(self.installers.len());
        self.installers.push(installer);
        idx
    }

    /// Get a file
    pub fn file(&self, idx: FileIdx) -> &File {
        self.files
            .get_index(idx.0)
            .expect("invalid FileIdx (did you remove an entry?)")
            .1
    }
    /// Get a mutable file
    pub fn file_mut(&mut self, idx: FileIdx) -> &mut File {
        self.files
            .get_index_mut(idx.0)
            .expect("invalid FileIdx (did you remove an entry?)")
            .1
    }
    /// Get the handle to a file, given the name
    pub fn file_idx(&self, name: &FileName) -> Option<FileIdx> {
        self.files.get_index_of(name).map(FileIdx)
    }
    /// Get all the handles to files
    pub fn file_indices(&self) -> impl Iterator<Item = FileIdx> {
        (0..self.files.len()).map(FileIdx)
    }
    /// Get all the files
    pub fn files(&self) -> impl Iterator<Item = &File> {
        self.files.values()
    }
    /// Get an installer
    pub fn installer(&self, idx: InstallerIdx) -> &Installer {
        &self.installers[idx.0]
    }
    /// Get all installers
    pub fn installers(&self) -> impl Iterator<Item = (InstallerIdx, &Installer)> {
        self.installers
            .iter()
            .enumerate()
            .map(|(idx, ins)| (InstallerIdx(idx), ins))
    }
    /// Get all target -> installer mappings
    pub fn installers_by_target(&self) -> &BTreeMap<TargetTriple, Vec<InstallerIdx>> {
        &self.targets
    }

    /// Add custom package manager values from the config
    pub fn add_package_managers(&mut self, config: &ArtifactsConfig) {
        // If we have a custom item for "npm" or "npx", then supress any entries
        // from earlier layers like cargo-dist that were also trying to specify this
        if config.package_managers.has_npm() {
            if let Some(installer) = self
                .installers
                .iter_mut()
                .find(|installer| installer.label == "npm")
            {
                installer.display = DisplayPreference::Hidden;
            }
        }
        for (label, script) in &config.package_managers.preferred {
            let mut installer = simple_run_installer(label, script);
            installer.display = DisplayPreference::Preferred;
            self.add_installer(installer);
        }

        for (label, script) in &config.package_managers.additional {
            let mut installer = simple_run_installer(label, script);
            installer.display = DisplayPreference::Additional;
            self.add_installer(installer);
        }
    }

    /// Now that we've added all the data sources, select installers for each target
    pub fn select_installers(&mut self, artifacts_config: &ArtifactsConfig) {
        // Hide anything that the user has asked for
        for installer in &mut self.installers {
            if artifacts_config.hidden.contains(&installer.label) {
                installer.display = DisplayPreference::Hidden;
            }
        }
        for target in KNOWN_TARGET_TRIPLES.iter().copied().flatten().copied() {
            // Gather up all the installers into an array
            let mut installers = vec![];
            for (idx, installer) in self.installers() {
                // Only the premo installers go here
                if installer.display != DisplayPreference::Preferred {
                    continue;
                }
                if let Some(preference) = installer.targets.get(target) {
                    installers.push((idx, preference));
                }
            }

            // Sort the array
            installers.sort_by(|(idx_a, pref_a), (idx_b, pref_b)| {
                let installer_a = self.installer(*idx_a);
                let installer_b = self.installer(*idx_b);

                pref_a
                    .cmp(pref_b)
                    .then_with(|| installer_a.label.cmp(&installer_b.label))
            });

            // If the result is non-empty, register the target as having these installers
            let installers: Vec<_> = installers.into_iter().map(|(i, _pref)| i).collect();
            if !installers.is_empty() {
                self.targets.insert(target.to_owned(), installers);
            }
        }
    }

    /// Make shell scripts viewable by copying the files to be statically hosted instead of hotlinked
    pub fn make_scripts_viewable(&mut self, config: &Config) -> Result<()> {
        for file in self.files.values_mut() {
            if KNOWN_SCRIPT_EXTS.iter().any(|ext| file.name.ends_with(ext)) {
                let path = write_source(config, file)?;
                file.view_path = Some(path);
            }
        }
        Ok(())
    }
}

/// Take an installer preference and uniformly apply it to every given TargetTriple
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

/// Serialize an IndexMap as just a flat array
fn flatten_files<S>(files: &IndexMap<FileName, File>, s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let files: Vec<_> = files.values().collect();
    files.serialize(s)
}

/// Make the source of a file available on the server
fn write_source(config: &Config, file: &File) -> Result<String> {
    let file_path = format!("{}.txt", &file.name);
    let full_file_path = Utf8PathBuf::from(&config.build.dist_dir).join(&file_path);
    if !full_file_path.exists() {
        let file_string_future = RemoteAsset::load_string(&file.download_url);
        let file_string = tokio::runtime::Handle::current().block_on(file_string_future)?;
        LocalAsset::write_new(&file_string, &full_file_path)?;
    }
    Ok(file_path)
}

fn simple_run_installer(label: &str, script: &str) -> Installer {
    let run_hint = script.to_owned();
    Installer {
        label: label.to_owned(),
        description: String::new(),
        app_name: None,
        targets: preference_to_targets(vec![], InstallerPreference::Custom),
        method: InstallMethod::Run {
            file: None,
            run_hint,
        },
        display: DisplayPreference::Preferred,
    }
}
