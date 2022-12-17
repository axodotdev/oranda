use crate::errors::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod css;
mod html;
mod logo;
mod markdown;

#[cfg(test)]
use crate::config::theme::Theme;

use crate::config::Config;

pub struct Site {
    pub html: String,
    pub css: String,
}

impl Site {
    fn css(config: &Config) -> Result<String> {
        css::build(config)
    }

    fn build(config: &Config) -> Result<Site> {
        let readme_path = Path::new(&config.readme_path);
        let content = markdown::body(readme_path)?;
        let html = html::build(config, content);
        let css = Self::css(config)?;

        Ok(Site { html, css })
    }

    pub fn write(config: &Config) -> Result<()> {
        let site = Self::build(config)?;

        let dist = &config.dist_dir;
        std::fs::create_dir_all(dist)?;
        let html_path = format!("{}/index.html", &dist);
        let css_path = format!("{}/styles.css", &dist);

        let mut html_file = File::create(html_path)?;
        html_file.write_all(site.html.as_bytes())?;

        let mut css_file = File::create(css_path)?;
        css_file.write_all(site.css.as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
fn config() -> Config {
    Config {
        description: String::from("you axolotl questions"),
        readme_path: String::from("./src/site/fixtures/readme.md"),
        additional_css: String::from("./src/site/fixtures/additional.css"),
        theme: Theme::Dark,
        ..Default::default()
    }
}

#[test]
fn it_builds_the_site() {
    let site = Site::build(&config()).unwrap();
    assert!(site
        .css
        .contains("--text-light:#fafafa;--text-800:#1f2937;"));
    assert!(site.html.contains("<h1>axo</h1>"));
}

#[test]
fn reads_description() {
    let site = Site::build(&config()).unwrap();
    assert!(site.html.contains("you axolotl questions"));
}

#[test]
fn reads_theme() {
    let site = Site::build(&config()).unwrap();
    assert!(site.html.contains("<div class=\"body dark\">"));
}

#[test]
fn reads_additional_css() {
    let site = Site::build(&config()).unwrap();
    assert!(site.css.contains("#oranda body{background:red}"));
}
