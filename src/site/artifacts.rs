use crate::config::Config;
use crate::errors::*;
use crate::site::layout;
use axohtml::elements::{div, span};
use axohtml::{html, text, unsafe_text};
use cargo_dist_schema::{Artifact, ArtifactKind, DistManifest};

use crate::config::artifacts::Artifacts;
use crate::site::markdown::syntax_highlight::syntax_highlight;
use crate::site::markdown::syntax_highlight::syntax_themes::SyntaxTheme;
use linked_hash_map::LinkedHashMap;

fn get_kind_string(kind: &ArtifactKind) -> String {
    match kind {
        ArtifactKind::ExecutableZip => String::from("Executable Zip"),
        ArtifactKind::Symbols => String::from("Symbols"),
        ArtifactKind::Installer => String::from("Installer"),
        _ => String::from("Unknown"),
    }
}

fn create_download_link(config: &Config, name: &String) -> String {
    if let (Some(repo), Some(version)) = (&config.repository, &config.version) {
        format!("{}/releases/download/v{}/{}", repo, version, name)
    } else {
        String::new()
    }
}

fn fetch_manifest(config: &Config) -> std::result::Result<DistManifest, reqwest::Error> {
    let url = create_download_link(config, &String::from("dist-manifest.json"));

    let resp = reqwest::blocking::get(url)?;

    resp.json::<DistManifest>()
}

fn build_cargo_dist(config: &Config) -> Result<Box<div<String>>> {
    if config.repository.is_none() || config.version.is_none() {
        return Err(OrandaError::Other(String::from(
            "The repository and version are required for cargo_dist",
        )));
    }

    let typed = fetch_manifest(&config)?;

    let mut html: Vec<Box<div<String>>> = vec![];
    for release in typed.releases.iter() {
        for artifact in release.artifacts.iter() {
            if let ArtifactKind::ExecutableZip = artifact.kind {
                let mut targets = String::new();
                for targ in artifact.target_triples.iter() {
                    targets.push_str(format!("{} ", targ).as_str());
                }
                let url = create_download_link(config, &artifact.name);
                let text = format!("Download v{}", &release.app_version);
                let install_code = get_install_hint(
                    &release.artifacts,
                    &artifact.target_triples,
                    &config.syntax_theme,
                );

                html.extend(html!(
                        <div class="hidden target artifact-header" data-targets=targets>
                            <h4 class="text-center">{text!("Quick install")}</h4>
                            {unsafe_text!(install_code)}
                            <div>
                                <a href=url class="text-center">
                                    {text!(text)}
                                </a>
                                <a href="/artifacts.html" class="download-all">{text!("View all downloads")}</a>
                            </div>
                        </div>
                    ));
            }
        }
    }

    Ok(html!(
    <div class="artifacts">
        {html}
        <a href="/artifacts.html" class="hidden backup-download business-button primary">{text!("View installation options")}</a>
    </div>
    ))
}

fn create_package_install_code(code: &str, syntax_theme: &SyntaxTheme) -> String {
    let highlighted_code = syntax_highlight(Some("sh"), code, &syntax_theme);
    match highlighted_code {
        Ok(code) => code,
        Err(_) => format!("<code class='text-center break-all'>{}</code>", code),
    }
}

fn build_package_managers(
    config: &Config,
    package_managers: &LinkedHashMap<String, String>,
) -> Result<Box<div<String>>> {
    let (manager, hint) = if let Some((manager, hint)) = package_managers.front() {
        (manager, hint)
    } else {
        return Err(OrandaError::Other(String::from(
            "You are using package managers but none is present, please add one.",
        )));
    };
    let install_code = create_package_install_code(hint.as_str(), &config.syntax_theme);

    Ok(html!(<div>
    <h4 class="text-center">{text!(format!("Install with {}", manager))}</h4>
    {unsafe_text!(install_code)}
    <div>
        <a href="/artifacts.html" class="download-all">{text!("View all downloads")}</a>
    </div>
</div>))
}

pub fn create_header(config: &Config) -> Result<Option<Box<div<String>>>> {
    if let Some(artifact) = &config.artifacts {
        build_artifacts_html(config)?;
        if artifact.cargo_dist.is_some() {
            Ok(Some(build_cargo_dist(&config)?))
        } else if let Some(package_managers) = &artifact.package_managers {
            Ok(Some(build_package_managers(&config, package_managers)?))
        } else {
            Ok(None)
        }
    } else {
        return Ok(None);
    }
}

pub fn get_install_hint(
    artifacts: &[Artifact],
    target_triples: &[String],
    syntax_theme: &SyntaxTheme,
) -> String {
    let hint = artifacts.iter().find(|artifact| {
        artifact.install_hint.is_some()
            && artifact
                .target_triples
                .iter()
                .any(|h| target_triples.iter().any(|item| item == h))
    });

    if let Some(current_hint) = hint {
        if let Some(install_hint) = &current_hint.install_hint {
            let highlighted_code = syntax_highlight(Some("sh"), install_hint, syntax_theme);
            return match highlighted_code {
                Ok(code) => code,
                Err(_) => format!(
                    "<code class='text-center break-all'>{}</code>",
                    install_hint
                ),
            };
        }
    }

    String::new()
}

// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
fn create_content(table: Vec<Box<span<String>>>) -> Box<div<String>> {
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

pub fn build_artifacts_html(config: &Config) -> Result<()> {
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
                        let install_code = get_install_hint(
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
            list.extend(html!(<li class="list-none"><h5>{text!(manager)}</h5> {unsafe_text!(create_package_install_code(install_code, &config.syntax_theme))}</li>))
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
        let mut table = vec![];
        for release in manifest.releases.iter() {
            for artifact in release.artifacts.iter() {
                let name = &artifact.name;
                let url = create_download_link(config, name);
                let kind = get_kind_string(&artifact.kind);
                let targets: &String = &artifact.target_triples.clone().into_iter().collect();
                table.extend(vec![
                    html!(<span>{text!(name)}</span>),
                    html!(<span>{text!(kind)}</span>),
                    html!(<span>{text!(targets)}</span>),
                    html!(<span><a href=url>{text!("Download")}</a></span>),
                ]);
            }
        }

        html.extend(create_content(table));
    };

    let doc = layout::build(config, html!(<div>{html}</div>), false)?;
    let html_path = format!("{}/artifacts.html", &config.dist_dir);
    let asset = axoasset::local::LocalAsset::new(&html_path, doc.into());
    axoasset::local::LocalAsset::write(&asset, &config.dist_dir)?;
    Ok(())
}
