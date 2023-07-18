use std::env;

use crate::errors::*;

use axoasset::{Asset, LocalAsset};
use camino::Utf8Path;
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

pub fn get_css_link(
    dist_dir: &str,
    path_prefix: &Option<String>,
    release_tag: &str,
) -> Result<String> {
    let filename = fetch_css(dist_dir, release_tag)?;
    Ok(crate::site::link::generate(path_prefix, &filename))
}

fn fetch_css(dist_dir: &str, release_tag: &str) -> Result<String> {
    match env::var("ORANDA_CSS") {
        Ok(path) => {
            let filename = "oranda.css".to_string();
            let msg = format!("Overriding oranda_css path with {}", &path);
            tracing::warn!("{}", &msg);
            LocalAsset::copy(&path, dist_dir)?;
            Ok(filename)
        }
        Err(_) => {
            let filename = format!("oranda-{release_tag}.css");
            let dest_path = Utf8Path::new(dist_dir).join(&filename);
            let oranda_css_response =
                tokio::runtime::Handle::current().block_on(fetch_oranda(release_tag))?;
            axoasset::LocalAsset::write_new(&oranda_css_response, dest_path)?;
            Ok(filename)
        }
    }
}

async fn fetch_oranda(release_tag: &str) -> Result<String> {
    let oranda_css_request =
        octolotl::request::ReleaseAsset::new("axodotdev", "oranda", release_tag, "oranda.css");
    Ok(octolotl::Request::send(&oranda_css_request, true)
        .await?
        .text()
        .await?)
}

pub fn write_additional_css(additional_css: &[String], dist_dir: &Utf8Path) -> Result<()> {
    let minified_css = concat_minify(additional_css)?;

    LocalAsset::write_new(&minified_css, dist_dir.join("custom.css"))?;
    Ok(())
}
