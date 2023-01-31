use std::path::Path;

use crate::config::Config;
use crate::errors::*;
use crate::site::artifacts;
use crate::site::layout;

use axohtml::{html, unsafe_text};

pub fn build(config: &Config, content: String, is_index: bool) -> Result<String> {
    let artifacts_header = artifacts::build_header(config)?;
    let content = if is_index {
        html!(<div>{artifacts_header}{unsafe_text!(content)}</div>)
    } else {
        html!(<div>{unsafe_text!(content)}</div>)
    };

    let doc = layout::build(config, content, is_index)?;
    Ok(doc)
}

pub fn get_html_file_name(file: &String, config: &Config) -> Result<String> {
    let file_name = if file == &config.readme_path {
        "index.html".to_string()
    } else {
        let file_path = Path::new(file).file_stem();

        match file_path {
            None => {
                return Err(OrandaError::FileNotFound {
                    filedesc: "Additional File".to_string(),
                    path: file.to_string(),
                });
            }
            Some(p) => format!("{}.html", p.to_str().unwrap()),
        }
    };

    Ok(file_name)
}
