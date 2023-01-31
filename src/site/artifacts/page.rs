use crate::config::artifacts::Artifacts;
use crate::config::Config;
use crate::errors::*;
use crate::site::artifacts::cargo_dist;
use crate::site::artifacts::package_managers;
use crate::site::layout;
use axohtml::{html, text};

pub fn build(config: &Config) -> Result<String> {
    let mut html = vec![];
    let manifest = cargo_dist::fetch_manifest(config)?;

    if let Some(Artifacts {
        package_managers: Some(managers),
        ..
    }) = &config.artifacts
    {
        let mut list = vec![];
        if let Some(Artifacts {
            cargo_dist: Some(true),
            ..
        }) = &config.artifacts
        {
            list.extend(cargo_dist::build_list(&manifest, &config.syntax_theme));
        }

        list.extend(package_managers::build_list(managers, config));

        html.extend(html!(
            <div class="package-managers-downloads">
                <h3>{text!("Install methods")}</h3>
                <ul>
                    {list}
                </ul>
            </div>
        ));
    };

    if let Some(Artifacts {
        cargo_dist: Some(true),
        ..
    }) = &config.artifacts
    {
        html.extend(cargo_dist::build_table(manifest, config));
    };

    layout::build(config, html!(<div>{html}</div>), false)
}
