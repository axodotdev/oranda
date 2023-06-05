mod icons;

use crate::config::Config;
use crate::data::github::GithubRepo;
use crate::errors::{OrandaError, Result};
use axohtml::dom::UnsafeTextNode;
use axohtml::types::SpacedList;
use axohtml::{html, text};
use base64::engine::general_purpose;
use base64::Engine;
use serde::{Deserialize, Serialize};

/// Contents of the HTTP response. Serialized from JSON.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct FundingResponse {
    pub name: String,
    pub content: String,
}

/// Contents of the FUNDING.yml file. Needs to follow GitHub's spec, since we serialize from it,
/// the most accurate resource for this seems to be here: https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/displaying-a-sponsor-button-in-your-repository#about-funding-files
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Funding {
    pub github: Option<Vec<String>>,
    pub patreon: Option<String>,
    pub open_collective: Option<String>,
    pub ko_fi: Option<String>,
    pub tidelift: Option<String>,
    pub community_bridge: Option<String>,
    pub otechie: Option<String>,
    pub liberapay: Option<String>,
    pub issuehunt: Option<String>,
    pub custom: Option<Vec<String>>,
}

impl Funding {
    /// Creates a new Funding struct by pulling from the GitHub repository specified in the
    /// configuration. Assumes that a repository is set (i.e. unwraps on the repo config key),
    /// so check for existence beforehand.
    pub fn new(config: &Config) -> Result<Self> {
        let repo = GithubRepo::from_url(&config.repository.clone().unwrap())?;
        match repo.fetch_funding_yaml() {
            Ok(Some(res)) => Ok(Self::parse_response(res)?),
            Ok(None) => Ok(Self::default()),
            Err(e) => Err(e),
        }
    }

    /// Generate the standalone funding page.
    pub fn page(&self) -> Result<String> {
        let mut html = vec![];
        dbg!(self);
        if let Some(github) = &self.github {
            for link in github {
                let gh_link = format!("https://github.com/sponsors/{}", link);
                html.extend(
                    html!(<li>{Self::create_link(&gh_link, icons::get_github_icon(), "GitHub")}</li>),
                )
            }
        }

        if let Some(patreon) = &self.patreon {
            let patreon_link = format!("https://patreon.com/{}", patreon);
            html.extend(html!(<li>{Self::create_link(&patreon_link, icons::get_patreon_icon(), "Patreon")}</li>))
        }

        if let Some(open_collective) = &self.open_collective {
            let oc_link = format!("https://opencollective.com/{}", open_collective);
            html.extend(html!(<li>{Self::create_link(&oc_link, icons::get_open_collective_icon(), "Open Collective")}</li>))
        }

        if let Some(kofi) = &self.ko_fi {
            let kofi_link = format!("https://ko-fi.com/{}", kofi);
            html.extend(
                html!(<li>{Self::create_link(&kofi_link, icons::get_kofi_icon(), "Ko-fi")}</li>),
            )
        }

        if let Some(tidelift) = &self.tidelift {
            let tidelift_link = format!("https://tidelift.com/subscription/pkg/{}", tidelift);
            html.extend(html!(<li>{Self::create_link(&tidelift_link, icons::get_tidelift_icon(), "Tidelift")}</li>))
        }

        if let Some(community_bridge) = &self.community_bridge {
            let cb_link = format!(
                "https://crowdfunding.lfx.linuxfoundation.org/projects/{}",
                community_bridge
            );
            html.extend(html!(<li>{Self::create_link(&cb_link, icons::get_linux_icon(), "LFX Mentorship")}</li>))
        }

        if let Some(otechie) = &self.otechie {
            let otechie_link = format!("https://otechie.com/{}", otechie);
            // Potential FIXME: Grab an Otechie SVG icon from somewhere (where?)
            html.extend(html!(<li>{Self::create_link(&otechie_link, icons::get_web_icon(), "Otechie")}</li>))
        }

        if let Some(liberapay) = &self.liberapay {
            let liberapay_link = format!("https://liberapay.com/{}", liberapay);
            html.extend(html!(<li>{Self::create_link(&liberapay_link, icons::get_liberapay_icon(), "Liberapay")}</li>))
        }

        if let Some(issuehunt) = &self.issuehunt {
            let issuehunt_link = format!("https://issuehunt.com/r/{}", issuehunt);
            // FIXME: Get an issuehunt icon from somewhere
            html.extend(html!(<li>{Self::create_link(&issuehunt_link, icons::get_web_icon(), "IssueHunt")}</li>))
        }

        if let Some(custom) = &self.custom {
            for link in custom {
                html.extend(html!(<li>{Self::create_link(link, icons::get_web_icon(), link)}</li>))
            }
        }

        Ok(html!(
            <div class="funding-wrapper">
                <h1>{text!("Help fund this project!")}</h1>
                <ul class="funding-list">
                    {html}
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
        let title = format!("Support us on {}", site_name);
        html!(<a class="button secondary" href=link target="_blank" title=title rel=rels>{icon}</a>)
    }

    fn parse_response(res: FundingResponse) -> Result<Self> {
        let string_yaml = &general_purpose::STANDARD
            .decode(res.content.replace('\n', ""))
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
}
