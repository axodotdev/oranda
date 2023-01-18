use crate::errors::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
mod head;
mod header;
mod html;
pub mod markdown;

#[cfg(test)]
use crate::config::theme::Theme;
use crate::config::Config;
use crate::create_runtime_tokio;

#[derive(Debug)]
pub struct Site {
    pub html: String,
}

impl Site {
    fn build(config: &Config, file_path: &String) -> Result<Site> {
        let dist = &config.dist_dir;
        std::fs::create_dir_all(dist)?;
        let readme_path = Path::new(&file_path);
        let content = markdown::body(readme_path)?;
        let html = html::build(config, content)?;

        Ok(Site { html })
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
        let dist = &config.dist_dir;

        let mut files = vec![readme_path];
        if config.additional_pages.is_some() {
            files.extend(config.additional_pages.as_ref().unwrap())
        }

        for file in files {
            let site = Self::build(config, file)?;
            let file_name = Self::get_html_file_name(file, config).unwrap();

            let html_path = format!("{}/{}", &dist, file_name);

            let mut html_file = File::create(html_path)?;
            html_file.write_all(site.html.as_bytes())?;
        }

        Ok(())
    }
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
fn it_builds_the_site() {
    let _guard = create_runtime_tokio().enter();
    let site = Site::build(&config(), &config().readme_path).unwrap();
    assert!(site.html.contains("<h1>axo</h1>"));
    assert!(site.html.contains("axo-oranda.css"));
}

#[test]
fn reads_description() {
    let _guard = create_runtime_tokio().enter();
    let site = Site::build(&config(), &config().readme_path).unwrap();
    println!("{:?}", site.html);
    assert!(site.html.contains("you axolotl questions"));
    assert!(site.html.contains("My Axo project"))
}

#[test]
fn reads_theme() {
    let _guard = create_runtime_tokio().enter();
    let site = Site::build(&config(), &config().readme_path).unwrap();
    assert!(site.html.contains("html class=\"dark\""));
}

#[test]
fn creates_nav() {
    let _guard = create_runtime_tokio().enter();
    let site = Site::build(&config(), &config().readme_path).unwrap();

    assert!(site.html.contains("<nav class=\"nav\"><ul><li><a href=\"/\">Home</a></li><li><a href=\"/readme\">readme</a></li></ul></nav>"));
}
