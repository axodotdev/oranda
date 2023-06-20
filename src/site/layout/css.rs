use std::env;

use crate::errors::*;
use crate::message::{Message, MessageType};

use axoasset::{Asset, LocalAsset};
use axohtml::elements::link;
use axohtml::html;
use camino::Utf8Path;
use minifier::css;

pub const LATEST_ORANDA_CSS: &str = "0.0.6";

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
    let version = match oranda_css_version {
        Some(version) => version,
        None => LATEST_ORANDA_CSS,
    };
    let filename = fetch_css(dist_dir, version)?;
    let abs_path = crate::site::link::generate(path_prefix, &filename);
    Ok(html!(<link rel="stylesheet" href=abs_path></link>))
}

fn fetch_css(dist_dir: &str, version: &str) -> Result<String> {
    match env::var("ORANDA_CSS") {
        Ok(path) => {
            let filename = "oranda.css".to_string();
            let msg = format!("Overriding oranda_css path with {}", &path);
            Message::new(MessageType::Warning, &msg).print();
            LocalAsset::copy(&path, dist_dir)?;
            Ok(filename)
        }
        Err(_) => {
            let filename = format!("oranda-v{version}.css");
            let dest_path = Utf8Path::new(dist_dir).join(&filename);
            let oranda_css_response =
                tokio::runtime::Handle::current().block_on(fetch_oranda(version))?;
            axoasset::LocalAsset::write_new(&oranda_css_response, dest_path)?;
            Ok(filename)
        }
    }
}

async fn fetch_oranda(version: &str) -> Result<String> {
    let tag = format!("css-v{version}");
    let oranda_css_request =
        octolotl::request::ReleaseAsset::new("axodotdev", "oranda", &tag, "oranda.css");
    Ok(octolotl::Request::send(&oranda_css_request, true)
        .await?
        .text()
        .await?)
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
