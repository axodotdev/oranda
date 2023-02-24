use oranda::config::artifacts::Artifacts;
use oranda::config::theme::Theme;
use oranda::config::Config;

use linked_hash_map::LinkedHashMap;

pub fn no_artifacts() -> Config {
    Config {
        description: String::from("you axolotl questions"),
        readme_path: String::from(
            "https://raw.githubusercontent.com/axodotdev/oranda/main/README.md",
        ),
        additional_pages: Some(vec![String::from(
            "https://raw.githubusercontent.com/axodotdev/oranda/main/README.md",
        )]),
        additional_css: vec![String::from(
            "https://raw.githubusercontent.com/axodotdev/axii/main/css/main.css",
        )],
        theme: Theme::Dark,
        ..Default::default()
    }
}

pub fn path_prefix() -> Config {
    Config {
        path_prefix: Some(String::from("axo")),
        artifacts: Some(Artifacts {
            cargo_dist: Some(true),
            package_managers: None,
        }),
        additional_css: vec![String::from(
            "https://raw.githubusercontent.com/axodotdev/axii/main/css/main.css",
        )],
        repository: Some(String::from("https://github.com/axodotdev/oranda")),
        version: Some(String::from("0.0.1-prerelease2")),
        ..Default::default()
    }
}

pub fn cargo_dist() -> Config {
    Config {
        artifacts: Some(Artifacts {
            cargo_dist: Some(true),
            package_managers: None,
        }),
        additional_pages: Some(vec![String::from(
            "https://raw.githubusercontent.com/axodotdev/oranda/main/README.md",
        )]),
        repository: Some(String::from("https://github.com/axodotdev/oranda")),
        version: Some(String::from("0.0.1-prerelease2")),
        ..Default::default()
    }
}

pub fn package_managers() -> Config {
    let mut package_managers = LinkedHashMap::new();
    package_managers.insert(String::from("npm"), String::from("npm install oranda"));
    package_managers.insert(String::from("yarn"), String::from("yarn add oranda"));
    Config {
        artifacts: Some(Artifacts {
            cargo_dist: None,
            package_managers: Some(package_managers),
        }),
        repository: Some(String::from("https://github.com/axodotdev/oranda")),
        version: Some(String::from("0.0.1-prerelease2")),
        ..Default::default()
    }
}
