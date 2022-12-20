use crate::errors::*;
use grass::{Options, OutputStyle};
use std::fs::read_to_string;
use std::path::Path;

use crate::config::Config;

fn fetch_additional_css(config: &Config, css_options: Options) -> Result<String> {
    match Path::new(&config.additional_css).try_exists() {
        Ok(_) => {
            let additional_css_str = read_to_string(&config.additional_css)?;

            let additional_css =
                grass::from_string(format!("#oranda{{{}}}", additional_css_str), &css_options)?;

            Ok(additional_css)
        }
        Err(details) => Err(OrandaError::LocalAssetNotFound {
            asset: "Additional CSS".to_string(),
            origin_path: config.additional_css.to_string(),
            details: details.to_string(),
        }),
    }
}

fn fetch_remote_css(config: &Config) -> Result<String> {
    let mut css = String::from("");
    for url in &config.remote_styles {
        let resp = reqwest::blocking::get(url);
        match resp {
            Err(details) => {
                return Err(OrandaError::RemoteAssetRequestFailed {
                    url: url.to_string(),
                    asset: "Remote CSS".to_string(),
                    details: details.to_string(),
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
