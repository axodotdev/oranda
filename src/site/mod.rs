use std::path::Path;

use axoasset::LocalAsset;
use camino::{Utf8Path, Utf8PathBuf};
use indexmap::IndexMap;

use crate::config::Config;
use crate::data::{funding::Funding, github::GithubRepo, Context};
use crate::errors::*;
use crate::message::{Message, MessageType};

pub use layout::javascript;
use layout::{css, Layout};
use page::Page;

pub mod artifacts;
pub mod changelog;
pub mod funding;
pub mod icons;
pub mod layout;
pub mod link;
pub mod markdown;
pub mod mdbook;
pub mod oranda_theme;
pub mod page;

#[derive(Debug)]
pub struct Site {
    pages: Vec<Page>,
}

impl Site {
    pub fn build(config: &Config) -> Result<Site> {
        Self::clean_dist_dir(&config.build.dist_dir)?;

        let mut pages = vec![];
        let layout_template = Layout::new(config)?;

        if !config.build.additional_pages.is_empty() {
            let mut additional_pages = Self::build_additional_pages(
                &config.build.additional_pages,
                &layout_template,
                config,
            )?;
            pages.append(&mut additional_pages);
        }

        let mut index = None;
        let needs_context = Self::needs_context(config)?;
        Self::print_plan(config);

        if needs_context {
            let mut context = match &config.project.repository {
                Some(repo_url) => Context::new_github(
                    repo_url,
                    &config.project,
                    config.components.artifacts.as_ref(),
                )?,
                None => {
                    Context::new_current(&config.project, config.components.artifacts.as_ref())?
                }
            };
            // FIXME: change the config so that you can set `artifacts: false` and disable this?
            let artifacts_enabled = config
                .components
                .artifacts
                .as_ref()
                .map(|a| a.has_some())
                .unwrap_or(false);
            if context.latest().is_some() && artifacts_enabled {
                context
                    .latest_mut()
                    .unwrap()
                    .artifacts
                    .make_scripts_viewable(config)?;
                index = Some(Page::index_with_artifacts(
                    &context,
                    &layout_template,
                    config,
                )?);
                let body = artifacts::page(&context, config)?;
                let artifacts_page =
                    Page::new_from_contents(body, "artifacts.html", &layout_template, config);
                pages.push(artifacts_page);
            }
            if config.components.changelog {
                let mut changelog_pages =
                    Self::build_changelog_pages(&context, &layout_template, config)?;
                pages.append(&mut changelog_pages);
            }
            if let Some(funding_cfg) = &config.components.funding {
                let funding = Funding::new(&config.build.path_prefix, funding_cfg, &config.styles)?;
                let body = funding::page(config, &funding)?;
                let page = Page::new_from_contents(body, "funding.html", &layout_template, config);
                pages.push(page);
            }
        }

        pages.push(index.unwrap_or(Page::index(&layout_template, config)?));
        Ok(Site { pages })
    }

    fn needs_context(config: &Config) -> Result<bool> {
        Ok(config
            .components
            .artifacts
            .as_ref()
            .map(|a| a.has_some())
            .unwrap_or(false)
            || config.components.changelog
            || config.components.funding.is_some()
            || Self::has_repo_and_releases(&config.project.repository)?)
    }

    fn has_repo_and_releases(repo_config: &Option<String>) -> Result<bool> {
        if let Some(repo) = repo_config {
            GithubRepo::from_url(repo)?.has_releases()
        } else {
            Ok(false)
        }
    }

    fn print_plan(config: &Config) {
        let mut planned_components = Vec::new();
        if config.components.artifacts.is_some() {
            planned_components.push("artifacts");
        }
        if config.components.changelog {
            planned_components.push("changelog");
        }
        if config.components.funding.is_some() {
            planned_components.push("funding");
        }
        if config.components.mdbook.is_some() {
            planned_components.push("mdbook");
        }

        let joined = planned_components
            .iter()
            .fold(String::new(), |acc, component| {
                if acc.is_empty() {
                    component.to_string()
                } else {
                    format!("{}, {}", acc, component)
                }
            });
        if !joined.is_empty() {
            Message::new(
                MessageType::Info,
                &format!("Building components: {}", joined),
            )
            .print();
        }
    }

    fn build_additional_pages(
        files: &IndexMap<String, String>,
        layout_template: &Layout,
        config: &Config,
    ) -> Result<Vec<Page>> {
        let mut pages = vec![];
        for file_path in files.values() {
            if page::source::is_markdown(file_path) {
                let additional_page =
                    Page::new_from_file_with_dir(file_path, layout_template, config)?;
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
        let dist = Utf8PathBuf::from(&config.build.dist_dir);
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
        if let Some(book_cfg) = &config.components.mdbook {
            mdbook::build_mdbook(
                &dist,
                book_cfg,
                &config.styles.theme,
                &config.styles.syntax_theme,
            )?;
        }
        if Path::new(&config.build.static_dir).exists() {
            Self::copy_static(&dist, &config.build.static_dir)?;
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
