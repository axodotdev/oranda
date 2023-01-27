use crate::config::artifacts::Artifacts;
use crate::config::Config;
use crate::errors::*;
use crate::site::html::build_common_html;
use axohtml::elements::{div, script, span};
use axohtml::{html, text, unsafe_text};
use cargo_dist_schema::{Artifact, ArtifactKind, DistManifest};

use crate::site::markdown::syntax_highlight::syntax_highlight;
use crate::site::markdown::syntax_highlight::syntax_themes::SyntaxTheme;
use std::fs::File;
use std::io::Write;

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

pub fn create_artifacts_header(config: &Config) -> Result<Option<Box<div<String>>>> {
    let Some(Artifacts { cargo_dist: true }) = &config.artifacts else {
        return Ok(None);
      };

    if config.repository.is_none() || config.version.is_none() {
        return Err(OrandaError::Other(String::from(
            "The repository and version are required for cargo_dist",
        )));
    }

    let url = create_download_link(config, &String::from("dist-manifest.json"));

    let resp = reqwest::blocking::get(url);

    let Ok(resp) = resp else {
        return Err(OrandaError::Other(String::from(
            "The repository and version configurations are required for cargo_dist",
        )));
      };

    let typed = &resp.json::<DistManifest>()?;

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

    build_artifacts_html(config, typed)?;

    Ok(Some(html!(
    <div class="artifacts">
        {html}
        <a href="/artifacts.html" class="hidden backup-download business-button primary">{text!("View installation options")}</a>
    </div>
    )))
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

pub fn get_os_script(config: &Config) -> Result<Box<script<String>>> {
    let detect_os_js = axoasset::copy("src/site/javascript/detect_os.js", &config.dist_dir);

    let path = tokio::runtime::Handle::current().block_on(detect_os_js)?;
    let path_as_string = path.strip_prefix(&config.dist_dir)?.to_string_lossy();
    Ok(html!(<script src=path_as_string />))
}

// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
fn create_content(table: Vec<Box<span<String>>>) -> Box<div<String>> {
    html!(
    <div>
        <h1>{text!("All downloads")}</h1>
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

pub fn build_artifacts_html(config: &Config, manifest: &DistManifest) -> Result<()> {
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
    let doc = build_common_html(config, create_content(table))?;
    let html_path = format!("{}/artifacts.html", &config.dist_dir);

    let mut html_file = File::create(html_path)?;
    html_file.write_all(doc.as_bytes())?;
    Ok(())
}
