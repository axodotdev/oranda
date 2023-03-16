use axohtml::{dom::UnsafeTextNode, elements::li, html, types::SpacedList};

use crate::site::funding::icons;

use crate::site::funding::Funding;

pub fn create_link(
    link: &str,
    icon: Box<UnsafeTextNode<String>>,
    site_name: &str,
) -> Box<axohtml::elements::a<String>> {
    let mut rels = SpacedList::new();
    rels.add("noopener");
    rels.add("noreferrer");
    let title = format!("Support us on {}", site_name);
    html!(<a class="button secondary" href=link target="_blank" title=title rel=rels>{icon}</a>)
}

// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
pub fn build(funding_links: Funding) -> Vec<Box<li<String>>> {
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

    if let Some(lfx_crowdfunding) = funding_links.lfx_crowdfunding {
        let lfx_crowdfunding_link = format!(
            "https://crowdfunding.lfx.linuxfoundation.org/projects/{}",
            lfx_crowdfunding
        );

        html.extend(html!(<li>{create_link(&lfx_crowdfunding_link,icons::get_linux_icon(), "LFX Crowdfunding")}</li>))
    }

    if let Some(community_bridge) = funding_links.community_bridge {
        let community_bridge_link = format!(
            "https://crowdfunding.lfx.linuxfoundation.org/projects/{}",
            community_bridge
        );

        html.extend(html!(<li>{create_link(&community_bridge_link,icons::get_linux_icon(), "LFX Mentorship")}</li>))
    }

    if let Some(custom) = funding_links.custom {
        for link in custom {
            html.extend(html!(<li>{create_link(&link,icons::get_web_icon(), &link)}</li>))
        }
    }

    html
}
