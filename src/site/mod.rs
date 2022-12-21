use crate::errors::*;
use crate::site::css::build;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod css;
mod html;
pub mod markdown;

#[cfg(test)]
use crate::config::theme::Theme;

use crate::config::Config;

pub struct Site {
    pub html: String,
    pub css: String,
}

impl Site {
    fn css(config: &Config) -> Result<String> {
        let css = build(config).unwrap();
        Ok(css)
    }

    fn build(config: &Config, file_path: &String) -> Result<Site> {
        let readme_path = Path::new(&file_path);
        let content = markdown::body(readme_path)?;
        let html = html::build(config, content);
        let css = Self::css(config)?;

        Ok(Site { html, css })
    }

    fn get_html_file_name(file: &String, config: &Config) -> Result<String> {
        let file_name = if file == &config.readme_path {
            "index.html".to_string()
        } else {
            let file_path = Path::new(file).file_stem();

            match file_path {
                None => {
                    return Err(OrandaError::FileNotFound {
                        filedesc: "Additional File".to_string(),
                        path: file.to_string(),
                    });
                }
                Some(p) => format!("{}.html", p.to_str().unwrap()),
            }
        };

        Ok(file_name)
    }

    pub fn write(config: &Config) -> Result<()> {
        let readme_path = &config.readme_path;
        let site = Self::build(config, readme_path)?;
        let dist = &config.dist_dir;
        let mut files = vec![readme_path];
        if config.additional_files.is_some() {
            files.extend(config.additional_files.as_ref().unwrap())
        }

        for file in files {
            let site = Self::build(config, file)?;
            let file_name = Self::get_html_file_name(file, &config).unwrap();

            std::fs::create_dir_all(dist)?;
            let html_path = format!("{}/{}", &dist, file_name);

            let mut html_file = File::create(html_path)?;
            html_file.write_all(site.html.as_bytes())?;
        }

        let css_path = format!("{}/styles.css", &dist);

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
    let site = Site::build(&config(), &config().readme_path).unwrap();
    assert!(site
        .css
        .contains("--text-light:#fafafa;--text-800:#1f2937;"));
    assert!(site.html.contains("<h1>axo</h1>"));
}

#[test]
fn reads_description() {
    let site = Site::build(&config(), &config().readme_path).unwrap();
    assert!(site.html.contains("you axolotl questions"));
}

#[test]
fn reads_theme() {
    let site = Site::build(&config(), &config().readme_path).unwrap();
    assert!(site.html.contains("<div class=\"body container dark\">"));
}

#[test]
fn reads_additional_css() {
    let site = Site::build(&config(), &config().readme_path).unwrap();
    assert!(site.css.contains("#oranda body{background:red}"));
}
