mod single_release;
mod types;
use axohtml::dom::UnsafeTextNode;
use axohtml::elements::{html, section};
use axohtml::html;
use axohtml::{text, unsafe_text};
use reqwest::header::USER_AGENT;

use crate::config::Config;
use crate::errors::*;
use url::Url;

use self::single_release::build_single_release;
use self::types::ReleasesApiResponse;

pub fn build_page(config: &Config, repo: &str) -> Result<String> {
    let repo_parsed = Url::parse(repo)?;
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
            releases_html.extend(build_single_release(release, &config.syntax_theme)?);
        }

        Ok(html!(
            <div>
                <h1>{text!("Releases")}</h1>
                {releases_html}
            </div>
        )
        .to_string())
    } else {
        Err(OrandaError::Other(String::from(
            "Your repository url is incorrect, cannot create releases API url",
        )))
    }
}
