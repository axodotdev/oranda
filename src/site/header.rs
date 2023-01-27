use crate::config::Config;
use crate::errors::*;
use axohtml::elements::{div, header, img, li, nav};
use axohtml::{html, text};
use std::path::Path;

fn get_logo(logo: String, config: &Config) -> Result<Box<img<String>>> {
    let fetched_logo = fetch_logo(&config.dist_dir, logo, &config.name);

    tokio::runtime::Handle::current().block_on(fetched_logo)
}

async fn fetch_logo(
    dist_dir: &str,
    origin_path: String,
    name: &String,
) -> Result<Box<img<String>>> {
    let copy_result = axoasset::copy(&origin_path, dist_dir).await?;

    let path_as_string = copy_result.strip_prefix(dist_dir)?.to_string_lossy();

    Ok(html!(<img src=path_as_string alt=name class="logo" />))
}

fn nav(pages: &[String], path_prefix: &Option<String>) -> Result<Box<nav<String>>> {
    let mut html: Vec<Box<li<String>>> = if let Some(prefix) = &path_prefix {
        let href = format!("/{}", prefix);
        vec![html!(<li><a href=href>"Home"</a></li>)]
    } else {
        vec![html!(<li><a href="/">"Home"</a></li>)]
    };
    for page in pages.iter() {
        let file_path = Path::new(page);
        let file_name = file_path
            .file_stem()
            .unwrap_or(file_path.as_os_str())
            .to_string_lossy();

        let href = if let Some(prefix) = &path_prefix {
            format!("/{}/{}.html", prefix, file_name)
        } else {
            format!("/{}.html", file_name)
        };

        html.extend(html!(<li><a href=href>{text!(file_name)}</a></li>));
    }
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

    let nav = if let Some(additional_pages) = &config.additional_pages {
        Some(nav(additional_pages, &config.path_prefix)?)
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
            <div class="icon" aria-hidden="true"/>
            {text!("Check out our GitHub")}
        </a>
    </div>
    ))
}
