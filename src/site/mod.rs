use std::collections::HashMap;
use std::path::Path;

use axoasset::LocalAsset;
use camino::{Utf8Path, Utf8PathBuf};

use crate::config::Config;
use crate::data::Context;
use crate::errors::*;
use crate::message::{Message, MessageType};

pub mod artifacts;
pub mod icons;
pub mod layout;
use layout::{css, javascript, Layout};
pub mod link;
pub mod markdown;
pub mod page;
use crate::data::funding::Funding;
use page::Page;

pub mod changelog;
pub mod funding;
pub mod mdbook;

#[derive(Debug)]
pub struct Site {
    pages: Vec<Page>,
}

impl Site {
    pub fn build(config: &Config, cached: bool) -> Result<Site> {
        Self::clean_dist_dir(&config.dist_dir)?;

        let mut pages = vec![];
        let layout_template = Layout::new(config)?;

        if let Some(files) = &config.additional_pages {
            let mut additional_pages =
                Self::build_additional_pages(files, &layout_template, config)?;
            pages.append(&mut additional_pages);
        }

        let mut index = None;

        if Self::needs_context(config) {
            match &config.repository {
                Some(repo_url) => {
                let context = if cached {
                    match Context::read_cache() {
                        Ok(cached) => {
                            Message::new(MessageType::Warning, "Using cached context...").print();
                            cached
                        }
                        Err(e) => Err(OrandaError::CachedContextReadError{ details: Box::new(e) })?
                    }
                } else {
                    Context::new(repo_url, config.artifacts.cargo_dist())?
                };
                    if config.artifacts.has_some() {
                        index = Some(Page::index_with_artifacts(&context, &layout_template, config)?);
                        if context.latest_dist_release.is_some()
                            || config.artifacts.package_managers.is_some()
                        {
                            let body = artifacts::page(&context, config)?;
                            let artifacts_page = Page::new_from_contents(
                                body,
                                "artifacts.html",
                                &layout_template,
                                config,
                            );
                            pages.push(artifacts_page);
                        }
                    }
                    if config.changelog {
                        let mut changelog_pages = Self::build_changelog_pages(&context, &layout_template, config)?;
                        pages.append(&mut changelog_pages);
                    }
                    if config.funding.is_some() {
                        let funding = Funding::new(config)?;
                        let body = funding::page(config, &funding)?;
                        let page = Page::new_from_contents(
                            body,
                            "funding.html",
                            &layout_template,
                            config,
                        );
                        pages.push(page);
                    }
                },
                None => Err(OrandaError::Other("You have indicated you want to use features that require a repository context. Please add a \"repository\" key and value to your project (such as a package.json or Cargo.toml) or oranda config (oranda.json).".to_string()))?
            }
        }

        pages.push(index.unwrap_or(Page::index(&layout_template, config)?));
        Ok(Site { pages })
    }

    fn needs_context(config: &Config) -> bool {
        config.artifacts.has_some() || config.changelog || config.funding.is_some()
    }

    fn build_additional_pages(
        files: &HashMap<String, String>,
        layout_template: &Layout,
        config: &Config,
    ) -> Result<Vec<Page>> {
        let mut pages = vec![];
        for file_path in files.values() {
            if page::source::is_markdown(file_path) {
                let additional_page = Page::new_from_file(file_path, layout_template, config)?;
                pages.push(additional_page)
            } else {
                let msg = format!(
                    "File {} in additional pages is not markdown and will be skipped",
                    file_path
                );
                Message::new(MessageType::Warning, &msg).print();
            }
        }
        Ok(pages)
    }

    fn build_changelog_pages(
        context: &Context,
        layout_template: &Layout,
        config: &Config,
    ) -> Result<Vec<Page>> {
        let mut pages = vec![];
        let changelog_html = changelog::build(context, config)?;
        let changelog_page =
            Page::new_from_contents(changelog_html, "changelog.html", layout_template, config);
        let changelog_releases = changelog::build_all(context, config)?;
        pages.push(changelog_page);
        for (name, content) in changelog_releases {
            let page = Page::new_from_contents(
                content,
                &format!("changelog/{}.html", name),
                layout_template,
                config,
            );
            pages.push(page);
        }
        Ok(pages)
    }

    pub fn copy_static(dist_dir: &Utf8Path, static_path: &str) -> Result<()> {
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        // We want to be able to rename dirs in the copy, this enables it
        options.copy_inside = true;
        fs_extra::copy_items(&[static_path], dist_dir, &options)?;

        Ok(())
    }

    pub fn write(self, config: &Config) -> Result<()> {
        let dist = Utf8PathBuf::from(&config.dist_dir);
        for page in self.pages {
            let filename_path = Utf8PathBuf::from(&page.filename);
            // Prepare to write a "pretty link" for pages that aren't index.html already. This essentially means that we rewrite
            // the page from "page.html" to "page/index.html", so that it can be loaded as "mysite.com/page" in the browser.
            let full_path: Utf8PathBuf = if !filename_path.ends_with("index.html") {
                // FIXME: Can we do anything BUT unwrap here? What's the smart way to deal with a missing filename path portion?
                let file_stem = filename_path.file_stem().unwrap();
                let parent = filename_path.parent().unwrap_or("".into());
                dist.join(parent).join(file_stem).join("index.html")
            } else {
                dist.join(filename_path)
            };
            LocalAsset::write_new_all(&page.contents, full_path)?;
        }
        if let Some(book_cfg) = &config.mdbook {
            mdbook::build_mdbook(
                &dist,
                book_cfg,
                &config.styles.theme(),
                &config.styles.syntax_theme(),
            )?;
        }
        if Path::new(&config.static_dir).exists() {
            Self::copy_static(&dist, &config.static_dir)?;
        }
        javascript::write_os_script(&dist)?;

        let additional_css = &config.styles.additional_css;
        if !additional_css.is_empty() {
            css::write_additional(additional_css, &dist)?;
        }

        Ok(())
    }

    pub fn clean_dist_dir(dist_path: &str) -> Result<()> {
        if Path::new(dist_path).exists() {
            std::fs::remove_dir_all(dist_path)?;
        }
        match std::fs::create_dir_all(dist_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(OrandaError::DistDirCreationError {
                dist_path: dist_path.to_string(),
                details: e,
            }),
        }
    }
}
