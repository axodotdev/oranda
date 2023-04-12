use axoasset::SourceFile;
use oranda::config::Config;
use oranda::site::{self, artifacts, markdown, page::Page};

fn readme() -> SourceFile {
    SourceFile::new(
        "readme.md",
        r#"
# axo
> a fun side project

```sh
$ axo | lotl
```

```
this block has no highlight annotation
```
"#
        .to_owned(),
    )
}

fn readme_invalid_annotation() -> SourceFile {
    SourceFile::new(
        "readme.md",
        r#"
# axo
> a fun side project

```sh
$ axo | lotl
```

```farts
fn this_annotation_will_never_be_supported() {
    println!("this block will render but not be highlighted!");
}
```"#
            .to_owned(),
    )
}

fn reset(dist_dir: &str) {
    site::Site::clean_dist_dir(dist_dir).unwrap();
}

pub fn index(config: &Config) -> String {
    reset(&config.dist_dir);
    let src = readme();
    let page = Page {
        contents: markdown::to_html(&src, &config.syntax_theme).unwrap(),
        filename: "index.html".to_string(),
        is_index: true,
        needs_js: true,
    };
    page.build(config).unwrap()
}

pub fn index_with_warning(config: &Config) -> String {
    reset(&config.dist_dir);
    let src = readme_invalid_annotation();
    let page = Page {
        contents: markdown::to_html(&src, &config.syntax_theme).unwrap(),
        filename: "index.html".to_string(),
        is_index: true,
        needs_js: true,
    };
    page.build(config).unwrap()
}

pub fn artifacts(config: &Config) -> String {
    reset(&config.dist_dir);
    let artifacts_content = artifacts::page::build(config).unwrap();
    let page = Page::new_from_contents(artifacts_content, "artifacts.html", true);
    page.build(config).unwrap()
}
