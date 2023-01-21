use crate::config::Config;
use crate::errors::*;
use axohtml::elements::{link, meta};
use axohtml::html;
use css_minify::optimizations::{Level, Minifier};
use std::fs::File;
use std::io::Write;

fn concat_minify_css(css_links: Vec<String>) -> Result<String> {
    let mut css = String::new();
    for url in css_links {
        let css_future = axoasset::load_string(url.as_str());

        let css_unminified = tokio::runtime::Handle::current().block_on(css_future)?;
        let minified_css = Minifier::default().minify(css_unminified.as_str(), Level::Three)?;
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
pub fn fetch_css(config: &Config) -> Result<Box<link<String>>> {
    const FRINGE_VERSION: &str = "0.0.7";
    let FRINGE_HREF = format!(
        "https://www.unpkg.com/@axodotdev/fringe@{}/themes/",
        FRINGE_VERSION
    );
    let minified_css = concat_minify_css(vec![
        format!("{}/axo-oranda.css", FRINGE_HREF),
        format!("{}/fringe-output.css", FRINGE_HREF),
        format!("{}/theme-output.css", FRINGE_HREF),
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

// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
pub fn create_social_cards(config: &Config) -> Vec<Box<meta<String>>> {
    let mut html = vec![];
    match config.social.as_ref() {
        Some(social) => {
            if let Some(image) = social.image.as_ref() {
                html.extend(html!(<meta name="twitter:card" content="summary_large_image"/>));

                html.extend(html!(<meta property="og:image" content=image />));
            };
            if let Some(image_alt) = social.image_alt.as_ref() {
                html.extend(html!(<meta property="og:image:alt" content=image_alt />));
            }

            if let Some(twitter_account) = social.twitter_account.as_ref() {
                html.extend(html!(<meta name="twitter:creator" content=twitter_account/>));
                html.extend(html!(<meta name="twitter:site" content=twitter_account/>));
            };

            Some(())
        }

        None => None,
    };

    html
}

pub fn get_favicon(favicon: String, dist_dir: String) -> Result<Box<link<String>>> {
    let copy_result_future = axoasset::copy(&favicon, &dist_dir[..]);
    let copy_result = tokio::runtime::Handle::current().block_on(copy_result_future)?;

    let path_as_string = copy_result.strip_prefix(dist_dir)?.to_string_lossy();

    Ok(html!(<link rel="icon" href=path_as_string />))
}

// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
pub fn create_meta_tags(config: &Config) -> Vec<Box<meta<String>>> {
    let mut social_meta = create_social_cards(config);
    let description = &config.description;
    let mut html = vec![
        html!(<meta charset="utf-8" />),
        html!(<meta name="viewport" content="width=device-width, initial-scale=1.0" />),
        html!(<meta name="description" content=description />),
        html!(<meta name="description" content=description />),
        html!(<meta property="og:description" content=description/>),
        html!(<meta property="og:type" content="website" />),
        html!(<meta property="og:title" content=&config.name />),
    ];

    html.append(&mut social_meta);

    html
}
