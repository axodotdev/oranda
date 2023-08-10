use assert_fs::prelude::PathChild;
use assert_fs::TempDir;
use scraper::{Html, Selector};
use serde_json::json;

mod fixtures;
use super::utils::tokio_utils::TEST_RUNTIME;
use fixtures::oranda_config;
use oranda::config::style::ORANDA_CSS_TAG;
use oranda::site::page::Page;
use oranda::site::Site;

fn temp_build_dir() -> TempDir {
    TempDir::new().unwrap()
}

fn find_page<'a>(pages: &'a [Page], name: &str) -> &'a Page {
    let page = pages.iter().find(|p| p.filename == name);
    assert!(page.is_some());
    page.unwrap()
}

fn assert_selector_exists(html: &str, selector: &str) {
    let doc = Html::parse_document(html);
    let sel = Selector::parse(selector).unwrap();
    let els = doc.select(&sel);
    assert!(els.count() > 0);
}

fn selector_get_inner(html: &str, selector: &str) -> String {
    let doc = Html::parse_document(html);
    let sel = Selector::parse(selector).unwrap();
    let els = doc.select(&sel);
    let mut iter = els.peekable();
    assert!(iter.peek().is_some());
    let first = iter.next().unwrap();
    first.inner_html()
}

#[test]
fn it_adds_additional_css() {
    let _guard = TEST_RUNTIME.enter();
    let mut t = temp_build_dir();
    let config = oranda_config::from_json(
        json!({
            "styles": {
                "additional_css": ["https://raw.githubusercontent.com/axodotdev/axii/main/css/main.css"]
            }
        }),
        &mut t,
    );
    let site = Site::build_single(&config, None).unwrap();
    let page = find_page(&site.pages, "index.html");
    assert_selector_exists(&page.contents, "link[rel='stylesheet'][href='/custom.css']");
}

#[test]
fn it_can_point_to_custom_repository() {
    let _guard = TEST_RUNTIME.enter();
    let mut t = temp_build_dir();
    let config = oranda_config::from_json(
        json!({
            "project": {
                "repository": "https://github.com/axodotdev/privaterepo"
            }
        }),
        &mut t,
    );
    let site = Site::build_single(&config, None).unwrap();
    let page = find_page(&site.pages, "index.html");
    assert_selector_exists(
        &page.contents,
        ".repo_banner>a[href='https://github.com/axodotdev/privaterepo']",
    );
}

#[test]
fn it_renders_changelog_with_release_content() {
    let _guard = TEST_RUNTIME.enter();
    let mut t = temp_build_dir();
    let config = oranda_config::from_json(
        json!({
            "components": {
                "changelog": true
            }
        }),
        &mut t,
    );
    let site = Site::build_single(&config, None).unwrap();
    let page = find_page(&site.pages, "changelog.html");
    let html = selector_get_inner(&page.contents, "h2[id='tag-v0.0.1']~.release-body p");
    assert_eq!(html, "Initial release.");
}

#[test]
fn it_adds_oranda_css() {
    let _guard = TEST_RUNTIME.enter();
    let mut t = temp_build_dir();
    let config = oranda_config::from_json(json!({}), &mut t);
    let site = Site::build_single(&config, None).unwrap();
    let css_name = format!("oranda-{}.css", ORANDA_CSS_TAG);
    assert!(t.child(&css_name).exists());
    let page = find_page(&site.pages, "index.html");
    assert_selector_exists(&page.contents, &format!("link[href='/{}']", css_name));
}

#[test]
fn it_adds_oranda_css_with_pinned_version() {
    let _guard = TEST_RUNTIME.enter();
    let mut t = temp_build_dir();
    let config = oranda_config::from_json(
        json!({
            "styles": {
                "oranda_css_version": "v0.1.0",
            }
        }),
        &mut t,
    );
    let site = Site::build_single(&config, None).unwrap();
    assert!(t.child("oranda-v0.1.0.css").exists());
    let page = find_page(&site.pages, "index.html");
    assert_selector_exists(&page.contents, "link[href='/oranda-v0.1.0.css']");
}

#[test]
fn reads_description() {
    let _guard = TEST_RUNTIME.enter();
    let mut t = temp_build_dir();
    let config = oranda_config::from_json(
        json!({
            "project": {
                "description": "you axolotl questions"
            }
        }),
        &mut t,
    );
    let site = Site::build_single(&config, None).unwrap();
    let page = find_page(&site.pages, "index.html");
    assert_selector_exists(&page.contents, "meta[content='you axolotl questions']");
}

#[test]
fn reads_theme() {
    let _guard = TEST_RUNTIME.enter();
    let mut t = temp_build_dir();
    let config = oranda_config::from_json(
        json!({
            "styles": {
                "theme": "cupcake"
            }
        }),
        &mut t,
    );
    let site = Site::build_single(&config, None).unwrap();
    let page = find_page(&site.pages, "index.html");
    assert_selector_exists(&page.contents, "html.cupcake");
}

#[test]
fn creates_footer() {
    let _guard = TEST_RUNTIME.enter();
    let mut t = temp_build_dir();
    let config = oranda_config::from_json(json!({}), &mut t);
    let site = Site::build_single(&config, None).unwrap();
    let page = find_page(&site.pages, "index.html");
    assert_selector_exists(
        &page.contents,
        "footer>a[href='https://github.com/axodotdev/oranda']",
    );
    assert!(selector_get_inner(&page.contents, "footer span").contains("MIT OR Apache-2.0"));
}

#[test]
fn creates_downloads_page() {
    let _guard = TEST_RUNTIME.enter();
    let mut t = temp_build_dir();
    let config = oranda_config::from_json(
        json!({
            "components": {
                "artifacts": {
                    "cargo_dist": true
                }
            }
        }),
        &mut t,
    );
    let site = Site::build_single(&config, None).unwrap();
    let page = find_page(&site.pages, "artifacts.html");
    assert_selector_exists(&page.contents, ".artifacts-table");
}

#[test]
fn adds_prefix() {
    let _guard = TEST_RUNTIME.enter();
    let mut t = temp_build_dir();
    let config = oranda_config::from_json(
        json!({
            "build": {
                "path_prefix": "axo"
            }
        }),
        &mut t,
    );
    let site = Site::build_single(&config, None).unwrap();
    let page = find_page(&site.pages, "index.html");
    assert_selector_exists(&page.contents, "script[src='/axo/artifacts.js']");
    assert_selector_exists(&page.contents, "nav.nav a[href='/axo/']");
}

#[test]
fn it_inserts_plausible_tag() {
    let _guard = TEST_RUNTIME.enter();
    let mut t = temp_build_dir();
    let config = oranda_config::from_json(
        json!({
            "marketing": {
                "analytics": {
                    "plausible": {
                        "domain": "opensource.axo.dev"
                    }
                }
            }
        }),
        &mut t,
    );
    let site = Site::build_single(&config, None).unwrap();
    let page = find_page(&site.pages, "index.html");
    assert_selector_exists(&page.contents, "script[defer='true'][data-domain='opensource.axo.dev'][src='https://plausible.io/js/script.js']");
}
