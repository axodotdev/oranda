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
    pub cargo_dist: bool,
    #[serde(default)]
    pub package_managers: Option<LinkedHashMap<String, String>>,
}

impl Artifacts {
    pub fn has_some(&self) -> bool {
        self.cargo_dist || self.package_managers.is_some()
    }
}
