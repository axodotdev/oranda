use crate::config::Config;
use crate::errors::*;
use axohtml::elements::link;
use axohtml::html;
use minifier::css::minify;
use std::fs::File;
use std::io::Write;

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
pub fn fetch_fringe_css(config: &Config) -> Result<Box<link<String>>> {
    const FRINGE_VERSION: &str = "0.0.7";
    let fringe_href = format!(
        "https://www.unpkg.com/@axodotdev/fringe@{}/themes/",
        FRINGE_VERSION
    );
    let minified_css = concat_minify_css(vec![
        format!("{}/fringe-output.css", fringe_href),
        format!("{}/theme-output.css", fringe_href),
    ])?;
    let css_file_name = format!("fringe@{}.css", FRINGE_VERSION);
    let css_path = format!("{}/{}", &config.dist_dir, css_file_name);

    let mut css_file = File::create(css_path)?;
    css_file.write_all(minified_css.as_bytes())?;

    Ok(html!(<link rel="stylesheet" href=css_file_name></link>))
}

pub fn fetch_additional_css(config: &Config) -> Result<Option<Box<link<String>>>> {
    if config.additional_css.is_empty() {
        return Ok(None);
    }

    let minified_css = concat_minify_css(config.additional_css.clone())?;
    let css_path = format!("{}/custom.css", &config.dist_dir);

    let mut css_file = File::create(css_path)?;
    css_file.write_all(minified_css.as_bytes())?;

    Ok(Some(
        html!(<link rel="stylesheet" href="custom.css"></link>),
    ))
}
