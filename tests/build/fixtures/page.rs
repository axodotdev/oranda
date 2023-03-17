use oranda::config::Config;
use oranda::site::{self, artifacts, layout, markdown, page::Page};

fn readme() -> String {
    r#"
# axo
> a fun side project

```sh
$ axo | lotl
```"#
        .to_string()
}

fn reset(dist_dir: &str) {
    site::Site::clean_dist_dir(dist_dir).unwrap();
}

pub fn index(config: &Config) -> String {
    reset(&config.dist_dir);
    let page = Page {
        contents: markdown::to_html(readme(), &config.syntax_theme).unwrap(),
        filename: "index.html".to_string(),
        is_index: true,
        needs_js: true,
    };
    let needs_js = page.needs_js;
    let contents = page.build(config).unwrap();
    layout::build(config, contents, needs_js).unwrap()
}

pub fn artifacts(config: &Config) -> String {
    reset(&config.dist_dir);
    let artifacts_content = artifacts::page::build(config).unwrap();
    let page = Page::new_from_contents(artifacts_content, "artifacts.html", true);
    let needs_js = page.needs_js;
    let contents = page.build(config).unwrap();
    layout::build(config, contents, needs_js).unwrap()
}
