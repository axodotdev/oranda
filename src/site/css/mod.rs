use crate::errors::*;
use grass::{Options, OutputStyle};
use std::fs::read_to_string;
use std::path::Path;

use crate::config::Config;

fn fetch_additional_css(config: &Config, css_options: Options) -> Result<String> {
    if !Path::new(&config.additional_css).exists() {
        return Err(OrandaError::FileNotFound {
            filedesc: "Additional CSS".to_string(),
            path: config.additional_css.to_owned(),
        });
    }

    let additional_css_str = read_to_string(&config.additional_css)?;

    let additional_css =
        grass::from_string(format!("#oranda{{{}}}", additional_css_str), &css_options)?;

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

    Ok(css.to_string())
}

pub fn build(config: &Config) -> Result<String> {
    let css_options = grass::Options::default().style(OutputStyle::Compressed);
    let mut css = grass::from_path("src/site/css/stylesheets/style.scss", &css_options)?;
    if !config.additional_css.is_empty() {
        let additional_css = fetch_additional_css(config, css_options)?;

        css = format!("{css}{additional}", css = css, additional = additional_css);
    }

    if !config.remote_styles.is_empty() {
        let remote_css = fetch_remote_css(config)?;

        css = format!("{css}{remote_css}", css = css, remote_css = remote_css);
    }
    Ok(css)
}
