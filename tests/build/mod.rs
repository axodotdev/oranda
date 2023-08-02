use assert_fs::TempDir;

mod fixtures;
use super::utils::tokio_utils::TEST_RUNTIME;
use fixtures::{oranda_config, page};

// FIXME(#528):
// Fix these tests by using a library that's actually HTML-aware so that we don't have to mess around
// with trying to detect variable levels of whitespace.

fn temp_build_dir() -> (TempDir, String) {
    let dir = assert_fs::TempDir::new().unwrap();
    let dir_str = dir.to_str().unwrap().to_string();
    (dir, dir_str)
}

#[test]
fn it_adds_additional_css() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let page = page::index(&config);
    assert!(page
        .contents
        .contains(r#"<link rel="stylesheet" href="/custom.css" />"#));
}
#[test]
fn it_can_point_to_custom_repository() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let mut config = oranda_config::cargo_dist(temp_dir);
    config.project.repository = Some("https://github.com/axodotdev/privaterepo".into());
    config.components.artifacts.as_mut().unwrap().auto = true;
    oranda::site::Site::build_single(&config, None).unwrap();
}

#[test]
fn it_renders_changelog_with_no_cargo_dist() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::changelog(temp_dir);
    let page = page::changelog(&config);
    assert!(page.contents.contains(r#"<h1>Releases</h1>"#));
}

#[test]
fn it_renders_changelog_with_release_content() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::changelog(temp_dir);
    let page = page::changelog(&config);
    assert!(page.contents.contains("Initial release."));
}

#[test]
fn it_adds_oranda_css() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let page = page::index(&config);
    let filename = "oranda.css".to_string();
    assert!(page.contents.contains(&filename));
}

#[test]
fn it_adds_oranda_css_with_pinned_version() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::pinned_css(temp_dir);
    let page = page::index(&config);
    dbg!(&page.contents);
    assert!(page
        .contents
        .contains(r#"<link rel="stylesheet" href="/oranda-v0.1.0.css" />"#));
}

#[test]
fn it_builds_the_site() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let page = page::index(&config);
    assert!(page.contents.contains(r#"axo"#));
    assert!(page.contents.contains("custom.css"));
}

#[test]
fn reads_description() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let page = page::index(&config);
    assert!(page.contents.contains("you axolotl questions"));
    assert!(page.contents.contains("My Oranda Project"))
}

#[test]
fn reads_theme() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::with_theme(temp_dir);
    let page = page::index(&config);
    assert!(page
        .contents
        .contains(r#"html lang="en" id="oranda" class="cupcake""#));
}

#[test]
fn creates_nav() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let page = page::index(&config);
    assert!(page.contents.contains(r#"<nav class="nav">"#));
    assert!(page.contents.contains(r#"<a href="/">"#));
    assert!(page.contents.contains(r#"<a href="/docs/src/cli/">"#));
    assert!(page.contents.contains(r#"<a href="/SECURITY/">"#));
}

#[test]
fn creates_nav_no_additional_pages() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let page = page::index(&config);
    assert!(page.contents.contains(r#"<nav class="nav">"#));
}

#[test]
fn creates_footer() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let page = page::index(&config);
    dbg!(&page.contents);
    assert!(page.contents.contains(r#"<footer>"#));
    assert!(page.contents.contains(r#"My Oranda Project"#))
}

#[test]
fn creates_nav_item() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::cargo_dist(temp_dir);
    let page = page::index_with_artifacts(&config);
    assert!(page
        .contents
        .contains(r#"<li><a href="/artifacts/">Install</a></li>"#));
}

#[test]
fn loads_js() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::cargo_dist(temp_dir);
    let page = page::index_with_artifacts(&config);
    assert!(page.contents.contains(r#"<script src="/artifacts.js">"#));
}

#[test]
fn creates_download_for_mac() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::cargo_dist(temp_dir);
    let page = page::index_with_artifacts(&config);
    assert!(page
        .contents
        .contains(r#"<option value="x86_64-apple-darwin">macOS Intel</option>"#));
}

#[test]
fn creates_downloads_page() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::cargo_dist(temp_dir);
    let artifacts_page = page::artifacts(&config);
    assert!(artifacts_page.contents.contains(r#"<h3>Downloads</h3>"#));
    assert!(artifacts_page
        .contents
        .contains(r#"x86_64-pc-windows-msvc.tar.gz</a>"#));
    assert!(artifacts_page.contents.contains(r#"<h3>powershell</h3>"#))
}

#[test]
fn creates_nav_item_install() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::package_managers(temp_dir);
    let page = page::index_with_artifacts(&config);
    eprintln!("{}", page.contents);
    assert!(page.contents.contains("<h4>Install "));
}

#[test]
fn creates_copy_to_clipboard_home() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::cargo_dist(temp_dir);
    let page = page::index_with_artifacts(&config);
    assert!(page.contents.contains("copy-clipboard-button"));
    assert!(page.contents.contains(r#"installer.sh.txt">Source</a>"#));
}

#[test]
fn creates_copy_to_clipboard_artifacts() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::package_managers(temp_dir);
    let page = page::artifacts(&config);
    assert!(page.contents.contains(
        r#"<button class="button copy-clipboard-button primary" data-copy="npm install oranda">"#
    ));
}

#[test]
fn adds_prefix() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::path_prefix(temp_dir);
    let page = page::index_with_artifacts(&config);
    assert!(page.contents.contains("<script src=\"/axo/artifacts.js\">"));
    assert!(page
        .contents
        .contains(r#"href="/axo/artifacts/">View all installation options</a>"#))
}

#[test]
fn adds_prefix_with_package_managers() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::path_prefix_with_package_managers(temp_dir);
    let page = page::index_with_artifacts(&config);
    assert!(page.contents.contains("<script src=\"/axo/artifacts.js\">"));
    assert!(page
        .contents
        .contains(r#"href="/axo/artifacts/">View all installation options</a>"#))
}

#[test]
fn adds_changelog_nav() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::changelog(temp_dir);
    let page = page::index(&config);
    assert!(page.contents.contains("/changelog/"));
}

#[test]
fn it_renders_code_blocks_with_invalid_annotations() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::with_warning(temp_dir);
    let page = page::index(&config);
    assert!(page
        .contents
        .contains("this block will render but not be highlighted!"));
}

#[test]
fn it_inserts_plausible_tag() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::analytics_plausible(temp_dir);
    let page = page::index(&config);
    dbg!(&page.contents);
    assert!(page.contents.contains(r#"<script defer="true" data-domain="opensource.axo.dev" src="https://plausible.io/js/script.js"></script>"#))
}
