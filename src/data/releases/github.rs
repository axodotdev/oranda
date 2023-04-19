use crate::config::Config;
use crate::data::artifacts::cargo_dist;
use crate::errors::*;
use crate::site::markdown;
use axohtml::dom::UnsafeTextNode;
use axohtml::elements::section;
use axohtml::html;
use axohtml::{text, unsafe_text};
use chrono::DateTime;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubRelease {
    pub url: String,
    pub assets_url: String,
    pub html_url: String,
    pub id: i64,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<GithubReleaseAsset>,
    pub tarball_url: String,
    pub zipball_url: String,
    pub body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubReleaseAsset {
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

impl GithubRelease {
    pub fn build(&self, config: &Config) -> Result<Box<section<String>>> {
        let tag_name = self.tag_name.clone();
        let title = self.name.clone().unwrap_or(tag_name.clone());

        let id: axohtml::types::Id = axohtml::types::Id::new(tag_name.clone());
        let formatted_date = match DateTime::parse_from_rfc3339(&self.published_at) {
            Ok(date) => date.format("%b %e %Y at %R UTC").to_string(),
            Err(_) => self.published_at.to_owned(),
        };

        let classnames = if self.prerelease {
            "release pre-release hidden"
        } else {
            "release"
        };
        let link = format!("#{}", &tag_name);
        let body = self.build_release_body(config)?;

        Ok(html!(
        <section class=classnames>
            <h2 id=id><a href=link>{text!(title)}</a></h2>
            <div class="release-info">
                <span class="flex items-center gap-2">
                    {tag_icon()}{text!(tag_name)}
                </span>
                <span class="flex items-center gap-2">
                    {date_icon()}{text!(&formatted_date)}
                </span>
            </div>
            <div class="release-body mb-6">
                {unsafe_text!(body)}
            </div>
        </section>
        ))
    }

    fn build_release_body(&self, config: &Config) -> Result<String> {
        let contents = if self.has_dist_manifest() {
            cargo_dist::fetch_manifest(config)?
                .manifest
                .announcement_changelog
                .unwrap_or(String::new())
        } else {
            self.body.clone().unwrap_or(String::new())
        };

        markdown::to_html(&contents, &config.syntax_theme)
    }

    pub fn has_dist_manifest(&self) -> bool {
        for asset in &self.assets {
            if asset.name == "dist-manifest.json" {
                return true;
            }
        }
        false
    }
}

fn tag_icon() -> Box<UnsafeTextNode<String>> {
    unsafe_text!("<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-6 h-6'>
    <path stroke-linecap='round' stroke-linejoin='round' d='M9.568 3H5.25A2.25 2.25 0 003 5.25v4.318c0 .597.237 1.17.659 1.591l9.581 9.581c.699.699 1.78.872 2.607.33a18.095 18.095 0 005.223-5.223c.542-.827.369-1.908-.33-2.607L11.16 3.66A2.25 2.25 0 009.568 3z' />
    <path stroke-linecap='round' stroke-linejoin='round' d='M6 6h.008v.008H6V6z' /></svg>")
}

fn date_icon() -> Box<UnsafeTextNode<String>> {
    unsafe_text!("<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-6 h-6'>
    <path stroke-linecap='round' stroke-linejoin='round' d='M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5' /></svg>")
}
