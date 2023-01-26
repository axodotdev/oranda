use std::fs;

use oranda::config::project::{JavaScript, ProjectConfig, Rust, Type};
use oranda::config::theme::Theme;
use oranda::config::Config;
use oranda::site::Site;

use assert_fs::fixture::{FileWriteStr, PathChild};

lazy_static::lazy_static! {
   pub static ref TEST_RUNTIME: tokio::runtime::Runtime = {
        tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .max_blocking_threads(128)
        .enable_all()
        .build()
        .expect("Initializing tokio runtime failed")
    };
}

fn config() -> Config {
    Config {
        description: String::from("you axolotl questions"),
        readme_path: String::from("./src/site/fixtures/readme.md"),
        additional_pages: Some(vec![String::from("./src/site/fixtures/readme.md")]),
        additional_css: vec![String::from("./src/site/fixtures/additional.css")],
        theme: Theme::Dark,
        ..Default::default()
    }
}

#[test]
fn it_detects_a_js_project() {
    let tempdir = assert_fs::TempDir::new().expect("failed creating tempdir");
    let package_json = tempdir.child("package.json");
    package_json
        .write_str(
            r#"
{
    "name": "axo",
    "description": ">o_o<"
}
    "#,
        )
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
        .write_str(
            r#"
{
    "name": "axo",
    "description": ">o_o<"
}
    "#,
        )
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
        .write_str(
            r#"
[package]
name = "axo"
description = ">o_o<"
    "#,
        )
        .expect("failed to write package_json");

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
        .write_str(
            r#"
[package]
name = "axo"
description = ">o_o<"
    "#,
        )
        .expect("failed to write package_json");
    let config = ProjectConfig::load(Some(tempdir.path().to_path_buf()))
        .expect("failed to load Cargo.toml")
        .unwrap();

    assert_eq!(config.name, "axo");
    tempdir
        .close()
        .expect("could not successfully delete temporary directory");
}

#[test]
fn it_adds_additional_css() {
    let _guard = TEST_RUNTIME.enter();
    let site = Site::build(&config(), &config().readme_path).unwrap();
    assert!(site
        .html
        .contains("<link href=\"custom.css\" rel=\"stylesheet\"/>"));
}

#[test]
fn it_creates_additional_css_file() {
    let _guard = TEST_RUNTIME.enter();
    Site::build(&config(), &config().readme_path).unwrap();
    let custom = fs::read_to_string("public/custom.css").unwrap();
    assert!(custom.eq("/* ./src/site/fixtures/additional.css */body{background:red;}"))
}

#[test]
fn it_builds_the_site() {
    let _guard = TEST_RUNTIME.enter();
    let site = Site::build(&config(), &config().readme_path).unwrap();
    assert!(site.html.contains("<h1>axo</h1>"));
    assert!(site.html.contains("custom.css"));
}

#[test]
fn reads_description() {
    let _guard = TEST_RUNTIME.enter();
    let site = Site::build(&config(), &config().readme_path).unwrap();
    assert!(site.html.contains("you axolotl questions"));
    assert!(site.html.contains("My Axo project"))
}

#[test]
fn reads_theme() {
    let _guard = TEST_RUNTIME.enter();
    let site = Site::build(&config(), &config().readme_path).unwrap();
    assert!(site.html.contains("html class=\"dark\""));
}

#[test]
fn creates_nav() {
    let _guard = TEST_RUNTIME.enter();
    let site = Site::build(&config(), &config().readme_path).unwrap();

    assert!(site.html.contains("<nav class=\"nav\"><ul><li><a href=\"/\">Home</a></li><li><a href=\"/readme.html\">readme</a></li></ul></nav>"));
}
