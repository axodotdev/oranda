mod fixtures;
use super::utils::tokio_utils::TEST_RUNTIME;
use fixtures::project_config;

use oranda::config::project::ProjectConfig;

use assert_fs::fixture::{FileWriteStr, PathChild};

#[test]
fn it_detects_a_js_project() {
    let tempdir = assert_fs::TempDir::new().expect("failed creating tempdir");
    let package_json = tempdir.child("package.json");
    package_json
        .write_str(project_config::package_json())
        .expect("failed to write package_json");

    assert_eq!(
        ProjectConfig::get_project(&Some(tempdir.path().to_path_buf()))
            .unwrap()
            .kind,
        axoproject::WorkspaceKind::Javascript
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
    let main = tempdir.child("src/main.rs");
    main.write_str(project_config::main_rs())
        .expect("failed to write main.rs");
    assert_eq!(
        ProjectConfig::get_project(&Some(tempdir.path().to_path_buf()))
            .unwrap()
            .kind,
        axoproject::WorkspaceKind::Rust
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
    let main = tempdir.child("src/main.rs");
    main.write_str(project_config::main_rs())
        .expect("failed to write main.rs");
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
