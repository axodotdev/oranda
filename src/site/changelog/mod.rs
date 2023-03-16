use std::vec;

mod github_release;
pub use github_release::GithubRelease;

use axohtml::elements::{div, li, section};
use axohtml::html;
use axohtml::text;
use reqwest::header::USER_AGENT;

use crate::config::Config;
use crate::errors::*;

use crate::site::repo;

fn build_prerelease_toggle(releases: Vec<GithubRelease>) -> Option<Box<div<String>>> {
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

pub fn fetch_releases(repo: &str) -> Result<Vec<GithubRelease>> {
    let url_parts = repo::parse(repo)?;

    let url = format!(
        "https://octolotl.axodotdev.host/releases/{}/{}",
        url_parts.owner, url_parts.repo
    );
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let header = format!("oranda-{}", VERSION);

    let releases_response = reqwest::blocking::Client::new()
        .get(url)
        .header(USER_AGENT, header)
        .send()?;

    parse_response(releases_response)
}

fn parse_response(response: reqwest::blocking::Response) -> Result<Vec<GithubRelease>> {
    match response.error_for_status() {
        Ok(r) => match r.json() {
            Ok(releases) => Ok(releases),
            Err(e) => Err(OrandaError::GithubReleaseParseError {
                details: e.to_string(),
            }),
        },
        Err(e) => Err(OrandaError::GithubReleasesFetchError {
            details: e.to_string(),
        }),
    }
}

pub fn build_page(config: &Config, repo: &str) -> Result<String> {
    let releases = fetch_releases(repo)?;
    let mut releases_html: Vec<Box<section<String>>> = vec![];
    let mut releases_nav: Vec<Box<li<String>>> = vec![];
    for release in releases.iter() {
        let classnames = if release.prerelease {
            "pre-release hidden"
        } else {
            ""
        };

        let link = format!("#{}", &release.tag_name);

        releases_html.extend(release.build(
            &config.syntax_theme,
            &config.version,
            &config.path_prefix,
        )?);
        releases_nav
            .extend(html!(<li class=classnames><a href=link>{text!(&release.tag_name)}</a></li>))
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
}
