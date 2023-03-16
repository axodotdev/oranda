mod icons;

use crate::{errors::*, site::repo};
use axohtml::{dom::UnsafeTextNode, elements::li, html, text, types::SpacedList};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contents {
    pub name: String,
    pub content: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Funding {
    github: Option<Vec<String>>,
    patreon: Option<String>,
    open_collective: Option<String>,
    ko_fi: Option<String>,
    tidelift: Option<String>,
    community_bridge: Option<String>,
    liberapay: Option<String>,
    issuehunt: Option<String>,
    lfx_crowdfunding: String,
    custom: Option<Vec<String>>,
}

fn create_link(
    link: &str,
    icon: Box<UnsafeTextNode<String>>,
    site_name: &str,
) -> Box<axohtml::elements::a<String>> {
    let mut rels = SpacedList::new();
    rels.add("noopener");
    rels.add("noreferrer");
    html!(<a class="button secondary" href=link target="_blank" title={format!("Support us on {}", site_name)}rel=rels>{icon}</a>)
}

pub fn build_funding_html(funding_links: Funding) -> Vec<Box<li<String>>> {
    let mut html = vec![];
    if let Some(github) = funding_links.github {
        for link in github {
            let gh_link = format!("https://github.com/sponsors/{}", link);

            html.extend(html!(<li>{create_link(&gh_link,icons::get_github_icon(), "GitHub")}</li>))
        }
    }

    if let Some(patreon) = funding_links.patreon {
        let patreon_link = format!("https://www.patreon.com/{}", patreon);

        html.extend(
            html!(<li>{create_link(&patreon_link,icons::get_patreon_icon(), "Patreon")}</li>),
        )
    }

    if let Some(open_collective) = funding_links.open_collective {
        let open_collective_link = format!("https://opencollective.com/{}", open_collective);

        html.extend(
            html!(<li>{create_link(&open_collective_link,icons::get_open_collective_icon(), "Open Collective")}</li>),
        )
    }

    if let Some(ko_fi) = funding_links.ko_fi {
        let ko_fi_link = format!("https://ko-fi.com/{}", ko_fi);

        html.extend(html!(<li>{create_link(&ko_fi_link,icons::get_kofi_icon(), "Ko-fi")}</li>))
    }

    if let Some(tidelift) = funding_links.tidelift {
        let tidelift_link = format!("https://tidelift.com/subscription/pkg/{}", tidelift);

        html.extend(
            html!(<li>{create_link(&tidelift_link,icons::get_tidelif_icon(), "Tidelift")}</li>),
        )
    }

    if let Some(liberapay) = funding_links.liberapay {
        let liberapay_link = format!("https://liberapay.com/{}", liberapay);

        html.extend(
            html!(<li>{create_link(&liberapay_link,icons::get_liberapay_icon(), "Liberapay")}</li>),
        )
    }

    if let Some(issue_hunt) = funding_links.issuehunt {
        // their logo SUCKS need to find a way to minify that
        let issue_hunt_link = format!("https://issuehunt.io/r/{}", issue_hunt);

        html.extend(html!(<li>{create_link(&issue_hunt_link,icons::get_liberapay_icon(), "Issue Hunt")}</li>))
    }

    html
}

pub fn parse_funding_yaml(content: String) -> Result<Funding> {
    let string_yaml = &general_purpose::STANDARD
        .decode(content.replace('\n', ""))
        .unwrap();
    match std::str::from_utf8(string_yaml) {
        Ok(parsed_yaml) => {
            let deserialized_map = serde_yaml::from_str(parsed_yaml);

            match deserialized_map {
                Ok(yaml) => Ok(yaml),
                Err(e) => Err(OrandaError::GithubFundingParseError {
                    details: e.to_string(),
                }),
            }
        }
        Err(e) => Err(OrandaError::GithubFundingParseError {
            details: e.to_string(),
        }),
    }
}

pub fn fetch_funding_info(repo: &str) -> Result<String> {
    let url_parts = repo::parse(repo)?;
    let response = repo::fetch_funding_file(url_parts)?;
    match response.json::<Contents>() {
        Ok(contents) => {
            let funding_yaml = parse_funding_yaml(contents.content)?;
            let funding_html = build_funding_html(funding_yaml);
            Ok(html!(
                <div class="funding-wrapper">
                    <h3>{text!("Help fund this project")}</h3>
                    <ul class="funding-list">
                        {funding_html}
                    </ul>
                </div>
            )
            .to_string())
        }
        Err(e) => Err(OrandaError::GithubFundingParseError {
            details: e.to_string(),
        }),
    }
}
