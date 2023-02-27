mod single_release;
mod types;
use std::vec;

use axohtml::elements::{li, section};
use axohtml::html;
use axohtml::text;
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
        let mut releases_nav: Vec<Box<li<String>>> = vec![];
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
            let classnames = if release.prerelease {
                "pre-release hidden"
            } else {
                ""
            };
            let link = format!("#{}", &release.tag_name);
            releases_nav.extend(
                html!(<li class=classnames><a href=link>{text!(&release.tag_name)}</a></li>),
            )
        }

        Ok(html!(
            <div>
                <h1>{text!("Releases")}</h1>
                <div class="prereleases-toggle">
                <div class="flex h-6 items-center">
                  <input id="show-prereleases" type="checkbox" />
                </div>
                <div class="ml-3">
                  <label for="show-prereleases">{text!("Show prereleases")}</label>
                </div>
              </div>
                <div class="releases-wrapper">
                <nav class="releases-nav">

                <ul>
                    {releases_nav}
                </ul>
            </nav>
            <div>{releases_html}</div>
                </div>
            </div>
        )
        .to_string())
    } else {
        Err(OrandaError::Other(String::from(
            "Your repository url is incorrect, cannot create releases API url",
        )))
    }
}
