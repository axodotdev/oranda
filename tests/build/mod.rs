mod fixtures;
use super::utils::tokio_utils::TEST_RUNTIME;
use fixtures::{oranda_config, page};

#[test]
fn it_adds_additional_css() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::no_artifacts();
    let page_html = page::index(config);
    assert!(page_html.contains("<link href=\"custom.css\" rel=\"stylesheet\"/>"));
}

#[test]
fn it_renders_changelog_with_no_cargo_dist() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::changelog();
    let page_html = page::changelog(config);
    assert!(page_html.contains("<h1>Releases</h1>"));
}

#[test]
fn it_renders_changelog_with_release_content() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::changelog();
    let page_html = page::changelog(config);
    assert!(page_html.contains("Initial release."));
}

#[test]
fn it_adds_oranda_css() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::no_artifacts();
    let page_html = page::index(config);
    assert!(page_html.contains("<link href=\"oranda.css\" rel=\"stylesheet\"/>"));
}

#[test]
fn it_builds_the_site() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::no_artifacts();
    let page_html = page::index(config);
    assert!(page_html.contains("<h1>axo</h1>"));
    assert!(page_html.contains("custom.css"));
}

#[test]
fn reads_description() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::no_artifacts();
    let page_html = page::index(config);
    assert!(page_html.contains("you axolotl questions"));
    assert!(page_html.contains("My Axo project"))
}

#[test]
fn reads_theme() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::no_artifacts();
    let page_html = page::index(config);
    assert!(page_html.contains("html class=\"dark\""));
}

#[test]
fn creates_nav() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::no_artifacts();
    let page_html = page::index(config);

    assert!(page_html.contains("<nav class=\"nav\"><ul><li><a href=\"/\">Home</a></li><li><a href=\"/README.html\">Another Page</a></li></ul></nav>"));
}

#[test]
fn creates_nav_no_additional_pages() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::cargo_dist();
    let page_html = page::index(config);

    assert!(page_html.contains("<nav class=\"nav\">"));
}

#[test]
fn creates_footer() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::no_artifacts();
    let page_html = page::index(config);

    assert!(page_html.contains("<footer class=\"footer\"><span>My Axo project</span></footer>"));
}

#[test]
fn creates_nav_item() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::cargo_dist();
    let page_html = page::index(config);
    assert!(page_html.contains("<li><a href=\"/artifacts.html\">Install</a></li>"));
}

#[test]
fn loads_js() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::cargo_dist();
    let page_html = page::index(config);
    assert!(page_html.contains("<script src=\"/artifacts.js\">"));
}

#[test]
fn creates_download_for_mac() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::cargo_dist();
    let page_html = page::index(config);
    assert!(page_html.contains("<span class=\"detect\">We have detected you are on <span class=\"detected-os\">mac</span>, are we wrong?</span>"));
}

#[test]
fn creates_downloads_page() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::cargo_dist();
    let artifacts_page = page::artifacts(config);
    assert!(artifacts_page.contains("<h3>Downloads</h3>"));
    assert!(
        artifacts_page.contains("<span>Executable Zip</span><span>x86_64-pc-windows-msvc</span>")
    );
    assert!(artifacts_page.contains("<h3>Install via script</h3>"))
}

#[test]
fn creates_nav_item_install() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::package_managers();
    let page_html = page::index(config);
    assert!(page_html.contains("View all downloads"));
}

#[test]
fn creates_copy_to_clipboard_home() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::cargo_dist();
    let page_html = page::index(config);
    assert!(page_html.contains("copy-clipboard-button"));
    assert!(page_html.contains("installer.sh.txt\">Source</a>"));
}

#[test]
fn creates_copy_to_clipboard_artifacts() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::package_managers();
    let page_html = page::artifacts(config);
    assert!(
        page_html.contains("<button class=\"button primary\" data-copy=\"npm install oranda\">")
    );
}

#[test]
fn adds_prefix() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::path_prefix();
    let page_html = page::index(config);
    assert!(page_html.contains("<script src=\"/axo/artifacts.js\">"));
    assert!(page_html.contains("<a href=\"/axo/artifacts.html\">View all installation options</a>"))
}

#[test]
fn adds_changelog_nav() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::changelog();
    let page_html = page::index(config);
    assert!(page_html.contains("changelog.html"));
}

#[test]
fn it_renders_code_blocks_with_invalid_annotations() {
    let _guard = TEST_RUNTIME.enter();
    let config = &oranda_config::no_artifacts();
    let page_html = page::index_with_warning(config);
    assert!(page_html.contains("this block will render but not be highlighted!"));
}
