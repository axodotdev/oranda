use std::env;
use std::fs;

use crate::errors::*;
use crate::message::{Message, MessageType};

use axoasset::{Asset, LocalAsset};
use axohtml::elements::link;
use axohtml::html;
use camino::Utf8Path;
use minifier::css;

pub const LATEST_ORANDA_CSS: &str = "0.0.4";

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

pub fn build_oranda(
    dist_dir: &str,
    path_prefix: &Option<String>,
    oranda_css_version: &Option<String>,
) -> Result<Box<link<String>>> {
    let dist_dir = dist_dir;
    let oranda_version = match oranda_css_version {
        Some(version) => version,
        None => LATEST_ORANDA_CSS,
    };
    let filename = format!("oranda-v{oranda_version}.css");
    match env::var("ORANDA_CSS") {
        Ok(path) => {
            let msg = format!("Overriding oranda_css path with {}", &path);
            Message::new(MessageType::Warning, &msg).print();
            LocalAsset::copy(&path, dist_dir)?;
        }
        Err(_) => {
            let oranda_url = format!("https://octolotl.axodotdev.host/downloads/axodotdev/oranda/css-v{oranda_version}/oranda.css");
            let fetched_oranda =
                tokio::runtime::Handle::current().block_on(Asset::copy(&oranda_url, dist_dir))?;
            fs::rename(fetched_oranda, format!("{dist_dir}/{filename}"))?;
        }
    };
    let abs_path = crate::site::link::generate(path_prefix, &filename);
    Ok(html!(<link rel="stylesheet" href=abs_path></link>))
}

pub fn build_additional(path_prefix: &Option<String>) -> Box<link<String>> {
    let abs_path = crate::site::link::generate(path_prefix, "custom.css");
    html!(<link rel="stylesheet" href=abs_path></link>)
}

pub fn write_additional(additional_css: &[String], dist_dir: &Utf8Path) -> Result<()> {
    let minified_css = concat_minify(additional_css)?;

    LocalAsset::write_new(&minified_css, dist_dir.join("custom.css"))?;
    Ok(())
}
