use crate::config::Config;
use crate::errors::*;
use crate::site::layout;
use axohtml::{html, text, unsafe_text};
use cargo_dist_schema::ArtifactKind;

use super::fetch_manifest;
use crate::config::artifacts::Artifacts;
use crate::site::artifacts::cargo_dist;
use crate::site::artifacts::package_managers;

pub fn build_page(config: &Config) -> Result<()> {
    let mut html = vec![];
    let manifest = fetch_manifest(&config)?;

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
            for release in manifest.releases.iter() {
                for artifact in release.artifacts.iter() {
                    if let ArtifactKind::ExecutableZip = artifact.kind {
                        let mut targets = String::new();
                        for targ in artifact.target_triples.iter() {
                            targets.push_str(format!("{} ", targ).as_str());
                        }
                        let install_code = cargo_dist::get_install_hint(
                            &release.artifacts,
                            &artifact.target_triples,
                            &config.syntax_theme,
                        );
                        list.extend(html!(<li class="list-none"><h5>{text!(targets)}</h5> {unsafe_text!(install_code)}</li>))
                    }
                }
            }
        }
        for (manager, install_code) in managers.iter() {
            list.extend(html!(<li class="list-none"><h5>{text!(manager)}</h5> {unsafe_text!(package_managers::create_package_install_code(install_code, &config.syntax_theme))}</li>))
        }

        html.extend(
            html!(<div class="package-managers-downloads"><h3>{text!("Install methods")}</h3><ul>{list}</ul></div>),
        );
    };

    if let Some(Artifacts {
        cargo_dist: Some(true),
        ..
    }) = &config.artifacts
    {
        html.extend(cargo_dist::build_table(manifest, &config));
    };

    let doc = layout::build(config, html!(<div>{html}</div>), false)?;
    let html_path = format!("{}/artifacts.html", &config.dist_dir);
    let asset = axoasset::local::LocalAsset::new(&html_path, doc.into());
    axoasset::local::LocalAsset::write(&asset, &config.dist_dir)?;
    Ok(())
}
