mod fixtures;
use fixtures::{config, page};

use oranda::config::project::{JavaScript, ProjectConfig, Rust, Type};

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
    "version": "0.1.0",
    "description": ">o_o<",
    "repository": {
        "type": "git",
        "url": "https://github.com/axodotdev/not-a-real-project"
    }
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
    let config = &config::no_artifacts();
    let page_html = page::index(config);
    assert!(page_html.contains("<link href=\"custom.css\" rel=\"stylesheet\"/>"));
}

#[test]
fn it_builds_the_site() {
    let _guard = TEST_RUNTIME.enter();
    let config = &config::no_artifacts();
    let page_html = page::index(config);
    assert!(page_html.contains("<h1>axo</h1>"));
    assert!(page_html.contains("custom.css"));
}

#[test]
fn reads_description() {
    let _guard = TEST_RUNTIME.enter();
    let config = &config::no_artifacts();
    let page_html = page::index(config);
    assert!(page_html.contains("you axolotl questions"));
    assert!(page_html.contains("My Axo project"))
}

#[test]
fn reads_theme() {
    let _guard = TEST_RUNTIME.enter();
    let config = &config::no_artifacts();
    let page_html = page::index(config);
    assert!(page_html.contains("html class=\"dark\""));
}

#[test]
fn creates_nav() {
    let _guard = TEST_RUNTIME.enter();
    let config = &config::no_artifacts();
    let page_html = page::index(config);

    assert!(page_html.contains("<nav class=\"nav\"><ul><li><a href=\"/\">Home</a></li><li><a href=\"/README.html\">README</a></li></ul></nav>"));
}

#[test]
fn creates_footer() {
    let _guard = TEST_RUNTIME.enter();
    let config = &config::no_artifacts();
    let page_html = page::index(config);

    assert!(page_html.contains("<footer class=\"axo-gradient flex items-center justify-between px-4 py-2 text-slate-50 text-xs w-full\"><span>My Axo project</span></footer>"));
}

#[test]
fn creates_nav_item() {
    let _guard = TEST_RUNTIME.enter();
    let config = &config::cargo_dist();
    let page_html = page::index(config);
    assert!(page_html
        .contains("<a class=\"download-all\" href=\"/artifacts.html\">View all downloads</a>"));
}

#[test]
fn loads_js() {
    let _guard = TEST_RUNTIME.enter();
    let config = &config::cargo_dist();
    let page_html = page::index(config);
    assert!(page_html.contains("<script src=\"detect_os.js\"></script>"));
}

#[test]
fn creates_download_for_mac() {
    let _guard = TEST_RUNTIME.enter();
    let config = &config::cargo_dist();
    let page_html = page::index(config);
    assert!(page_html.contains("<a class=\"text-center\" href=\"https://github.com/axodotdev/oranda/releases/download/v0.0.1-prerelease1/oranda-v0.0.1-prerelease1-x86_64-apple-darwin.tar.xz\">Download v0.0.1-prerelease1</a><a class=\"download-all\" href=\"/artifacts.html\">View all downloads</a>"));
}

#[test]
fn creates_downloads_page() {
    let _guard = TEST_RUNTIME.enter();
    let config = &config::cargo_dist();
    let artifacts_page = page::artifacts(config);
    assert!(artifacts_page.contains("<h3>Downloads</h3>"));
    assert!(artifacts_page.contains("<span>oranda-v0.0.1-prerelease1-x86_64-pc-windows-msvc.zip</span><span>Executable Zip</span><span>x86_64-pc-windows-msvc</span><span><a href=\"https://github.com/axodotdev/oranda/releases/download/v0.0.1-prerelease1/oranda-v0.0.1-prerelease1-x86_64-pc-windows-msvc.zip\">Download</a></span>"));
}

#[test]
fn creates_nav_item_package_managers() {
    let _guard = TEST_RUNTIME.enter();
    let config = &config::package_managers();
    let page_html = page::index(config);
    assert!(page_html
        .contains("<a class=\"download-all\" href=\"/artifacts.html\">View all downloads</a>"));
}
