use std::path::Path;

use axoasset::{Asset, LocalAsset};
use camino::{Utf8Path, Utf8PathBuf};
use indexmap::IndexMap;
use minijinja::context;

use crate::config::Config;
use crate::data::{funding::Funding, github::GithubRepo, workspaces, Context};
use crate::errors::*;

use crate::data::workspaces::WorkspaceData;
use crate::site::templates::Templates;
use layout::css;
pub use layout::javascript;
use page::Page;

pub mod artifacts;
pub mod changelog;
pub mod funding;
pub mod layout;
pub mod link;
pub mod markdown;
pub mod mdbook;
pub mod oranda_theme;
pub mod page;
pub mod templates;

#[derive(Debug)]
pub struct Site {
    pub workspace_data: Option<WorkspaceData>,
    pub pages: Vec<Page>,
}

impl Site {
    pub fn build_multi(workspace_config: &Config) -> Result<Vec<Site>> {
        tracing::info!("Workspace detected, gathering info...");
        // We assume the root path is wherever oranda-workspace.json is located (current dir)
        let root_path = Utf8PathBuf::from_path_buf(std::env::current_dir()?.canonicalize()?)
            .unwrap_or(Utf8PathBuf::new());

        let mut workspace_config_path = root_path.clone();
        workspace_config_path.push("oranda-workspace.json");
        let mut results = Vec::new();
        let members =
            workspaces::from_config(&workspace_config, &root_path, &workspace_config_path)?;
        tracing::info!("Building {} workspace member(s)...", members.len());
        for member in members {
            std::env::set_current_dir(&member.path)?;
            let mut site = Self::build_single(&member.config, Some(member.clone()))?;
            site.workspace_data = Some(member.clone());
            results.push(site);
            std::env::set_current_dir(&root_path)?;
        }

        Ok(results)
    }

    pub fn build_single(config: &Config, workspace: Option<WorkspaceData>) -> Result<Site> {
        let prefix = workspace.map(|w| w.slug);
        Self::clean_dist_dir(&config.build.dist_dir)?;
        let templates = Templates::new(config, &prefix)?;

        let mut pages = vec![];

        if !config.build.additional_pages.is_empty() {
            let mut additional_pages = Self::build_additional_pages(
                &config.build.additional_pages,
                &templates,
                config,
                &prefix,
            )?;
            pages.append(&mut additional_pages);
        }

        let mut index = None;
        let needs_context = Self::needs_context(config)?;
        Self::print_plan(config, &prefix);

        if needs_context {
            let mut context = match &config.project.repository {
                Some(repo_url) => Context::new_github(
                    repo_url,
                    &config.project,
                    config.components.artifacts.as_ref(),
                    &prefix,
                )?,
                None => Context::new_current(
                    &config.project,
                    config.components.artifacts.as_ref(),
                    &prefix,
                )?,
            };
            if config.components.artifacts_enabled() {
                if let Some(latest) = context.latest_mut() {
                    // Give especially nice treatment to the latest release and make
                    // its scripts easy to view (others get hotlinked and will just download)
                    latest.artifacts.make_scripts_viewable(config)?;

                    let template_context = artifacts::template_context(&context, config, &prefix)?;
                    index = Some(Page::new_from_both(
                        &config.project.readme_path,
                        "index.html",
                        &templates,
                        "index.html",
                        context!(artifacts => template_context),
                        config,
                    )?);
                    let artifacts_page = Page::new_from_template(
                        "artifacts.html",
                        &templates,
                        "artifacts.html",
                        &template_context,
                    )?;
                    pages.push(artifacts_page);
                }
            }
            if config.components.changelog {
                let mut changelog_pages =
                    Self::build_changelog_pages(&context, &templates, config)?;
                pages.append(&mut changelog_pages);
            }
            if let Some(funding_cfg) = &config.components.funding {
                let funding = Funding::new(funding_cfg, &config.styles)?;
                let context = funding::context(funding_cfg, &funding)?;
                let page =
                    Page::new_from_template("funding.html", &templates, "funding.html", context)?;
                pages.push(page);
            }
        }

        pages.push(index.unwrap_or(Page::new_from_both(
            &config.project.readme_path,
            "index.html",
            &templates,
            "index.html",
            context!(),
            config,
        )?));
        Ok(Site {
            pages,
            workspace_data: None,
        })
    }

    pub fn get_workspace_config() -> Result<Option<Config>> {
        let path = Utf8PathBuf::from("./oranda-workspace.json");
        if path.exists() {
            let workspace_config = Config::build_workspace_root(&path)?;
            Ok(Some(workspace_config))
        } else {
            Ok(None)
        }
    }

    fn needs_context(config: &Config) -> Result<bool> {
        Ok(config.components.artifacts_enabled()
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

    fn print_plan(config: &Config, prefix: &Option<String>) {
        let mut planned_components = Vec::new();
        if config.components.artifacts_enabled() {
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
            tracing::info!(prefix, "Building components: {}", joined);
        }
    }

    fn build_additional_pages(
        files: &IndexMap<String, String>,
        templates: &Templates,
        config: &Config,
        prefix: &Option<String>,
    ) -> Result<Vec<Page>> {
        let mut pages = vec![];
        for file_path in files.values() {
            if page::source::is_markdown(file_path) {
                let additional_page = Page::new_from_markdown(file_path, templates, config)?;
                pages.push(additional_page)
            } else {
                let msg = format!(
                    "File {} in additional pages is not markdown and will be skipped",
                    file_path
                );
                tracing::warn!(prefix, "{}", &msg);
            }
        }
        Ok(pages)
    }

    fn build_changelog_pages(
        context: &Context,
        templates: &Templates,
        config: &Config,
    ) -> Result<Vec<Page>> {
        let mut pages = vec![];
        let index_context = changelog::index_context(context, config)?;
        let changelog_page = Page::new_from_template(
            "changelog.html",
            templates,
            "changelog_index.html",
            index_context,
        )?;
        pages.push(changelog_page);
        for release in context.releases.iter() {
            let single_context = changelog::single_context(release, config);
            let page = Page::new_from_template(
                &format!("changelog/{}.html", single_context.version_tag),
                templates,
                "changelog_single.html",
                context!(release => single_context),
            )?;
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

    pub fn write(self, config: Option<&Config>) -> Result<()> {
        let config = if self.workspace_data.is_some() {
            &self.workspace_data.as_ref().unwrap().config
        } else {
            config.unwrap()
        };
        let dist = Utf8PathBuf::from(&config.build.dist_dir);
        for page in self.pages {
            let filename_path = Utf8PathBuf::from(&page.filename);
            // Prepare to write a "pretty link" for pages that aren't index.html already.
            // This essentially means that we rewrite the page from "page.html" to
            // "page/index.html", so that it can be loaded as "mysite.com/page" in the browser.
            let full_path: Utf8PathBuf = if !filename_path.ends_with("index.html") {
                // Surely we can't we do anything BUT unwrap here? A file without a name is a mess.
                let file_stem = filename_path.file_stem().expect("missing file_stem???");
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
        if let Some(origin_path) = config.styles.favicon.as_ref() {
            let copy_result_future = Asset::copy(origin_path, &config.build.dist_dir[..]);
            tokio::runtime::Handle::current().block_on(copy_result_future)?;
        }
        if Path::new(&config.build.static_dir).exists() {
            Self::copy_static(&dist, &config.build.static_dir)?;
        }
        javascript::write_os_script(&dist)?;

        let additional_css = &config.styles.additional_css;
        if !additional_css.is_empty() {
            css::write_additional_css(additional_css, &dist)?;
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
