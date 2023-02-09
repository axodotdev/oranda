use crate::config::artifacts::Artifacts;
use crate::config::Config;
use crate::errors::*;
use crate::site::artifacts::cargo_dist;
use crate::site::artifacts::package_managers;
use axohtml::html;

pub fn build(config: &Config) -> Result<String> {
    let mut html = vec![];
    let manifest = cargo_dist::fetch_manifest(config)?;

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
