use std::env;
use std::fs;

use crate::config::Config;
use crate::errors::*;
use crate::message::{Message, MessageType};

use axoasset::{Asset, LocalAsset};
use axohtml::elements::link;
use axohtml::html;
use minifier::css;

fn concat_minify(css_files: &[String]) -> Result<String> {
    let mut css = String::new();
    for file in css_files {
        let future = Asset::load_string(file);
        let unminified = tokio::runtime::Handle::current().block_on(future)?;
        let minified = match css::minify(&unminified) {
            Ok(css) => Ok(css),
            Err(e) => Err(OrandaError::Other(e.to_string())),
        };
        css = format!("{css}/* {file} */{minified}", minified = minified?);
    }

    Ok(css)
}

pub fn build_oranda(config: &Config) -> Result<Box<link<String>>> {
    let dist_dir = &config.dist_dir;
    match env::var("ORANDA_CSS") {
        Ok(path) => {
            let msg = format!("Overriding oranda_css path with {}", &path);
            Message::new(MessageType::Warning, &msg).print();
            LocalAsset::copy(&path, dist_dir)?;
        }
        Err(_) => {
            let fetched_oranda = tokio::runtime::Handle::current().block_on(Asset::copy(
                "https://github.com/axodotdev/oranda/releases/download/css-v0.0.2/oranda.css",
                dist_dir,
            ))?;
            let path = "oranda.css";
            fs::rename(fetched_oranda, format!("{dist_dir}/{path}"))?;
        }
    };
    let abs_path = crate::site::link::generate(&config.path_prefix, "oranda.css");
    Ok(html!(<link rel="stylesheet" href=abs_path></link>))
}

pub fn build_additional() -> Box<link<String>> {
    html!(<link rel="stylesheet" href="custom.css"></link>)
}

pub fn write_additional(additional_css: &[String], dist_dir: &str) -> Result<()> {
    let minified_css = concat_minify(additional_css)?;

    LocalAsset::write_new(&minified_css, "custom.css", dist_dir)?;
    Ok(())
}
