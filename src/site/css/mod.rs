use crate::errors::*;
use std::fs::read_to_string;
use std::path::Path;

use crate::config::Config;

fn fetch_additional_css(config: &Config) -> Result<String> {
    if !Path::new(&config.additional_css).exists() {
        return Err(OrandaError::FileNotFound {
            filedesc: "Additional CSS".to_string(),
            path: config.additional_css.to_owned(),
        });
    }

    let additional_css = read_to_string(&config.additional_css)?;
    
    Ok(additional_css)
}

fn fetch_remote_css(config: &Config) -> Result<String> {
    let mut css = String::from("");
    for url in &config.remote_styles {
        let resp = reqwest::blocking::get(url);
        match resp {
            Err(_) => {
                return Err(OrandaError::RequestFailed {
                    url: url.to_string(),
                    resource: String::from("Remote CSS"),
                });
            }
            Ok(additional) => {
                css = format!(
                    "{css}{additional}",
                    css = css,
                    additional = additional.text().unwrap()
                );
            }
        }
    }

    Ok(css)
}

pub fn build(config: &Config) -> Result<String> {
    let mut css = String::from("");
    if !config.additional_css.is_empty() {
        let additional_css = fetch_additional_css(config)?;

        css = format!("{css}{additional}", css = css, additional = additional_css);
    }

    if !config.remote_styles.is_empty() {
        let remote_css = fetch_remote_css(config)?;

        css = format!("{css}{remote_css}", css = css, remote_css = remote_css);
    }
    Ok(css)
}
