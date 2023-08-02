use indexmap::IndexMap;

use oranda::config::{
    AnalyticsConfig, ArtifactsConfig, BuildConfig, ComponentConfig, Config, MarketingConfig,
    PackageManagersConfig, ProjectConfig, StyleConfig,
};
use oranda::site::javascript::analytics::Plausible;
use oranda::site::oranda_theme::OrandaTheme;

pub fn no_artifacts(temp_dir: String) -> Config {
    let mut additional_pages = IndexMap::new();
    additional_pages.insert("Another Page".to_string(), "docs/src/cli.md".to_string());
    additional_pages.insert("A New Page".to_string(), "SECURITY.md".to_string());
    Config {
        project: ProjectConfig {
            description: Some(String::from("you axolotl questions")),
            readme_path: String::from("README.md"),
            ..Default::default()
        },
        build: BuildConfig {
            dist_dir: temp_dir,
            additional_pages,
            ..Default::default()
        },
        styles: StyleConfig {
            additional_css: vec![String::from(
                "https://raw.githubusercontent.com/axodotdev/axii/main/css/main.css",
            )],
            ..Default::default()
        },
        components: ComponentConfig {
            mdbook: None,
            funding: None,
            artifacts: Some(ArtifactsConfig {
                cargo_dist: false,
                package_managers: PackageManagersConfig::default(),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn with_warning(temp_dir: String) -> Config {
    let mut config = no_artifacts(temp_dir);
    config.project.readme_path = "tests/build/fixtures/readme_with_warning.md".to_string();
    config
}

pub fn with_theme(temp_dir: String) -> Config {
    let mut config = no_artifacts(temp_dir);
    config.styles.theme = OrandaTheme::Cupcake;
    config
}

pub fn pinned_css(temp_dir: String) -> Config {
    let mut additional_pages = IndexMap::new();
    additional_pages.insert(
        "Another Page".to_string(),
        "https://raw.githubusercontent.com/axodotdev/oranda/main/README.md".to_string(),
    );
    Config {
        project: ProjectConfig {
            description: Some(String::from("you axolotl questions")),
            readme_path: String::from("README.md"),
            ..Default::default()
        },
        build: BuildConfig {
            dist_dir: temp_dir,
            additional_pages,
            ..Default::default()
        },
        styles: StyleConfig {
            additional_css: vec![String::from(
                "https://raw.githubusercontent.com/axodotdev/axii/main/css/main.css",
            )],
            oranda_css_version: "v0.1.0".to_string(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn path_prefix(temp_dir: String) -> Config {
    Config {
        project: ProjectConfig {
            repository: Some(String::from("https://github.com/axodotdev/oranda")),
            version: Some(String::from("0.0.1-prerelease2")),
            ..Default::default()
        },
        build: BuildConfig {
            dist_dir: temp_dir,
            path_prefix: Some(String::from("axo")),
            ..Default::default()
        },
        components: ComponentConfig {
            artifacts: Some(ArtifactsConfig {
                cargo_dist: true,
                ..Default::default()
            }),
            ..Default::default()
        },
        styles: StyleConfig {
            additional_css: vec![String::from(
                "https://raw.githubusercontent.com/axodotdev/axii/main/css/main.css",
            )],
            ..Default::default()
        },
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
        preferred,
        additional,
    }
}

pub fn path_prefix_with_package_managers(temp_dir: String) -> Config {
    Config {
        project: ProjectConfig {
            repository: Some(String::from("https://github.com/axodotdev/oranda")),
            version: Some(String::from("0.0.1-prerelease2")),
            ..Default::default()
        },
        build: BuildConfig {
            dist_dir: temp_dir,
            path_prefix: Some(String::from("axo")),
            ..Default::default()
        },
        components: ComponentConfig {
            artifacts: Some(ArtifactsConfig {
                cargo_dist: false,
                package_managers: build_package_managers(),
                ..Default::default()
            }),
            ..Default::default()
        },
        styles: StyleConfig {
            additional_css: vec![String::from(
                "https://raw.githubusercontent.com/axodotdev/axii/main/css/main.css",
            )],
            ..Default::default()
        },

        ..Default::default()
    }
}

pub fn cargo_dist(temp_dir: String) -> Config {
    Config {
        project: ProjectConfig {
            repository: Some(String::from("https://github.com/axodotdev/oranda")),
            ..Default::default()
        },
        build: BuildConfig {
            dist_dir: temp_dir,
            ..Default::default()
        },
        components: ComponentConfig {
            artifacts: Some(ArtifactsConfig {
                cargo_dist: true,
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn package_managers(temp_dir: String) -> Config {
    Config {
        project: ProjectConfig {
            repository: Some(String::from("https://github.com/axodotdev/oranda")),
            ..Default::default()
        },
        build: BuildConfig {
            dist_dir: temp_dir,
            ..Default::default()
        },
        components: ComponentConfig {
            artifacts: Some(ArtifactsConfig {
                package_managers: build_package_managers(),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn changelog(temp_dir: String) -> Config {
    Config {
        project: ProjectConfig {
            repository: Some(String::from("https://github.com/axodotdev/oranda")),
            ..Default::default()
        },
        build: BuildConfig {
            dist_dir: temp_dir,
            ..Default::default()
        },
        components: ComponentConfig {
            changelog: true,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn analytics_plausible(temp_dir: String) -> Config {
    Config {
        project: ProjectConfig {
            repository: Some(String::from("https://github.com/axodotdev/oranda")),
            ..Default::default()
        },
        build: BuildConfig {
            dist_dir: temp_dir,
            ..Default::default()
        },
        marketing: MarketingConfig {
            analytics: Some(AnalyticsConfig::Plausible(Plausible {
                domain: "opensource.axo.dev".into(),
                script_url: None,
            })),
            ..Default::default()
        },
        ..Default::default()
    }
}
