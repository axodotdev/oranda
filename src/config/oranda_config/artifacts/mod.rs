use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyOptExt};

pub use package_managers::PackageManagersConfig;

mod package_managers;

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

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ArtifactsOpts {
    #[serde(default)]
    pub cargo_dist: Option<bool>,
    #[serde(default)]
    pub package_managers: Option<PackageManagersConfig>,
}

#[derive(Debug, Default)]
pub struct ArtifactsConfig {
    pub cargo_dist: bool,
    pub package_managers: Option<PackageManagersConfig>,
}

impl ApplyLayer for ArtifactsConfig {
    fn apply_layer(&mut self, layer: Self) {
        self.cargo_dist.apply_opt(layer.cargo_dist);
        self.package_managers.apply_opt(layer.package_managers);
    }
}

impl ArtifactsConfig {
    pub fn has_some(&self) -> bool {
        self.cargo_dist() || self.package_managers.is_some()
    }
}
