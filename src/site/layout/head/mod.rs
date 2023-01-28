mod css;

use crate::config::Config;
use crate::errors::*;
use axohtml::elements;
use axohtml::html;

pub struct Head {
    pub name: String,
    pub metatags: Vec<Box<elements::meta<String>>>,
    pub fringe_css: Box<elements::link<String>>,
    pub additional_css: Option<Box<elements::link<String>>>,
    pub favicon: Option<Box<elements::link<String>>>,
}

impl Head {
    pub async fn build(config: Config) -> Result<Self> {
        Ok(Head {
            name: config.name,
            metatags: build_metatags(&config),
            favicon: if let Some(favicon) = config.favicon.clone() {
                Some(get_favicon(favicon, config.dist_dir.clone())?)
            } else {
                None
            },
            additional_css: css::fetch_additional(&config)?,
            fringe_css: css::fetch_fringe(&config).await?,
        })
    }
}

// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
fn build_socialcards(config: &Config) -> Vec<Box<elements::meta<String>>> {
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

fn get_favicon(favicon: String, dist_dir: String) -> Result<Box<elements::link<String>>> {
    let copy_result_future = axoasset::copy(&favicon, &dist_dir[..]);
    let copy_result = tokio::runtime::Handle::current().block_on(copy_result_future)?;

    let path_as_string = copy_result.strip_prefix(dist_dir)?.to_string_lossy();

    Ok(html!(<link rel="icon" href=path_as_string />))
}

// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
fn build_metatags(config: &Config) -> Vec<Box<elements::meta<String>>> {
    let mut social_meta = build_socialcards(config);
    let description = &config.description;
    let mut html = vec![
        html!(<meta charset="utf-8" />),
        html!(<meta name="viewport" content="width=device-width, initial-scale=1.0" />),
        html!(<meta name="description" content=description />),
        html!(<meta name="description" content=description />),
        html!(<meta property="og:description" content=description/>),
        html!(<meta property="og:type" content="website" />),
        html!(<meta property="og:title" content=&config.name />),
        html!(<meta http-equiv="Permissions-Policy" content="interest-cohort=()"/>),
    ];

    html.append(&mut social_meta);
    let homepage = if let Some(homepage) = config.homepage {
        let metatag = html!(
          <meta property="og:url" content=homepage/>
        );
        html.push(metatag);
    };

    html
}
