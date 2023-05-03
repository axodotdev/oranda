use oranda::config::Config;
use oranda::data::Context;
use oranda::site::{self, artifacts, changelog, layout::Layout, markdown, page::Page};

fn readme() -> &'static str {
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
}

fn readme_invalid_annotation() -> &'static str {
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
}

fn reset(dist_dir: &str) {
    site::Site::clean_dist_dir(dist_dir).unwrap();
}

pub fn index(config: &Config, layout: &Layout) -> Page {
    reset(&config.dist_dir);
    let body = markdown::to_html(readme(), &config.syntax_theme).unwrap();
    Page::new_from_contents(body, "index.html", layout)
}

pub fn index_with_artifacts(config: &Config, layout: &Layout) -> Page {
    reset(&config.dist_dir);
    let repo_url = config.repository.as_ref().unwrap();
    let context = Context::new(repo_url).unwrap();
    Page::index_with_artifacts(&context, layout, config).unwrap()
}

pub fn index_with_warning(config: &Config, layout: &Layout) -> Page {
    reset(&config.dist_dir);
    let body = markdown::to_html(readme_invalid_annotation(), &config.syntax_theme).unwrap();
    Page::new_from_contents(body, "index.html", layout)
}

pub fn artifacts(config: &Config, layout: &Layout) -> Page {
    reset(&config.dist_dir);
    let repo_url = config.repository.as_ref().unwrap();
    let context = Context::new(repo_url).unwrap();
    let artifacts_content = artifacts::page(&context, config).unwrap();
    Page::new_from_contents(artifacts_content, "artifacts.html", layout)
}

pub fn changelog(config: &Config, layout: &Layout) -> Page {
    reset(&config.dist_dir);
    let repo_url = config.repository.as_ref().unwrap();
    let context = Context::new(repo_url).unwrap();
    let changelog_content = changelog::build(&context, config).unwrap();
    Page::new_from_contents(changelog_content, "changelog.html", layout)
}
