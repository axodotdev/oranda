use crate::errors::*;
use axohtml::{html, text};
use std::path::Path;

use crate::config::Config;
use axohtml::elements::{header, img, li};

fn get_logo(config: &Config) -> Option<Result<Box<img<String>>>> {
    config
        .logo
        .as_ref()
        .map(|logo| fetch_logo(&config.dist_dir, logo.to_string(), &config.name))
}

fn fetch_logo(dist_dir: &str, origin_path: String, name: &String) -> Result<Box<img<String>>> {
    if Path::new(&origin_path).exists() {
        match axoasset::copy(&origin_path, dist_dir, "Logo") {
            Ok(path) => {
                let path_as_string = path.strip_prefix(dist_dir).unwrap().to_string_lossy();

                Ok(html!(<img src=path_as_string alt=name class="logo" />))
            }
            Err(_) => Err(OrandaError::Other(
                "There was a problem copying your logo".to_owned(),
            )),
        }
    } else {
        Err(OrandaError::FileNotFound {
            filedesc: "Logo".to_owned(),
            path: origin_path,
        })
    }
}

pub fn create(config: &Config) -> Box<header<String>> {
    // we want to unwrap here since we want the error from the logo functions to surface if there is one
    let logo = get_logo(config).map(|html| html.unwrap());

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

    html!(
        <header>
            {logo}
            <h1 class="title">{text!(&config.name)}</h1>
            {nav}
           
        </header>
    )
}
