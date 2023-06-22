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

/// Info about downloadable artifacts / installers / package-managers (cimplete version)
#[derive(Debug)]
pub struct ArtifactsConfig {
    pub cargo_dist: bool,
    pub package_managers: PackageManagersConfig,
    pub hidden: Vec<String>,
}
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct ArtifactsLayer {
    pub cargo_dist: Option<bool>,
    pub package_managers: Option<PackageManagersLayer>,
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
