use oranda::config::Config;
use oranda::site::artifacts;
use oranda::site::layout;
use oranda::site::markdown;
use oranda::site::page::Page;

fn readme() -> String {
    r#"
# axo
> a fun side project

```sh
$ axo | lotl
```"#
        .to_string()
}

pub fn index(config: &Config) -> String {
    let page = Page {
        contents: markdown::to_html(readme(), &config.syntax_theme).unwrap(),
        filename: "index.html".to_string(),
        is_index: true,
        needs_js: true,
    };
    let contents = page.build(config, true).unwrap();
    layout::build(config, contents, true).unwrap()
}

pub fn artifacts(config: &Config) -> String {
    let artifacts_content = artifacts::page::build(config).unwrap();
    let page = Page::new_from_contents(artifacts_content, "artifacts.html", true);
    let contents = page.build(config, true).unwrap();
    layout::build(config, contents, true).unwrap()
}
