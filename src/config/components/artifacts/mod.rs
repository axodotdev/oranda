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
#[derive(Debug, Clone)]
pub struct ArtifactsConfig {
    pub auto: bool,
    pub cargo_dist: bool,
    pub package_managers: PackageManagersConfig,
    pub hidden: Vec<String>,
}

/// Setting for downloadable artifacts, installers, and package-managers
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct ArtifactsLayer {
    /// Whether to enable auto-detection of artifacts/installers in your Github Releases
    ///
    /// This allows us to look at the assets listed in a Github Release and use their
    /// names to guess their platforms and what they're for.
    ///
    /// If `cargo_dist` is also enabled, that data source will be preferred over this one
    /// for the assets that cargo-dist knows about, as it should presumably be more reliable.
    ///
    /// Artifact auto-detection currently includes the following concepts:
    ///
    /// * **platforms**: if an artifact name contains a [rust target triple][triple] then we will
    ///   assume it's specific to that platform. Otherwise we will infer platform using extension
    ///   (with rules like "ps1 scripts are for windows, sh scripts are for unix").
    /// * **archives**: if a platform-specific artifact name ends with an archive format
    ///   (.tar.*, .zip .7z, .rar...) then we will assume it contains the binaries
    ///    for that platform and recommend its download accordingly.
    /// * **scripts**: if an artifact name contains "install" and ends with ".sh" or ".ps1"
    ///   we will assume it's an install script and generate an appropriate `curl | sh` for
    ///   it (`irm | iex` for ps1)
    /// * **bundles**: if an artifact name ends with a known format for some kind of
    ///   installer/bundle we will recommend its download at a higher priority, assuming
    ///   this is a very good way to install your app. This includes ".msi", ".app", ".dmg",
    ///   ".deb", ".rpm", ".pkg.tar.*", ".flatpak", and ".snap".
    ///
    /// [triple]: https://doc.rust-lang.org/nightly/rustc/platform-support.html
    pub auto: Option<bool>,
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
            auto: false,
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
            auto,
            cargo_dist,
            package_managers,
            hidden,
        } = layer;

        self.auto.apply_val(auto);
        self.cargo_dist.apply_val(cargo_dist);
        self.package_managers.apply_val_layer(package_managers);
        // In the future this might want to be `extend`
        self.hidden.apply_val(hidden);
    }
}

impl ArtifactsConfig {
    pub fn has_some(&self) -> bool {
        self.cargo_dist || self.auto || !self.package_managers.is_empty()
    }
}
