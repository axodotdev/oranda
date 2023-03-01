mod single_release;
mod types;
use std::vec;

use axohtml::elements::{div, li, section};
use axohtml::html;
use axohtml::text;
use reqwest::header::USER_AGENT;

use crate::config::Config;
use crate::errors::*;
use url::Url;

use crate::site::changelog::single_release::build_single_release;
use crate::site::changelog::types::ReleasesApiResponse;

fn build_prerelease_toggle(releases: Vec<ReleasesApiResponse>) -> Option<Box<div<String>>> {
    let has_pre_releases = releases.iter().any(|release| release.prerelease);

    if has_pre_releases {
        Some(html!(
    <div class="prereleases-toggle">
        <div class="flex h-6 items-center">
            <input id="show-prereleases" type="checkbox" />
        </div>
        <div class="ml-3">
            <label for="show-prereleases">{text!("Show prereleases")}</label>
        </div>
    </div>))
    } else {
        None
    }
}

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

        let releases = reqwest::blocking::Client::new()
            .get(&url)
            .header(USER_AGENT, "oranda")
            .send()?
            .json::<Vec<ReleasesApiResponse>>()?;

        for release in releases.iter() {
            let classnames = if release.prerelease {
                "pre-release hidden"
            } else {
                ""
            };

            let link = format!("#{}", &release.tag_name);

            releases_html.extend(build_single_release(
                release,
                &config.syntax_theme,
                &config.version,
                &config.path_prefix,
            )?);
            releases_nav.extend(
                html!(<li class=classnames><a href=link>{text!(&release.tag_name)}</a></li>),
            )
        }

        Ok(html!(
            <div>
                <h1>{text!("Releases")}</h1>
                {build_prerelease_toggle(releases)}
                <div class="releases-wrapper">
                    <nav class="releases-nav">
                        <ul>
                            {releases_nav}
                        </ul>
                    </nav>
                    <div class="releases-list">{releases_html}</div>
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
