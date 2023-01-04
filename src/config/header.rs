use crate::errors::*;
use axohtml::{html, text};
use std::path::Path;

use crate::config::Config;
use axohtml::elements::{header, img, li};

fn get_logo(config: &Config) -> Option<Result<Box<img<String>>>> {
    config
        .logo
        .to_owned()
        .map(|logo_origin_path| fetch_logo(&config.dist_dir, logo_origin_path))
}

fn fetch_logo(dist_dir: &str, origin_path: String) -> Result<Box<img<String>>> {
    if Path::new(&origin_path).exists() {
        let new_path = match axoasset::copy(&origin_path, "Logo", &dist_dir) {
            Ok(path) => {
                let path_as_string = path.to_str().unwrap();
                println!("OMG HERE {:?}", path_as_string);
                return Ok(html!(<img src=path_as_string />));
            }
            Err(_) => Err(OrandaError::Other(
                "There was a problem copying your logo".to_owned(),
            )),
        };

        new_path
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
    let logo = get_logo(config);
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

    Some(html!(<header>{nav}<h1>{text!(&config.name)}</h1></header>))
}
