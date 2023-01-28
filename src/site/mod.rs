use std::path::Path;

pub mod artifacts;
mod layout;
pub mod page;
use page::Page;

use crate::config::Config;
use crate::errors::*;

pub struct Site {
    pages: Vec<Page>,
    config: Config,
}

impl Site {
    pub fn new(config: Config) -> Result<Self> {
        let mut files = vec![config.readme_path];
        let mut pages = vec![];
        if let Some(additional) = config.additional_pages {
            files.extend(additional);
        }
        for file in files {
            pages.push(Page::new_from_source(&config, &file).await?);
        }
        Ok(Site { pages, config })
    }

    pub async fn build(&self) -> Result<()> {
        let static_dir = &self.config.static_dir;
        let dist_dir = &self.config.dist_dir;

        Self::create_dist_dir(dist_dir)?;
        if Path::new(static_dir).exists() {
            Self::copy_static(dist_dir, static_dir)?;
        }

        for page in &self.pages {
            let bytes = page.contents.as_bytes().to_vec();
            let asset = axoasset::new(&page.filename, bytes)?;
            axoasset::write(asset, &dist_dir).await?;
        }
        Ok(())
    }

    fn create_dist_dir(dist_path: &str) -> Result<()> {
        if Path::new(dist_path).exists() {
            std::fs::create_dir_all(dist_path)?;
        }

        Ok(())
    }

    fn copy_static(dist_path: &str, static_path: &str) -> Result<()> {
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        fs_extra::copy_items(&[static_path], dist_path, &options)?;

        Ok(())
    }
}
