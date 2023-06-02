use std::collections::HashMap;

use oranda::config::analytics::{Analytics, PlausibleTracking};
use oranda::config::artifacts::Artifacts;
use oranda::config::{Config, StyleConfig};

use linked_hash_map::LinkedHashMap;

pub fn no_artifacts(temp_dir: String) -> Config {
    let mut additional_pages = HashMap::new();
    additional_pages.insert(
        "Another Page".to_string(),
        "https://raw.githubusercontent.com/axodotdev/oranda/main/README.md".to_string(),
    );
    Config {
        dist_dir: temp_dir,
        description: String::from("you axolotl questions"),
        readme_path: String::from(
            "https://raw.githubusercontent.com/axodotdev/oranda/main/README.md",
        ),
        additional_pages: Some(additional_pages),
        styles: StyleConfig {
            additional_css: vec![String::from(
                "https://raw.githubusercontent.com/axodotdev/axii/main/css/main.css",
            )],
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn pinned_css(temp_dir: String) -> Config {
    let mut additional_pages = HashMap::new();
    additional_pages.insert(
        "Another Page".to_string(),
        "https://raw.githubusercontent.com/axodotdev/oranda/main/README.md".to_string(),
    );
    Config {
        dist_dir: temp_dir,
        description: String::from("you axolotl questions"),
        readme_path: String::from(
            "https://raw.githubusercontent.com/axodotdev/oranda/main/README.md",
        ),
        additional_pages: Some(additional_pages),
        styles: StyleConfig {
            additional_css: vec![String::from(
                "https://raw.githubusercontent.com/axodotdev/axii/main/css/main.css",
            )],
            oranda_css_version: Some("0.0.3".to_string()),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn path_prefix(temp_dir: String) -> Config {
    Config {
        dist_dir: temp_dir,
        path_prefix: Some(String::from("axo")),
        artifacts: Artifacts {
            cargo_dist: true,
            package_managers: None,
        },
        styles: StyleConfig {
            additional_css: vec![String::from(
                "https://raw.githubusercontent.com/axodotdev/axii/main/css/main.css",
            )],
            ..Default::default()
        },
        repository: Some(String::from("https://github.com/axodotdev/oranda")),
        version: Some(String::from("0.0.1-prerelease2")),
        ..Default::default()
    }
}

pub fn cargo_dist(temp_dir: String) -> Config {
    Config {
        dist_dir: temp_dir,
        artifacts: Artifacts {
            cargo_dist: true,
            package_managers: None,
        },
        repository: Some(String::from("https://github.com/axodotdev/oranda")),
        ..Default::default()
    }
}

pub fn package_managers(temp_dir: String) -> Config {
    let mut package_managers = LinkedHashMap::new();
    package_managers.insert(String::from("npm"), String::from("npm install oranda"));
    package_managers.insert(String::from("yarn"), String::from("yarn add oranda"));
    Config {
        dist_dir: temp_dir,
        artifacts: Artifacts {
            cargo_dist: false,
            package_managers: Some(package_managers),
        },
        repository: Some(String::from("https://github.com/axodotdev/oranda")),
        ..Default::default()
    }
}

pub fn changelog(temp_dir: String) -> Config {
    Config {
        dist_dir: temp_dir,
        repository: Some(String::from("https://github.com/axodotdev/oranda")),
        changelog: true,
        ..Default::default()
    }
}

pub fn analytics_plausible(temp_dir: String) -> Config {
    Config {
        dist_dir: temp_dir,
        repository: Some(String::from("https://github.com/axodotdev/oranda")),
        analytics: Some(Analytics::Plausible(PlausibleTracking {
            domain: "opensource.axo.dev".into(),
            script_url: None,
        })),
        ..Default::default()
    }
}
