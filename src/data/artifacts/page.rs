use crate::config::artifacts::Artifacts;
use crate::config::Config;
use crate::data::artifacts::{cargo_dist, package_managers};
use crate::errors::*;

use axohtml::html;

pub fn build(config: &Config) -> Result<String> {
    let mut html = vec![];
    let manifest = cargo_dist::fetch_manifest(config)?.manifest;

    if config.artifacts.is_some() {
        let mut lists = vec![];
        if let Some(Artifacts {
            cargo_dist: Some(true),
            ..
        }) = &config.artifacts
        {
            lists.extend(cargo_dist::build_list(&manifest, config));
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
        html.extend(cargo_dist::build_table(manifest, config));
    }
    Ok(html!(<div>{html}</div>).to_string())
}
