use oranda::config::artifacts::Artifacts;
use oranda::config::theme::Theme;
use oranda::config::Config;

use linked_hash_map::LinkedHashMap;

pub fn no_artifacts() -> Config {
    Config {
        description: String::from("you axolotl questions"),
        readme_path: String::from("./src/site/fixtures/readme.md"),
        additional_pages: Some(vec![String::from("./src/site/fixtures/readme.md")]),
        additional_css: vec![String::from("./src/site/fixtures/additional.css")],
        theme: Theme::Dark,
        ..Default::default()
    }
}

pub fn cargo_dist() -> Config {
    Config {
        artifacts: Some(Artifacts {
            cargo_dist: Some(true),
            package_managers: None,
        }),
        repository: Some(String::from("https://github.com/axodotdev/oranda")),
        version: Some(String::from("0.0.1-prerelease1")),
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
        version: Some(String::from("0.0.1-prerelease1")),
        ..Default::default()
    }
}
