use crate::errors::*;
use grass::OutputStyle;
use std::fs::read_to_string;
use std::path::Path;

use crate::config::Config;

pub fn build_css(config: &Config) -> Result<String> {
    let css_options = grass::Options::default().style(OutputStyle::Compressed);
    let mut css = grass::from_path("src/site/css/stylesheets/style.scss", &css_options)?;
    if !config.additional_css.is_empty() {
        if !Path::new(&config.additional_css).exists() {
            return Err(OrandaError::FileNotFound {
                filedesc: "Addicional CSS".to_string(),
                path: config.additional_css.to_owned(),
            });
        }

        let additional_css_str = read_to_string(&config.additional_css)?;

        let additional_css =
            grass::from_string(format!("#oranda{{{}}}", additional_css_str), &css_options)?;

        css = format!("{css}{additional}", css = css, additional = additional_css);
    }

    if !config.remote_styles.is_empty() {
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
                    )
                }
            }
        }
    }
    Ok(css)
}
