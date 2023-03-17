use std::collections::HashMap;

use oranda::config::artifacts::Artifacts;
use oranda::config::theme::Theme;
use oranda::config::Config;

use linked_hash_map::LinkedHashMap;

fn temp_build_dir() -> String {
    assert_fs::TempDir::new()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn no_artifacts() -> Config {
    let mut additional_pages = HashMap::new();
    additional_pages.insert(
        "Another Page".to_string(),
        "https://raw.githubusercontent.com/axodotdev/oranda/main/README.md".to_string(),
    );
    Config {
        dist_dir: temp_build_dir(),
        description: String::from("you axolotl questions"),
        readme_path: String::from(
            "https://raw.githubusercontent.com/axodotdev/oranda/main/README.md",
        ),
        additional_pages: Some(additional_pages),
        additional_css: vec![String::from(
            "https://raw.githubusercontent.com/axodotdev/axii/main/css/main.css",
        )],
        theme: Theme::Dark,
        ..Default::default()
    }
}

pub fn path_prefix() -> Config {
    Config {
        dist_dir: temp_build_dir(),
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
        dist_dir: temp_build_dir(),
        artifacts: Some(Artifacts {
            cargo_dist: Some(true),
            package_managers: None,
        }),
        repository: Some(String::from("https://github.com/axodotdev/oranda")),
        version: Some(String::from("0.0.1")),
        ..Default::default()
    }
}

pub fn package_managers() -> Config {
    let mut package_managers = LinkedHashMap::new();
    package_managers.insert(String::from("npm"), String::from("npm install oranda"));
    package_managers.insert(String::from("yarn"), String::from("yarn add oranda"));
    Config {
        dist_dir: temp_build_dir(),
        artifacts: Some(Artifacts {
            cargo_dist: None,
            package_managers: Some(package_managers),
        }),
        repository: Some(String::from("https://github.com/axodotdev/oranda")),
        version: Some(String::from("0.0.1-prerelease2")),
        ..Default::default()
    }
}

pub fn changelog() -> Config {
    Config {
        dist_dir: temp_build_dir(),
        artifacts: Some(Artifacts {
            cargo_dist: Some(true),
            package_managers: None,
        }),
        repository: Some(String::from("https://github.com/axodotdev/oranda")),
        version: Some(String::from("0.0.1")),
        changelog: true,
        ..Default::default()
    }
}
