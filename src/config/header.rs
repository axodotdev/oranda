use crate::errors::*;
use axohtml::{html, text};
use std::path::Path;
use syntect::html;

use crate::config::Config;
use axohtml::elements::{header, img, li};

fn get_logo(config: &Config) -> Option<Result<Box<img<String>>>> {
    match &config.logo {
        None => None,
        Some(logo) => Some(fetch_logo(&config.dist_dir, logo.to_string(), &config.name)),
    }
}

fn fetch_logo(dist_dir: &str, origin_path: String, name: &String) -> Result<Box<img<String>>> {
    if Path::new(&origin_path).exists() {
        match axoasset::copy(&origin_path, &dist_dir, "Logo") {
            Ok(path) => {
                let path_as_string = path.strip_prefix(&dist_dir).unwrap().to_string_lossy();

                return Ok(html!(<img src=path_as_string alt=name/>));
            }
            Err(_) => Err(OrandaError::Other(
                "There was a problem copying your logo".to_owned(),
            )),
        }
    } else {
        Err(OrandaError::FileNotFound {
            filedesc: "Logo".to_owned(),
            path: origin_path.to_string(),
        })
    }
}

pub fn create_header(config: &Config) -> Option<Box<header<String>>> {
    if config.no_header {
        return None;
    }
    let logo = get_logo(config).unwrap();
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
            <nav>
                <ul>
                     {html}
                </ul>
            </nav>
            ))
        }
        None => None,
    };
    Some(html!(
    <header>
        {nav}
        <h1>{text!(&config.name)}</h1>
        {logo}
    </header>
    ))
}
