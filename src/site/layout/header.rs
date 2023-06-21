use crate::config::{ArtifactsConfig, Config, FundingConfig, MdBookConfig};
use crate::errors::*;
use crate::message::{Message, MessageType};
use crate::site::{link, page};

use axoasset::Asset;
use axohtml::elements::{div, header, img, li, nav};
use axohtml::{html, text};
use indexmap::IndexMap;

fn get_logo(logo: String, config: &Config) -> Result<Box<img<String>>> {
    let fetched_logo = fetch_logo(&config.build.dist_dir, logo, &config.project.name);

    tokio::runtime::Handle::current().block_on(fetched_logo)
}

async fn fetch_logo(
    dist_dir: &str,
    origin_path: String,
    name: &String,
) -> Result<Box<img<String>>> {
    let copy_result = Asset::copy(&origin_path, dist_dir).await?;

    let path_as_string = copy_result.strip_prefix(dist_dir)?.to_string_lossy();

    Ok(html!(<img src=path_as_string alt=name class="logo" />))
}

fn nav(
    additional_pages: &IndexMap<String, String>,
    path_prefix: &Option<String>,
    artifacts: &ArtifactsConfig,
    md_book: &Option<MdBookConfig>,
    changelog: &bool,
    funding: &Option<FundingConfig>,
) -> Result<Box<nav<String>>> {
    Message::new(MessageType::Info, "Building nav...").print();
    let mut html: Vec<Box<li<String>>> = if let Some(prefix) = &path_prefix {
        let href = format!("/{}/", prefix);
        vec![html!(<li><a href=href>"Home"</a></li>)]
    } else {
        vec![html!(<li><a href="/">"Home"</a></li>)]
    };

    if !additional_pages.is_empty() {
        Message::new(MessageType::Info, "Found additional pages...").print();
        for (page_name, page_path) in additional_pages.iter() {
            if page::source::is_markdown(page_path) {
                let file_path = page::source::get_filename(page_path);

                if let Some(file_name) = file_path {
                    let href =
                        link::generate(path_prefix, &format!("{}/", file_name.to_string_lossy()));

                    html.extend(html!(<li><a href=href>{text!(page_name)}</a></li>));
                } else {
                    let msg = format!(
                        "Could not parse filename of file {} in additional pages and this file will be skipped",
                        page_path
                    );
                    Message::new(MessageType::Warning, &msg).print();
                }
            }
        }
    }

    if artifacts.has_some() {
        Message::new(MessageType::Info, "Adding artifacts page...").print();
        let href = link::generate(path_prefix, "artifacts/");
        html.extend(html!(<li><a href=href>{text!("Install")}</a></li>));
    };

    if md_book.is_some() {
        Message::new(MessageType::Info, "Adding book...").print();
        let href = if let Some(prefix) = &path_prefix {
            format!("/{}/{}/", prefix, "book")
        } else {
            format!("/{}/", "book")
        };
        html.extend(html!(<li><a href=href>{text!("Docs")}</a></li>));
    };

    if funding.is_some() {
        Message::new(MessageType::Info, "Adding funding page...").print();
        let href = if let Some(prefix) = &path_prefix {
            format!("/{}/{}/", prefix, "funding")
        } else {
            format!("/{}/", "funding")
        };
        html.extend(html!(<li><a href=href>{text!("Funding")}</a></li>));
    }

    if *changelog {
        Message::new(MessageType::Info, "Adding changelog...").print();
        let href = if let Some(prefix) = &path_prefix {
            format!("/{}/{}/", prefix, "changelog")
        } else {
            format!("/{}/", "changelog")
        };
        html.extend(html!(<li><a href=href>{text!("Changelog")}</a></li>));
    };

    Ok(html!(
        <nav class="nav">
            <ul>
                {html}
            </ul>
        </nav>
    ))
}

pub fn create(config: &Config) -> Result<Box<header<String>>> {
    let logo = if let Some(logo) = config.styles.logo.clone() {
        Some(get_logo(logo, config)?)
    } else {
        None
    };

    let nav = if !config.build.additional_pages.is_empty()
        || config.components.artifacts.has_some()
        || config.components.mdbook.is_some()
        || config.components.changelog
    {
        Some(nav(
            &config.build.additional_pages,
            &config.build.path_prefix,
            &config.components.artifacts,
            &config.components.mdbook,
            &config.components.changelog,
            &config.components.funding,
        )?)
    } else {
        None
    };
    Ok(html!(
        <header>
            {logo}
            <h1 class="title">{text!(&config.project.name)}</h1>
            {nav}
        </header>
    ))
}

pub fn repo_banner(config: &Config) -> Option<Box<div<String>>> {
    let repository = config.project.repository.as_ref()?;
    Some(html!(
    <div class="repo_banner">
        <a href=repository>
            <div class="github-icon" aria-hidden="true"/>
            {text!("Check out our GitHub")}
        </a>
    </div>
    ))
}
