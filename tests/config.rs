mod fixtures;
use fixtures::{project_config, tokio_utils::TEST_RUNTIME};

use oranda::config::project::{JavaScript, ProjectConfig, Rust, Type};

use assert_fs::fixture::{FileWriteStr, PathChild};

#[test]
fn it_detects_a_js_project() {
    let tempdir = assert_fs::TempDir::new().expect("failed creating tempdir");
    let package_json = tempdir.child("package.json");
    package_json
        .write_str(project_config::package_json())
        .expect("failed to write package_json");

    assert_eq!(
        ProjectConfig::detect(&Some(tempdir.path().to_path_buf())),
        Some(Type::JavaScript(JavaScript {}))
    );
    tempdir
        .close()
        .expect("could not successfully delete temporary directory");
}

#[test]
fn it_loads_a_js_project_config() {
    let _guard = TEST_RUNTIME.enter();
    let tempdir = assert_fs::TempDir::new().expect("failed creating tempdir");
    let package_json = tempdir.child("package.json");
    package_json
        .write_str(project_config::package_json())
        .expect("failed to write package_json");

    let config = ProjectConfig::load(Some(tempdir.path().to_path_buf()))
        .expect("failed to load package.json")
        .unwrap();

    assert_eq!(config.name, "axo");
    assert_eq!(config.description, ">o_o<");
    assert_eq!(config.homepage, None);
    tempdir
        .close()
        .expect("could not successfully delete temporary directory");
}

#[test]
fn it_detects_a_rust_project() {
    let tempdir = assert_fs::TempDir::new().expect("failed creating tempdir");
    let cargo_toml = tempdir.child("Cargo.toml");
    cargo_toml
        .write_str(project_config::cargo_toml())
        .expect("failed to write cargo toml");

    assert_eq!(
        ProjectConfig::detect(&Some(tempdir.path().to_path_buf())),
        Some(Type::Rust(Rust {}))
    );
    tempdir
        .close()
        .expect("could not successfully delete temporary directory");
}

#[test]
fn it_loads_a_rust_project_config() {
    let _guard = TEST_RUNTIME.enter();
    let tempdir = assert_fs::TempDir::new().expect("failed creating tempdir");
    let cargo_toml = tempdir.child("Cargo.toml");
    cargo_toml
        .write_str(project_config::cargo_toml())
        .expect("failed to write cargo toml");
    let config = ProjectConfig::load(Some(tempdir.path().to_path_buf()))
        .expect("failed to load Cargo.toml")
        .unwrap();

    assert_eq!(config.name, "axo");
    assert_eq!(config.description, "blublublub");
    assert_eq!(config.version, Some("0.0.0".to_string()));
    tempdir
        .close()
        .expect("could not successfully delete temporary directory");
}
