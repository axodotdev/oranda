use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::Deserialize;

use super::{ApplyLayer, ApplyOptExt};

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

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct Artifacts {
    #[serde(default)]
    pub cargo_dist: Option<bool>,
    #[serde(default)]
    pub package_managers: Option<IndexMap<String, String>>,
}

impl ApplyLayer for Artifacts {
    fn apply_layer(&mut self, layer: Self) {
        self.cargo_dist.apply_opt(layer.cargo_dist);
        // FIXME: should this get merged with e.g. `extend?`
        self.package_managers.apply_opt(layer.package_managers);
    }
}

impl Artifacts {
    pub fn has_some(&self) -> bool {
        self.cargo_dist() || self.package_managers.is_some()
    }

    pub fn cargo_dist(&self) -> bool {
        self.cargo_dist.unwrap_or(false)
    }
}
