mod fixtures;
use super::utils::tokio_utils::TEST_RUNTIME;
use camino::Utf8Path;
use fixtures::project_config;

use oranda::config::axoproject::AxoprojectLayer;

use assert_fs::fixture::{FileWriteStr, PathChild};

#[test]
fn it_detects_a_js_project() {
    let tempdir = assert_fs::TempDir::new().expect("failed creating tempdir");
    let temppath = Utf8Path::from_path(tempdir.path()).expect("non-utf8 temp path");
    let package_json = tempdir.child("package.json");
    package_json
        .write_str(project_config::package_json())
        .expect("failed to write package_json");

    let ws = AxoprojectLayer::get_best_workspace(temppath).unwrap();
    assert_eq!(ws.kind, axoproject::WorkspaceKind::Javascript);
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

    let config = AxoprojectLayer::load(Some(tempdir.path().to_path_buf()))
        .expect("failed to load package.json")
        .unwrap();
    let project = config.project.unwrap();

    assert_eq!(project.name, Some("axo".to_owned()));
    assert_eq!(project.description, Some(">o_o<".to_owned()));
    assert_eq!(project.homepage, None);
    tempdir
        .close()
        .expect("could not successfully delete temporary directory");
}

#[test]
fn it_detects_a_rust_project() {
    let tempdir = assert_fs::TempDir::new().expect("failed creating tempdir");
    let temppath = Utf8Path::from_path(tempdir.path()).expect("non-utf8 temp path");
    let cargo_toml = tempdir.child("Cargo.toml");
    cargo_toml
        .write_str(project_config::cargo_toml())
        .expect("failed to write cargo toml");
    let main = tempdir.child("src/main.rs");
    main.write_str(project_config::main_rs())
        .expect("failed to write main.rs");
    let ws = AxoprojectLayer::get_best_workspace(temppath).unwrap();
    assert_eq!(ws.kind, axoproject::WorkspaceKind::Rust);
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
    let config = AxoprojectLayer::load(Some(tempdir.path().to_path_buf()))
        .expect("failed to load Cargo.toml")
        .unwrap();
    let project = config.project.unwrap();

    assert_eq!(project.name, Some("axo".to_owned()));
    assert_eq!(project.description, Some("blublublub".to_owned()));
    assert_eq!(project.version, Some("0.0.0".to_string()));
    tempdir
        .close()
        .expect("could not successfully delete temporary directory");
}

#[test]
fn it_can_successfully_not_detect_a_project() {
    let tempdir = assert_fs::TempDir::new().expect("failed creating tempdir");
    let temppath = Utf8Path::from_path(tempdir.path()).expect("non-utf8 temp path");

    assert!(AxoprojectLayer::get_best_workspace(temppath).is_none());
    tempdir
        .close()
        .expect("could not successfully delete temporary directory");
}
