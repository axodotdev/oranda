use std::fs::File;
use std::io::Write;
use std::path::Path;

pub mod artifacts;
mod layout;
pub mod markdown;
pub mod page;

use crate::config::Config;
use crate::errors::*;

#[derive(Debug)]
pub struct Site {
    pub html: String,
}

impl Site {
    pub fn build(config: &Config, file_path: &String) -> Result<Site> {
        Self::create_dist_dir(&config.dist_dir)?;
        let markdown_path = Path::new(&file_path);
        let is_main_readme = file_path == &config.readme_path;
        let content = markdown::body(markdown_path, &config.syntax_theme, is_main_readme)?;
        let html = page::build(config, content, is_main_readme)?;

        if let Some(book_path) = &config.md_book {
            Self::copy_static(&config.dist_dir, book_path)?;
        }

        Ok(Site { html })
    }

    pub fn copy_static(dist_path: &String, static_path: &String) -> Result<()> {
        Self::create_dist_dir(dist_path)?;
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        fs_extra::copy_items(&[static_path], dist_path, &options)?;

        Ok(())
    }

    pub fn write(config: &Config) -> Result<()> {
        let dist = &config.dist_dir;
        let readme_path = &config.readme_path;
        if Path::new(&config.static_dir).exists() {
            Self::copy_static(dist, &config.static_dir)?;
        }
        let mut files = vec![readme_path];
        if config.additional_pages.is_some() {
            files.extend(config.additional_pages.as_ref().unwrap())
        }

        for file in files {
            let site = Self::build(config, file)?;
            let file_name = page::get_html_file_name(file, config)?;

            let html_path = format!("{}/{}", &dist, file_name);

            let mut html_file = File::create(html_path)?;
            html_file.write_all(site.html.as_bytes())?;
        }

        Ok(())
    }

    fn create_dist_dir(dist_path: &String) -> Result<()> {
        if !Path::new(dist_path).exists() {
            std::fs::create_dir_all(dist_path)?;
        }

        Ok(())
    }
}
