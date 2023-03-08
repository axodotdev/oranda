mod release;
mod types;
use std::vec;

use axohtml::elements::{div, li, section};
use axohtml::html;
use axohtml::text;
use reqwest::header::USER_AGENT;

use crate::config::Config;
use crate::errors::*;
use url::Url;

fn build_prerelease_toggle(releases: Vec<types::ReleasesApiResponse>) -> Option<Box<div<String>>> {
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

pub fn fetch_releases(repo: &str) -> Result<Vec<types::ReleasesApiResponse>> {
    let repo_parsed = match Url::parse(repo) {
        Ok(parsed) => Ok(parsed),
        Err(parse_error) => Err(OrandaError::RepoParseError {
            repo: repo.to_string(),
            details: parse_error.to_string(),
        }),
    };
    let binding = repo_parsed?;
    let parts = binding.path_segments().map(|c| c.collect::<Vec<_>>());
    if let Some(url_parts) = parts {
        let url = format!(
            "https://octolotl.axodotdev.host/releases/{}/{}",
            url_parts[0], url_parts[1]
        );
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let header = format!("oranda-{}", VERSION);

        let releases = reqwest::blocking::Client::new()
            .get(url)
            .header(USER_AGENT, header)
            .send()?
            .json::<Vec<types::ReleasesApiResponse>>()?;

        let releases_non_drafts = releases
            .iter()
            .filter(|&r| !r.draft)
            .map(|f| f.to_owned())
            .collect();
        Ok(releases_non_drafts)
    } else {
        Err(OrandaError::RepoParseError {
            repo: binding.to_string(),
            details: "This URL is not structured the expected way, expected more segments-"
                .to_owned(),
        })
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

        releases_html.extend(release::build_release(
            release,
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
