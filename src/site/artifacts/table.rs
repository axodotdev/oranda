use axohtml::elements::{div, span};
use axohtml::{html, text};

use crate::config::Config;
use crate::data::cargo_dist::{self, DistRelease};
use crate::errors::*;

pub fn build(release: DistRelease, config: &Config) -> Result<Box<div<String>>> {
    let mut table = vec![];
    let manifest = release.manifest;
    for app in manifest.releases.iter() {
        for artifact_id in app.artifacts.iter() {
            let artifact = &manifest.artifacts[artifact_id];
            if let Some(name) = artifact.name.clone() {
                let url = cargo_dist::download_link(config, &name, &app.app_version)?;
                let kind = cargo_dist::get_kind_string(&artifact.kind);
                let targets: &String = &artifact.target_triples.clone().into_iter().collect();
                table.extend(vec![
                    html!(<span>{text!(name)}</span>),
                    html!(<span>{text!(kind)}</span>),
                    html!(<span>{text!(targets)}</span>),
                    html!(<span><a href=url>{text!("Download")}</a></span>),
                ]);
            }
        }
    }

    Ok(html(table))
}

// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
fn html(table: Vec<Box<span<String>>>) -> Box<div<String>> {
    html!(
    <div>
        <h3>{text!("Downloads")}</h3>
        <div class="table">
            <span class="th">
                {text!("Name")}
            </span>
            <span class="th">
                {text!("Kind")}
            </span>
            <span class="th">
            {text!("Target")}
            </span>
            <span class="th">
                {text!("Download")}
            </span>
            {table}
        </div>
    </div>
    )
}
