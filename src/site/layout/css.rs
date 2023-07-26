use std::{env, sync::RwLock};

use crate::errors::*;

use axoasset::{Asset, LocalAsset};
use camino::Utf8Path;
use minifier::css;

static CSS_CACHE: RwLock<Vec<CssItem>> = RwLock::new(Vec::new());

struct CssItem {
    release_tag: String,
    contents: String,
}

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

            // Do we already have this value cached?
            let cache_val = {
                let cache = CSS_CACHE.read().expect("CSS Cache should not be poisoned");
                cache
                    .iter()
                    .find(|elem| elem.release_tag.as_str() == release_tag)
                    .map(|elem| elem.contents.clone())
            };

            let oranda_css_response = if let Some(c) = cache_val {
                // Yes, we do!
                c
            } else {
                // Nope, sure don't. Get it, and if we are successful, store it for next time.
                let fresh =
                    tokio::runtime::Handle::current().block_on(fetch_oranda(release_tag))?;

                let mut cache = CSS_CACHE.write().expect("CSS Cache should not be poisoned");
                cache.push(CssItem {
                    release_tag: release_tag.to_string(),
                    contents: fresh.clone(),
                });
                fresh
            };

            axoasset::LocalAsset::write_new_all(&oranda_css_response, dest_path)?;
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
