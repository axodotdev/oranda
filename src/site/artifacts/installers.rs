use axohtml::elements::div;
use axohtml::{html, text};

use crate::config::Config;
use crate::data::Release;
use crate::errors::*;
use crate::site::link;

pub fn build_header(release: &Release, config: &Config) -> Result<Box<div<String>>> {
    let downloads_href = link::generate(&config.path_prefix, "artifacts/");
    let tag = &release.source.tag_name;

    // TODO: generate html?
    let html = None;

    Ok(html!(
    <div class="artifacts" data-tag=tag>
        {html}
        <a href=&downloads_href class="hidden backup-download business-button primary">{text!("View installation options")}</a>
    </div>
    ))
}
