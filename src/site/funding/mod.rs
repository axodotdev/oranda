mod icons;

use crate::config::Config;
use crate::data::funding::{load_funding_docs, load_funding_file, Funding};
use crate::errors::{OrandaError, Result};
use axohtml::dom::UnsafeTextNode;
use axohtml::types::SpacedList;
use axohtml::{html, text, unsafe_text};
use serde::{Deserialize, Serialize};

/// Contents of the HTTP response. Serialized from JSON.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct FundingResponse {
    pub name: String,
    pub content: String,
}

/// Generate the standalone funding page.
pub fn page(funding: &Funding) -> Result<String> {
    let mut list_html = vec![];
    if let Some(github) = &funding.github {
        for link in github {
            let gh_link = format!("https://github.com/sponsors/{}", link);
            list_html
                .extend(html!(<li>{create_link(&gh_link, icons::get_github_icon(), "GitHub")}</li>))
        }
    }

    if let Some(patreon) = &funding.patreon {
        let patreon_link = format!("https://patreon.com/{}", patreon);
        list_html.extend(
            html!(<li>{create_link(&patreon_link, icons::get_patreon_icon(), "Patreon")}</li>),
        )
    }

    if let Some(open_collective) = &funding.open_collective {
        let oc_link = format!("https://opencollective.com/{}", open_collective);
        list_html.extend(html!(<li>{create_link(&oc_link, icons::get_open_collective_icon(), "Open Collective")}</li>))
    }

    if let Some(kofi) = &funding.ko_fi {
        let kofi_link = format!("https://ko-fi.com/{}", kofi);
        list_html.extend(html!(<li>{create_link(&kofi_link, icons::get_kofi_icon(), "Ko-fi")}</li>))
    }

    if let Some(tidelift) = &funding.tidelift {
        let tidelift_link = format!("https://tidelift.com/subscription/pkg/{}", tidelift);
        list_html.extend(
            html!(<li>{create_link(&tidelift_link, icons::get_tidelift_icon(), "Tidelift")}</li>),
        )
    }

    if let Some(community_bridge) = &funding.community_bridge {
        let cb_link = format!(
            "https://crowdfunding.lfx.linuxfoundation.org/projects/{}",
            community_bridge
        );
        list_html.extend(
            html!(<li>{create_link(&cb_link, icons::get_linux_icon(), "LFX Mentorship")}</li>),
        )
    }

    if let Some(liberapay) = &funding.liberapay {
        let liberapay_link = format!("https://liberapay.com/{}", liberapay);
        list_html.extend(html!(<li>{create_link(&liberapay_link, icons::get_liberapay_icon(), "Liberapay")}</li>))
    }

    if let Some(issuehunt) = &funding.issuehunt {
        let issuehunt_link = format!("https://issuehunt.com/r/{}", issuehunt);
        // FIXME: Get an issuehunt icon from somewhere
        list_html.extend(
            html!(<li>{create_link(&issuehunt_link, icons::get_web_icon(), "IssueHunt")}</li>),
        )
    }

    if let Some(custom) = &funding.custom {
        for link in custom {
            list_html.extend(html!(<li>{create_link(link, icons::get_web_icon(), link)}</li>))
        }
    }

    Ok(html!(
        <div class="funding-wrapper">
            <h1>{text!("Help fund this project!")}</h1>
            {unsafe_text!(funding.docs_content.clone().unwrap_or("".into()))}
            <ul class="funding-list">
                {list_html}
            </ul>
        </div>
    )
    .to_string())
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
