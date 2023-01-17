use std::path::Path;

use axohtml::{html, text};

use crate::config::Config;
use axohtml::elements::{header, li};

pub fn create_header(config: &Config) -> Option<Box<header<String>>> {
    if config.no_header {
        return None;
    }
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

    Some(html!(<header><h1 class="text-center">{text!(&config.name)}</h1>{nav}</header>))
}
