use std::collections::HashMap;

use indexmap::IndexMap;

use oranda::config::oranda_config::{
    artifacts::PackageManagersConfig, AnalyticsConfig, ArtifactsConfig, BuildConfig, StyleConfig,
};
use oranda::config::Config;
use oranda::site::javascript::analytics::Plausible;

pub fn no_artifacts(temp_dir: String) -> Config {
    let mut additional_pages = HashMap::new();
    additional_pages.insert(
        "Another Page".to_string(),
        "https://raw.githubusercontent.com/axodotdev/oranda/main/README.md".to_string(),
    );
    Config {
        build: BuildConfig::new(Some(temp_dir), None, None),
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
        mdbook: None,
        funding: None,
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
        build: BuildConfig::new(Some(temp_dir), None, None),
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
        build: BuildConfig::new(Some(temp_dir), None, Some(String::from("axo"))),
        artifacts: ArtifactsConfig {
            cargo_dist: Some(true),
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

fn build_package_managers() -> PackageManagersConfig {
    let mut preferred = IndexMap::new();
    preferred.insert(String::from("npm"), String::from("npm install oranda"));
    preferred.insert(String::from("yarn"), String::from("yarn add oranda"));
    let mut additional = IndexMap::new();
    additional.insert(String::from("cargo"), String::from("cargo install oranda"));
    additional.insert(
        String::from("binstall"),
        String::from("cargo binstall oranda"),
    );
    PackageManagersConfig {
        preferred: Some(preferred),
        additional: Some(additional),
    }
}

pub fn path_prefix_with_package_managers(temp_dir: String) -> Config {
    Config {
        build: BuildConfig::new(Some(temp_dir), None, Some(String::from("axo"))),
        artifacts: ArtifactsConfig {
            cargo_dist: Some(false),
            package_managers: Some(build_package_managers()),
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
        build: BuildConfig::new(Some(temp_dir), None, None),
        artifacts: ArtifactsConfig {
            cargo_dist: Some(true),
            package_managers: None,
        },
        repository: Some(String::from("https://github.com/axodotdev/oranda")),
        ..Default::default()
    }
}

pub fn package_managers(temp_dir: String) -> Config {
    Config {
        build: BuildConfig::new(Some(temp_dir), None, None),
        artifacts: ArtifactsConfig {
            cargo_dist: Some(false),
            package_managers: Some(build_package_managers()),
        },
        repository: Some(String::from("https://github.com/axodotdev/oranda")),
        ..Default::default()
    }
}

pub fn changelog(temp_dir: String) -> Config {
    Config {
        build: BuildConfig::new(Some(temp_dir), None, None),
        repository: Some(String::from("https://github.com/axodotdev/oranda")),
        changelog: true,
        ..Default::default()
    }
}

pub fn analytics_plausible(temp_dir: String) -> Config {
    Config {
        build: BuildConfig::new(Some(temp_dir), None, None),
        repository: Some(String::from("https://github.com/axodotdev/oranda")),
        analytics: Some(AnalyticsConfig::Plausible(Plausible {
            domain: "opensource.axo.dev".into(),
            script_url: None,
        })),
        ..Default::default()
    }
}
