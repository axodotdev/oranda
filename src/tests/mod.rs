#[cfg(test)]
use std::fs;

#[cfg(test)]
use crate::config::theme::Theme;
#[cfg(test)]
use crate::config::Config;
#[cfg(test)]
use crate::site::Site;

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

#[cfg(test)]
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

    assert!(site.html.contains("<nav class=\"nav\"><ul><li><a href=\"/\">Home</a></li><li><a href=\"/readme\">readme</a></li></ul></nav>"));
}
