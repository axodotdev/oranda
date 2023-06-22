use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyValExt};

mod package_managers;
pub use package_managers::{PackageManagersConfig, PackageManagersLayer};

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
enum ArtifactSystem {
    Windows,
    Windows64,
    WindowsArm,

    Mac,
    MacPpc,
    Mac32,
    MacSilicon,

    Linux,
    LinuxUbuntu,
    LinuxDebian,
    LinuxMandriva,
    LinuxRedhat,
    LinuxFedora,
    LinuxSuse,
    LinuxGentoo,

    Ios,
    Android,

    Freebsd,
}

/// Info about downloadable artifacts / installers / package-managers (complete version)
#[derive(Debug)]
pub struct ArtifactsConfig {
    pub cargo_dist: bool,
    pub package_managers: PackageManagersConfig,
    pub hidden: Vec<String>,
}

/// Setting for downloadable artifacts, installers, and package-managers
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct ArtifactsLayer {
    /// Whether to enable cargo-dist integration
    ///
    /// If enabled, we will check every GitHub Release of your project for a dist-manifest.json
    /// and use that as the authoritative source of information on the artifacts uploaded to your
    /// Github Release.
    ///
    /// This integration only works if `project.repository` points to a GitHub repo (see that field's
    /// docs for details).
    ///
    /// We default this to true if we find `[workspace.metadata.dist]` in your Cargo.toml
    pub cargo_dist: Option<bool>,
    /// Snippets saying how to install your project using various package-managers
    ///
    /// These are grouped into "preferred" and "additional"
    ///
    /// - "additional" packages only show up on the install page
    /// - "preferred" packages show up as options in the install widget
    ///
    /// Both are ordered maps of "label": "one-liner script"
    ///
    /// For example:
    ///
    /// ```json
    /// {
    ///   "components": {
    ///     "artifacts": {
    ///       "package_managers": {
    ///         "preferred": {
    ///           "npm": "npm -i @axodotdev/axolotlsay",
    ///           "cargo": "cargo install axolotlsay"
    ///         },
    ///         "additional": {
    ///           "binstall": "cargo binstall axolotlsay"
    ///         }
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// If a package_manager has the same name as an auto-detected installer,
    /// it will overwrite the auto-detected result, allowing you to specify something
    /// more preferrable (relevant to cargo-dist npm packages).
    pub package_managers: Option<PackageManagersLayer>,
    /// Artifact/installer listings to supress from the install widget's tabs
    /// and the install page's package-manager/script listings.
    ///
    /// Currently this won't supress them from the install page's "downloads" table
    ///
    /// Example (hide auto-detect shell scripts): `"hidden": ["shell", "powershell"]`
    pub hidden: Option<Vec<String>>,
}

impl Default for ArtifactsConfig {
    fn default() -> Self {
        ArtifactsConfig {
            cargo_dist: false,
            package_managers: PackageManagersConfig::default(),
            hidden: vec![],
        }
    }
}
impl ApplyLayer for ArtifactsConfig {
    type Layer = ArtifactsLayer;
    fn apply_layer(&mut self, layer: Self::Layer) {
        // This is intentionally written slightly cumbersome to make you update this
        let ArtifactsLayer {
            cargo_dist,
            package_managers,
            hidden,
        } = layer;
        self.cargo_dist.apply_val(cargo_dist);
        self.package_managers.apply_val_layer(package_managers);
        // In the future this might want to be `extend`
        self.hidden.apply_val(hidden);
    }
}

impl ArtifactsConfig {
    pub fn has_some(&self) -> bool {
        self.cargo_dist || !self.package_managers.is_empty()
    }
}
