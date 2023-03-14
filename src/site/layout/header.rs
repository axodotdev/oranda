use std::collections::HashMap;

use crate::config::artifacts::Artifacts;
use crate::config::Config;
use crate::errors::*;
use crate::message::{Message, MessageType};
use crate::site::{link, page};

use axoasset::Asset;
use axohtml::elements::{div, header, img, li, nav};
use axohtml::{html, text};

fn get_logo(logo: String, config: &Config) -> Result<Box<img<String>>> {
    let fetched_logo = fetch_logo(&config.dist_dir, logo, &config.name);

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
    additional_pages: &Option<HashMap<String, String>>,
    path_prefix: &Option<String>,
    artifacts: &Option<Artifacts>,
    md_book: &Option<String>,
    changelog: &bool,
) -> Result<Box<nav<String>>> {
    let mut html: Vec<Box<li<String>>> = if let Some(prefix) = &path_prefix {
        let href = format!("/{}", prefix);
        vec![html!(<li><a href=href>"Home"</a></li>)]
    } else {
        vec![html!(<li><a href="/">"Home"</a></li>)]
    };

    if let Some(pages) = additional_pages {
        for (page_name, page_path) in pages.iter() {
            if page::source::is_markdown(page_path) {
                let file_path = page::source::get_filename(page_path);

                if let Some(file_name) = file_path {
                    let href = link::generate(
                        path_prefix,
                        format!("{}.html", file_name.to_string_lossy()),
                    );

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

    if let Some(artifact) = artifacts {
        if artifact.cargo_dist.is_some() {
            let href = link::generate(path_prefix, String::from("artifacts.html"));
            html.extend(html!(<li><a href=href>{text!("Install")}</a></li>));
        }
    };

    if md_book.is_some() {
        let href = if let Some(prefix) = &path_prefix {
            format!("/{}/{}/", prefix, "book")
        } else {
            format!("/{}/", "book")
        };
        html.extend(html!(<li><a href=href>{text!("Docs")}</a></li>));
    };

    if *changelog {
        let href = if let Some(prefix) = &path_prefix {
            format!("/{}/{}", prefix, "changelog.html")
        } else {
            format!("/{}", "changelog.html")
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
    let logo = if let Some(logo) = config.logo.clone() {
        Some(get_logo(logo, config)?)
    } else {
        None
    };

    let nav = if config.additional_pages.is_some()
        || config.artifacts.is_some()
        || config.md_book.is_some()
    {
        Some(nav(
            &config.additional_pages,
            &config.path_prefix,
            &config.artifacts,
            &config.md_book,
            &config.changelog,
        )?)
    } else {
        None
    };
    Ok(html!(
        <header>
            {logo}
            <h1 class="title">{text!(&config.name)}</h1>
            {nav}
        </header>
    ))
}

pub fn repo_banner(config: &Config) -> Option<Box<div<String>>> {
    let repository = config.repository.as_ref()?;
    Some(html!(
    <div class="repo_banner">
        <a href=repository>
            <div class="github-icon" aria-hidden="true"/>
            {text!("Check out our GitHub")}
        </a>
    </div>
    ))
}
