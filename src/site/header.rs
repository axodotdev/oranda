use crate::config::Config;
use crate::errors::*;
use axohtml::elements::{header, img, li};
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

pub fn create(config: &Config) -> Result<Box<header<String>>> {
    let logo = if let Some(logo) = config.logo.clone() {
        Some(get_logo(logo, config)?)
    } else {
        None
    };

    let nav = match config.additional_pages.as_ref() {
        Some(pages) => {
            let mut html: Vec<Box<li<String>>> = vec![html!(<li><a href="/">"Home"</a></li>)];
            for page in pages.iter() {
                let path = Path::new(page);
                let file_name = path
                    .file_stem()
                    .unwrap_or(path.as_os_str())
                    .to_string_lossy();
                let path = format!("/{}", file_name);
                html.extend(html!(<li><a href=path>{text!(file_name)}</a></li>));
            }
            Some(html!(
                <nav class="nav">
                    <ul>
                        {html}
                    </ul>
                </nav>
            ))
        }
        None => None,
    };

    Ok(html!(
        <header>
            {logo}
            <h1 class="title">{text!(&config.name)}</h1>
            {nav}
        </header>
    ))
}
