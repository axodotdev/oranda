use std::env;
use std::fs;

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

pub fn build_oranda(dist_dir: &str) -> Result<Box<link<String>>> {
    match env::var("ORANDA_CSS") {
        Ok(path) => {
            let msg = format!("Overriding oranda_css path with {}", &path);
            Message::new(MessageType::Warning, &msg).print();
            LocalAsset::copy(&path, dist_dir)?;
        }
        Err(_) => {
            let fetched_oranda = tokio::runtime::Handle::current().block_on(Asset::copy(
                "https://github.com/axodotdev/oranda/releases/download/css-v0.0.0/oranda.css",
                dist_dir,
            ))?;
            let path = "oranda.css";
            fs::rename(fetched_oranda, format!("{dist_dir}/{path}"))?;
        }
    };
    Ok(html!(<link rel="stylesheet" href="oranda.css"></link>))
}

pub fn build_additional() -> Box<link<String>> {
    html!(<link rel="stylesheet" href="custom.css"></link>)
}

pub fn write_additional(additional_css: &[String], dist_dir: &str) -> Result<()> {
    let minified_css = concat_minify(additional_css)?;
    let css_path = format!("{}/custom.css", dist_dir);

    let asset = LocalAsset::new(&css_path, minified_css.as_bytes().to_vec());
    asset.write(dist_dir)?;
    Ok(())
}
