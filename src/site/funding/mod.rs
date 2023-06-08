mod icons;

use crate::config::Config;
use crate::data::funding::{Funding, FundingContent, FundingType};
use crate::errors::Result;
use axohtml::dom::UnsafeTextNode;
use axohtml::elements::{div, li};
use axohtml::types::SpacedList;
use axohtml::{html, text, unsafe_text};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Contents of the HTTP response. Serialized from JSON.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct FundingResponse {
    pub name: String,
    pub content: String,
}

/// Generate the standalone funding page.
pub fn page(config: &Config, funding: &Funding) -> Result<String> {
    let mut funding_items = funding.content.clone();
    // We've already made sure that we can unwrap on all of these `Option`s
    let unwrapped_config = config.funding.as_ref().unwrap();
    let preferred_html = if unwrapped_config.preferred_funding.is_some() {
        let preferred = unwrapped_config.preferred_funding.as_ref().unwrap();
        // Remove the preferred item from the rest of the list
        funding_items.remove(preferred);
        preferred_funding_section(preferred.clone(), funding.content.clone())
    } else {
        None
    };
    let regular_html = create_funding_list(funding_items);
    Ok(html!(
        <div class="funding-wrapper">
            <h1>{text!("Help fund this project!")}</h1>
            {preferred_html}
            {unsafe_text!(funding.docs_content.clone().unwrap_or("".into()))}
            <ul class="funding-list">
                {regular_html}
            </ul>
        </div>
    )
    .to_string())
}

fn preferred_funding_section(
    preferred: FundingType,
    funding: HashMap<FundingType, FundingContent>,
) -> Option<Box<div<String>>> {
    if let Some(element) = funding.get(&preferred).cloned() {
        let mut hashmap = HashMap::new();
        hashmap.insert(preferred, element);
        Some(html!(
        <div>
            <ul class="funding-list preferred-funding-list">
                {create_funding_list(hashmap)}
            </ul>
        </div>))
    } else {
        None
    }
}

#[allow(clippy::vec_box)]
fn create_funding_list(funding: HashMap<FundingType, FundingContent>) -> Vec<Box<li<String>>> {
    let mut list_html = vec![];
    if let Some(FundingContent::Multiple(github)) = &funding.get(&FundingType::Github) {
        for link in github {
            let gh_link = format!("https://github.com/sponsors/{}", link);
            list_html
                .extend(html!(<li>{create_link(&gh_link, icons::get_github_icon(), "GitHub")}</li>))
        }
    }

    if let Some(FundingContent::One(patreon)) = &funding.get(&FundingType::Patreon) {
        let patreon_link = format!("https://patreon.com/{}", patreon);
        list_html.extend(
            html!(<li>{create_link(&patreon_link, icons::get_patreon_icon(), "Patreon")}</li>),
        )
    }

    if let Some(FundingContent::One(open_collective)) = &funding.get(&FundingType::OpenCollective) {
        let oc_link = format!("https://opencollective.com/{}", open_collective);
        list_html.extend(html!(<li>{create_link(&oc_link, icons::get_open_collective_icon(), "Open Collective")}</li>))
    }

    if let Some(FundingContent::One(kofi)) = &funding.get(&FundingType::KoFi) {
        let kofi_link = format!("https://ko-fi.com/{}", kofi);
        list_html.extend(html!(<li>{create_link(&kofi_link, icons::get_kofi_icon(), "Ko-fi")}</li>))
    }

    if let Some(FundingContent::One(tidelift)) = &funding.get(&FundingType::Tidelift) {
        let tidelift_link = format!("https://tidelift.com/subscription/pkg/{}", tidelift);
        list_html.extend(
            html!(<li>{create_link(&tidelift_link, icons::get_tidelift_icon(), "Tidelift")}</li>),
        )
    }

    if let Some(FundingContent::One(community_bridge)) = &funding.get(&FundingType::CommunityBridge)
    {
        let cb_link = format!(
            "https://crowdfunding.lfx.linuxfoundation.org/projects/{}",
            community_bridge
        );
        list_html.extend(
            html!(<li>{create_link(&cb_link, icons::get_linux_icon(), "LFX Mentorship")}</li>),
        )
    }

    if let Some(FundingContent::One(liberapay)) = &funding.get(&FundingType::Liberapay) {
        let liberapay_link = format!("https://liberapay.com/{}", liberapay);
        list_html.extend(html!(<li>{create_link(&liberapay_link, icons::get_liberapay_icon(), "Liberapay")}</li>))
    }

    if let Some(FundingContent::One(issuehunt)) = &funding.get(&FundingType::Issuehunt) {
        let issuehunt_link = format!("https://issuehunt.com/r/{}", issuehunt);
        // FIXME: Get an issuehunt icon from somewhere
        list_html.extend(
            html!(<li>{create_link(&issuehunt_link, icons::get_web_icon(), "IssueHunt")}</li>),
        )
    }

    if let Some(FundingContent::Multiple(custom)) = &funding.get(&FundingType::Custom) {
        for link in custom {
            list_html.extend(html!(<li>{create_link(link, icons::get_web_icon(), link)}</li>))
        }
    }

    list_html
}

/// Creates a link element to be used in the funding page.
fn create_link(
    link: &str,
    icon: Box<UnsafeTextNode<String>>,
    site_name: &str,
) -> Box<axohtml::elements::a<String>> {
    let mut rels = SpacedList::new();
    rels.add("noopener");
    rels.add("noreferrer");
    let title = &format!("Support us on {}", site_name);
    html!(<a href=link target="_blank" title=title rel=rels>
            <button class="button secondary">
                {icon}
            </button>
            {unsafe_text!(title)}
        </a>)
}
