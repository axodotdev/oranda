use crate::config::artifacts::Artifacts;
use crate::config::Config;
use crate::data::cargo_dist;
use crate::errors::*;

pub mod header;
mod installers;
mod package_managers;
mod table;

use axohtml::html;

pub fn build(config: &Config) -> Result<String> {
    let mut html = vec![];
    let release = cargo_dist::fetch_release(config)?;

    if config.artifacts.is_some() {
        let mut lists = vec![];
        if let Some(Artifacts {
            cargo_dist: Some(true),
            ..
        }) = &config.artifacts
        {
            lists.extend(installers::build_list(&release, config));
        }

        if let Some(Artifacts {
            package_managers: Some(managers),
            ..
        }) = &config.artifacts
        {
            lists.extend(package_managers::build_list(managers, config));
        }

        html.extend(html!(
            <div class="package-managers-downloads">
                {lists}
            </div>
        ));
    };

    if let Some(Artifacts {
        cargo_dist: Some(true),
        ..
    }) = &config.artifacts
    {
        html.extend(table::build(release, config));
    }
    Ok(html!(<div>{html}</div>).to_string())
}
