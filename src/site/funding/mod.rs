mod icons;

use crate::config::Config;
use crate::data::funding::{load_funding_docs, load_funding_file};
use crate::errors::{OrandaError, Result};
use crate::site::markdown::to_html;
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

/// Contents of the FUNDING.yml file. Needs to follow GitHub's spec, since we serialize from it,
/// the most accurate resource for this seems to be here: <https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/displaying-a-sponsor-button-in-your-repository#about-funding-files>
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Funding {
    pub github: Option<Vec<String>>,
    pub patreon: Option<String>,
    pub open_collective: Option<String>,
    pub ko_fi: Option<String>,
    pub tidelift: Option<String>,
    pub community_bridge: Option<String>,
    pub liberapay: Option<String>,
    pub issuehunt: Option<String>,
    pub custom: Option<Vec<String>>,
    /// Content read from the optional Markdown file
    #[serde(skip)]
    pub docs_content: Option<String>,
}

impl Funding {
    /// Creates a new Funding struct by attempting to read from the FUNDING.yml, and the docs file.
    pub fn new(config: &Config) -> Result<Self> {
        let mut funding = match load_funding_file() {
            Ok(Some(res)) => Ok(Self::parse_response(res)?),
            Ok(None) => Ok(Self::default()),
            Err(e) => Err(e),
        }?;
        if let Ok(Some(res)) = load_funding_docs() {
            let html = to_html(&res, &config.styles.syntax_theme)?;
            funding.docs_content = Some(html);
        }

        Ok(funding)
    }

    /// Generate the standalone funding page.
    pub fn page(&self) -> Result<String> {
        let mut list_html = vec![];
        if let Some(github) = &self.github {
            for link in github {
                let gh_link = format!("https://github.com/sponsors/{}", link);
                list_html.extend(
                    html!(<li>{Self::create_link(&gh_link, icons::get_github_icon(), "GitHub")}</li>),
                )
            }
        }

        if let Some(patreon) = &self.patreon {
            let patreon_link = format!("https://patreon.com/{}", patreon);
            list_html.extend(html!(<li>{Self::create_link(&patreon_link, icons::get_patreon_icon(), "Patreon")}</li>))
        }

        if let Some(open_collective) = &self.open_collective {
            let oc_link = format!("https://opencollective.com/{}", open_collective);
            list_html.extend(html!(<li>{Self::create_link(&oc_link, icons::get_open_collective_icon(), "Open Collective")}</li>))
        }

        if let Some(kofi) = &self.ko_fi {
            let kofi_link = format!("https://ko-fi.com/{}", kofi);
            list_html.extend(
                html!(<li>{Self::create_link(&kofi_link, icons::get_kofi_icon(), "Ko-fi")}</li>),
            )
        }

        if let Some(tidelift) = &self.tidelift {
            let tidelift_link = format!("https://tidelift.com/subscription/pkg/{}", tidelift);
            list_html.extend(html!(<li>{Self::create_link(&tidelift_link, icons::get_tidelift_icon(), "Tidelift")}</li>))
        }

        if let Some(community_bridge) = &self.community_bridge {
            let cb_link = format!(
                "https://crowdfunding.lfx.linuxfoundation.org/projects/{}",
                community_bridge
            );
            list_html.extend(html!(<li>{Self::create_link(&cb_link, icons::get_linux_icon(), "LFX Mentorship")}</li>))
        }

        if let Some(liberapay) = &self.liberapay {
            let liberapay_link = format!("https://liberapay.com/{}", liberapay);
            list_html.extend(html!(<li>{Self::create_link(&liberapay_link, icons::get_liberapay_icon(), "Liberapay")}</li>))
        }

        if let Some(issuehunt) = &self.issuehunt {
            let issuehunt_link = format!("https://issuehunt.com/r/{}", issuehunt);
            // FIXME: Get an issuehunt icon from somewhere
            list_html.extend(html!(<li>{Self::create_link(&issuehunt_link, icons::get_web_icon(), "IssueHunt")}</li>))
        }

        if let Some(custom) = &self.custom {
            for link in custom {
                list_html
                    .extend(html!(<li>{Self::create_link(link, icons::get_web_icon(), link)}</li>))
            }
        }

        Ok(html!(
            <div class="funding-wrapper">
                <h1>{text!("Help fund this project!")}</h1>
                {unsafe_text!(self.docs_content.clone().unwrap_or("".into()))}
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

    fn parse_response(contents: String) -> Result<Self> {
        let deserialized_map = serde_yaml::from_str(&contents);
        match deserialized_map {
            Ok(yaml) => Ok(yaml),
            Err(e) => Err(OrandaError::GithubFundingParseError {
                details: e.to_string(),
            }),
        }
    }
}
