use axohtml::elements::{div, li, section};
use axohtml::html;
use axohtml::{text, unsafe_text};
use chrono::DateTime;

use crate::config::Config;
use crate::data::cargo_dist;
use crate::data::github::GithubRelease;
use crate::errors::*;
use crate::site::{icons, markdown};

pub fn build(config: &Config) -> Result<String> {
    if let Some(repo) = &config.repository {
        let releases = GithubRelease::fetch_all(repo)?;
        let mut releases_html: Vec<Box<section<String>>> = vec![];
        let mut releases_nav: Vec<Box<li<String>>> = vec![];
        for release in releases.iter() {
            let classnames = if release.prerelease {
                "pre-release hidden"
            } else {
                ""
            };

            let link = format!("#{}", &release.tag_name);

            releases_html.extend(build_page_preview(release, config)?);
            releases_nav.extend(
                html!(<li class=classnames><a href=link>{text!(&release.tag_name)}</a></li>),
            )
        }

        Ok(html!(
            <div>
                <h1>{text!("Releases")}</h1>
                <div class="releases-wrapper">
                    <nav class="releases-nav">
                        {build_prerelease_toggle(releases)}
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
        Err(OrandaError::Other(
            "repository required for changelog feature".to_string(),
        ))
    }
}

pub fn build_page_preview(
    release: &GithubRelease,
    config: &Config,
) -> Result<Box<section<String>>> {
    let tag_name = release.tag_name.clone();
    let title = release.name.clone().unwrap_or(tag_name.clone());

    let id: axohtml::types::Id = axohtml::types::Id::new(tag_name.clone());
    let formatted_date = match DateTime::parse_from_rfc3339(&release.published_at) {
        Ok(date) => date.format("%b %e %Y at %R UTC").to_string(),
        Err(_) => release.published_at.to_owned(),
    };

    let classnames = if release.prerelease {
        "release pre-release hidden"
    } else {
        "release"
    };
    let link = format!("#{}", &tag_name);
    let body = build_release_body(release, config)?;

    Ok(html!(
    <section class=classnames>
        <h2 id=id><a href=link>{text!(title)}</a></h2>
        <div class="release-info">
            <span class="flex items-center gap-2">
                {icons::tag()}{text!(tag_name)}
            </span>
            <span class="flex items-center gap-2">
                {icons::date()}{text!(&formatted_date)}
            </span>
        </div>
        <div class="release-body mb-6">
            {unsafe_text!(body)}
        </div>
    </section>
    ))
}

fn build_release_body(release: &GithubRelease, config: &Config) -> Result<String> {
    let contents = if release.has_dist_manifest() {
        cargo_dist::fetch_release(config)?
            .manifest
            .announcement_changelog
            .unwrap_or(String::new())
    } else {
        release.body.clone().unwrap_or(String::new())
    };

    markdown::to_html(&contents, &config.syntax_theme)
}

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
