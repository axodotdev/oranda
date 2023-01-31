use oranda::config::Config;
use oranda::site::artifacts;
use oranda::site::layout;
use oranda::site::page::Page;

pub fn index(config: &Config) -> String {
    let page = Page::new_from_file(config, &config.readme_path).unwrap();
    let contents = page.build(config).unwrap();
    layout::build(config, contents, true).unwrap()
}

pub fn artifacts(config: &Config) -> String {
    let artifacts_content = artifacts::page::build(config).unwrap();
    let page = Page::new_from_contents(artifacts_content, "artifacts.html");
    let contents = page.build(config).unwrap();
    layout::build(config, contents, false).unwrap()
}
