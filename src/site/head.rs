use crate::config::Config;
use crate::errors::*;
use axohtml::elements::{link, meta};
use axohtml::html;

// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
pub fn fetch_additional_css(config: &Config) -> Result<Vec<Box<link<String>>>> {
    let mut css = vec![];
    for url in &config.additional_css {
        let additional_css_future = axoasset::copy(url, &config.dist_dir);

        let additional_path = tokio::runtime::Handle::current().block_on(additional_css_future)?;
        let path_as_string = additional_path
            .strip_prefix(&config.dist_dir)?
            .to_string_lossy();

        css.push(html!(<link rel="stylesheet" href=path_as_string.to_string()></link>))
    }
    Ok(css)
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
