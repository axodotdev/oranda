use axohtml::dom::UnsafeTextNode;
use axohtml::elements::{div, section};
use axohtml::{dom::TextNode, html};
use axohtml::{text, unsafe_text};
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::errors::*;
use url::Url;

use super::markdown;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleasesApiResponse {
    pub url: String,
    pub assets_url: String,
    pub html_url: String,
    pub id: i64,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<Asset>,
    pub tarball_url: String,
    pub zipball_url: String,
    pub body: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub label: String,
    pub content_type: String,
    pub state: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}

pub fn tag_icon() -> Box<UnsafeTextNode<String>> {
    unsafe_text!("<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-6 h-6'>
    <path stroke-linecap='round' stroke-linejoin='round' d='M9.568 3H5.25A2.25 2.25 0 003 5.25v4.318c0 .597.237 1.17.659 1.591l9.581 9.581c.699.699 1.78.872 2.607.33a18.095 18.095 0 005.223-5.223c.542-.827.369-1.908-.33-2.607L11.16 3.66A2.25 2.25 0 009.568 3z' />
    <path stroke-linecap='round' stroke-linejoin='round' d='M6 6h.008v.008H6V6z' /></svg>")
}

pub fn build_page(config: &Config, repo: &String) -> Result<String> {
    let repo_parsed = Url::parse(repo.as_str())?;
    let parts = repo_parsed.path_segments().map(|c| c.collect::<Vec<_>>());
    if let Some(url_parts) = parts {
        let mut releases_html: Vec<Box<section<String>>> = vec![];
        let url = format!(
            "https://api.github.com/repos/{}/{}/releases",
            url_parts[0], url_parts[1]
        );
        let client = reqwest::blocking::Client::new();
        let rsp = client
            .get(&url)
            .header(USER_AGENT, "oranda")
            .send()?
            .json::<Vec<ReleasesApiResponse>>()?;

        for release in rsp.iter() {
            let body = match &release.body {
                Some(md) => markdown::to_html(md.to_string(), &config.syntax_theme)?,
                None => String::new(),
            };
            releases_html.extend(
                html!(<section class="release"><h2>{text!(&release.name)}</h2>
            <h5 class="flex items-center gap-2">{tag_icon()}{text!(&release.tag_name)}</h5>
            <div class="release-body">{unsafe_text!(body)}</div>
            </section>),
            )
        }

        Ok(html!(<div><h1>{text!("Releases")}</h1>{releases_html}</div>).to_string())
    } else {
        Err(OrandaError::Other(String::from(
            "Your repository url is incorrect, cannot create releases API url",
        )))
    }
}
