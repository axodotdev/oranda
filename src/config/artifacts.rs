use linked_hash_map::LinkedHashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Default, Deserialize)]
pub struct Artifacts {
    #[serde(default)]
    pub cargo_dist: Option<bool>,
    #[serde(default)]
    pub package_managers: Option<LinkedHashMap<String, String>>,
}

impl Artifacts {
    /// Merge this value with another layer of itself, preferring the new layer
    pub fn apply_layer(&mut self, layer: Self) {
        if let Some(val) = layer.cargo_dist {
            self.cargo_dist = Some(val);
        }
        if let Some(val) = layer.package_managers {
            // FIXME: should this get merged with e.g. `extend?`
            self.package_managers = Some(val);
        }
    }

    pub fn has_some(&self) -> bool {
        self.cargo_dist() || self.package_managers.is_some()
    }

    pub fn cargo_dist(&self) -> bool {
        self.cargo_dist.unwrap_or(false)
    }
}
