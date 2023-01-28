use crate::config::Config;
use crate::errors::*;

use axohtml::elements::link;
use axohtml::html;
use minifier::css::minify;

fn concat_minify_css(css_links: Vec<String>) -> Result<String> {
    let mut css = String::new();
    for url in css_links {
        let css_future = axoasset::load_string(url.as_str());

        let css_unminified = tokio::runtime::Handle::current().block_on(css_future)?;
        let minified_css = minify(css_unminified.as_str()).unwrap();
        css = format!(
            "{css}/* {url} */{minified_css}",
            css = css,
            url = url,
            minified_css = minified_css
        );
    }

    Ok(css)
}

// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
pub async fn fetch_fringe(config: &Config) -> Result<Box<link<String>>> {
    const FRINGE_VERSION: &str = "0.0.8";
    let fringe_href = format!(
        "https://www.unpkg.com/@axodotdev/fringe@{}/themes/",
        FRINGE_VERSION
    );
    let minified_css = concat_minify_css(vec![
        format!("{}/fringe-output.css", fringe_href),
        format!("{}/theme-output.css", fringe_href),
    ])?;
    let css_filename = format!("fringe@{}.css", FRINGE_VERSION);
    let css_path = format!("{}/{}", config.dist_dir, css_filename);

    let asset = axoasset::new(&css_path, minified_css.into())?;
    axoasset::write(asset, &config.dist_dir).await?;

    Ok(html!(<link rel="stylesheet" href=css_filename></link>))
}

pub fn fetch_additional(config: &Config) -> Result<Option<Box<link<String>>>> {
    if config.additional_css.is_empty() {
        return Ok(None);
    }

    let minified_css = concat_minify_css(config.additional_css.clone())?;
    let css_path = format!("{}/custom.css", &config.dist_dir);

    let asset = axoasset::new(&css_path, minified_css.into())?;
    axoasset::write(asset, &config.dist_dir).await?;

    Ok(Some(
        html!(<link rel="stylesheet" href="custom.css"></link>),
    ))
}
