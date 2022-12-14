use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

use grass::OutputStyle;

mod html;
mod markdown;

#[cfg(test)]
use crate::config::theme::Theme;

use crate::config::Config;
use crate::errors::*;

pub struct Site {
    pub html: String,
    pub css: String,
}

impl Site {
    fn css(config: &Config) -> Result<String> {
        let css_options = grass::Options::default().style(OutputStyle::Compressed);
        let mut css = grass::from_path("src/site/css/style.scss", &css_options)?;

        if !config.additional_css.is_empty() && Path::new(&config.additional_css).exists() {
            let additional_css_str = read_to_string(&config.additional_css)?;

            let additional_css =
                grass::from_string(format!("#oranda{{{}}}", additional_css_str), &css_options)?;

            css = format!("{css}{additional}", css = css, additional = additional_css);
        }
        Ok(css)
    }

    fn build(config: &Config) -> Result<Site> {
        let readme_path = Path::new(&config.readme_path);
        let html = format!(
            "{}{}{}",
            html::head(config),
            markdown::body(readme_path)?,
            html::footer()
        );
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
        description: String::from("description"),
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
    assert!(site
        .html
        .contains("<meta name=\"description\" content=\"description\">"));
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
