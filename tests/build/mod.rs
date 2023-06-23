use assert_fs::TempDir;
use oranda::config::style::ORANDA_CSS_TAG;
use oranda::site::layout::Layout;

mod fixtures;
use super::utils::tokio_utils::TEST_RUNTIME;
use fixtures::{oranda_config, page};

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
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    assert!(page
        .contents
        .contains(r#"<link href="/custom.css" rel="stylesheet"/>"#));
}

#[test]
fn it_renders_changelog_with_no_cargo_dist() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::changelog(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::changelog(&config, &layout);
    assert!(page.contents.contains(r#"<h1>Releases</h1>"#));
}

#[test]
fn it_renders_changelog_with_release_content() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::changelog(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::changelog(&config, &layout);
    assert!(page.contents.contains("Initial release."));
}

#[test]
fn it_adds_oranda_css() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    let filename = format!("oranda-{ORANDA_CSS_TAG}.css");
    assert!(page.contents.contains(&filename));
}

#[test]
fn it_adds_oranda_css_with_pinned_version() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::pinned_css(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    assert!(page
        .contents
        .contains(r#"<link href="/oranda-css-v0.0.3.css" rel="stylesheet"/>"#));
}

#[test]
fn it_builds_the_site() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    assert!(page.contents.contains(r#"<h1>axo</h1>"#));
    assert!(page.contents.contains("custom.css"));
}

#[test]
fn reads_description() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    assert!(page.contents.contains("you axolotl questions"));
    assert!(page.contents.contains("My Oranda Project"))
}

#[test]
fn reads_theme() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    assert!(page.contents.contains(r#"html class="dark""#));
}

#[test]
fn creates_nav() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    eprintln!("{}", page.contents);
    assert!(page.contents.contains(r#"<nav class="nav"><ul><li><a href="/">Home</a></li><li><a href="/README/">Another Page</a></li></ul></nav>"#));
}

#[test]
fn creates_nav_no_additional_pages() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    assert!(page.contents.contains(r#"<nav class="nav">"#));
}

#[test]
fn creates_footer() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    assert!(page
        .contents
        .contains(r#"<footer><span>My Oranda Project</span></footer>"#));
}

#[test]
fn creates_nav_item() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::cargo_dist(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index_with_artifacts(&config, &layout);
    assert!(page
        .contents
        .contains(r#"<li><a href="/artifacts/">Install</a></li>"#));
}

#[test]
fn loads_js() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::cargo_dist(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index_with_artifacts(&config, &layout);
    assert!(page.contents.contains(r#"<script src="/artifacts.js">"#));
}

#[test]
fn creates_download_for_mac() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::cargo_dist(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index_with_artifacts(&config, &layout);
    assert!(page
        .contents
        .contains(r#"<option value="x86_64-apple-darwin">x64 macOS</option>"#));
}

#[test]
fn creates_downloads_page() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::cargo_dist(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let artifacts_page = page::artifacts(&config, &layout);
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
    let layout = Layout::new(&config).unwrap();
    let page = page::index_with_artifacts(&config, &layout);
    eprintln!("{}", page.contents);
    assert!(page.contents.contains("<h4>Install "));
}

#[test]
fn creates_copy_to_clipboard_home() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::cargo_dist(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index_with_artifacts(&config, &layout);
    assert!(page.contents.contains("copy-clipboard-button"));
    assert!(page.contents.contains(r#"installer.sh.txt">Source</a>"#));
}

#[test]
fn creates_copy_to_clipboard_artifacts() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::package_managers(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::artifacts(&config, &layout);
    assert!(page.contents.contains(
        r#"<button class="button copy-clipboard-button primary" data-copy="npm install oranda">"#
    ));
}

#[test]
fn adds_prefix() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::path_prefix(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index_with_artifacts(&config, &layout);
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
    let layout = Layout::new(&config).unwrap();
    let page = page::index_with_artifacts(&config, &layout);
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
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    assert!(page.contents.contains("/changelog/"));
}

#[test]
fn it_renders_code_blocks_with_invalid_annotations() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index_with_warning(&config, &layout);
    assert!(page
        .contents
        .contains("this block will render but not be highlighted!"));
}

#[test]
fn it_inserts_plausible_tag() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::analytics_plausible(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    dbg!(&page.contents);
    assert!(page.contents.contains(r#"<script defer="true" src="https://plausible.io/js/script.js" data-domain="opensource.axo.dev"></script>"#))
}
