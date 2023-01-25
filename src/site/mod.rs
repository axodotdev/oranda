use crate::errors::*;

use std::fs::File;
use std::io::Write;
use std::path::Path;
mod css;
mod head;
mod header;
mod html;
pub mod markdown;
use crate::config::Config;

#[derive(Debug)]
pub struct Site {
    pub html: String,
}

impl Site {
    pub fn build(config: &Config, file_path: &String) -> Result<Site> {
        let readme_path = Path::new(&file_path);
        let content = markdown::body(readme_path, &config.syntax_theme)?;
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

    pub fn copy_static(dist_path: &String, static_path: &String) -> Result<()> {
        let dist = &config.dist_dir;
        std::fs::create_dir_all(dist)?;
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        fs_extra::copy_items(&[static_path], dist_path, &options)?;

        Ok(())
    }

    pub fn write(config: &Config) -> Result<()> {
        let readme_path = &config.readme_path;
        let dist = &config.dist_dir;
        Self::copy_static(dist, &config.static_dir)?;

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
