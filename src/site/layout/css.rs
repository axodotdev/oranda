use crate::errors::*;

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

pub fn build_fringe() -> Box<link<String>> {
    const FRINGE_VERSION: &str = "0.0.10";
    let _css_file_name = format!("fringe@{}.css", FRINGE_VERSION);

    html!(<link rel="stylesheet" href="http://localhost:45365/axo-oranda.css"></link>)
}

pub fn write_fringe(dist_dir: &str) -> Result<()> {
    const FRINGE_VERSION: &str = "0.0.10";
    let css_file_name = format!("fringe@{}.css", FRINGE_VERSION);
    let fringe_href = format!(
        "https://www.unpkg.com/@axodotdev/fringe@{}/themes/",
        FRINGE_VERSION
    );
    let minified_css = concat_minify(&[
        format!("{}/fringe-output.css", fringe_href),
        format!("{}/theme-output.css", fringe_href),
    ])?;

    let css_path = format!("{}/{}", dist_dir, css_file_name);

    let asset = LocalAsset::new(&css_path, minified_css.as_bytes().to_vec());
    asset.write(dist_dir)?;
    Ok(())
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
