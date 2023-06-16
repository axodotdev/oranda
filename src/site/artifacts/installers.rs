use axohtml::elements::{a, div, li, select};
use axohtml::{html, text, unsafe_text};
use chrono::DateTime;
use std::collections::HashMap;

use crate::config::Config;
use crate::data::artifact_inference::triple_to_display_name;
use crate::data::artifacts::{InstallMethod, InstallerIdx, TargetTriple};
use crate::data::Release;
use crate::errors::*;
use crate::site::{icons, link, markdown};

type Platforms = HashMap<TargetTriple, Vec<InstallerIdx>>;

pub fn build_header(release: &Release, config: &Config) -> Result<Box<div<String>>> {
    let downloads_href = link::generate(&config.path_prefix, "artifacts/");
    let tag = &release.source.tag_name;
    let platforms_we_want = filter_platforms(release);
    let formatted_date = match DateTime::parse_from_rfc3339(&release.source.published_at) {
        Ok(date) => date.format("%b %e %Y at %R UTC").to_string(),
        Err(_) => release.source.published_at.to_owned(),
    };
    let arches = build_arches(&platforms_we_want, release, config);
    let selector = selector_html(&platforms_we_want);

    let html = html!(
        <div class="artifact-header target">
            <h4>{text!("Install {}", release.source.tag_name)}</h4>
            <div><small class="published-date">{text!("Published at {}", formatted_date)}</small></div>

            <ul class="arches">
                {arches}
            </ul>
        </div>
    );

    Ok(html!(
    <div class="artifacts" data-tag=tag>
        {html}
        <div class="no-autodetect hidden">{text!("We weren't able to detect your OS.")}</div>
        <div class="arch-select hidden">
            {text!("Select your platform manually:")} {selector}
        </div>
        <noscript><a href=&downloads_href class="backup-download primary">{text!("View installation options")}</a></noscript>
    </div>
    ))
}

/// Build the tab and content HTML for all arches.
fn build_arches(platforms: &Platforms, release: &Release, config: &Config) -> Vec<Box<li<String>>> {
    let mut html = vec![];

    for (target, installers) in platforms {
        let tabs = tab_list(target, release, installers);
        let contents = content_list(target, &installers, release, config);

        html.push(html!(
            <li class="arch hidden" data-arch=target>
                <ul class="tabs">
                    {tabs}
                </ul>
                <ul class="contents">
                    {contents}
                </ul>
            </li>
        ));
    }

    html
}

/// Build tabs for one arch/triple.
fn tab_list(
    target: &TargetTriple,
    release: &Release,
    installers: &Vec<InstallerIdx>,
) -> Vec<Box<li<String>>> {
    let mut list = vec![];
    for i in installers.iter() {
        let installer = release.artifacts.installer(i.to_owned());
        let string_idx = i.0.to_string();
        list.push(
            html!(<li class="install-tab" data-id=string_idx data-triple=target>{text!(installer.label.clone())}</li>),
        )
    }
    list
}

/// Build content for one arch/triple.
fn content_list(
    target: &TargetTriple,
    installers: &Vec<InstallerIdx>,
    release: &Release,
    config: &Config,
) -> Vec<Box<li<String>>> {
    let mut list = vec![];
    for idx in installers {
        let installer = release.artifacts.installer(idx.to_owned());

        let html = match &installer.method {
            InstallMethod::Run { file, run_hint } => {
                let code = {
                    let highlighted_code = markdown::syntax_highlight(
                        Some("sh"),
                        &run_hint,
                        &config.styles.syntax_theme(),
                    );
                    match highlighted_code {
                        Ok(code) => code,
                        Err(_) => format!("<code class='inline-code'>{}</code>", run_hint),
                    }
                };
                let source_file = if file.is_some() {
                    let file = release.artifacts.file(file.unwrap());
                    let html: Box<a<String>> = html!(<a class="button primary" href=&file.download_url>{text!("Source")}</a>);
                    html.to_string()
                } else {
                    String::new()
                };
                let icon = icons::copy();

                html!(
                    <div class="install-code-wrapper">
                        {unsafe_text!(code)}
                        <button class="button copy-clipboard-button primary" data-copy=run_hint>{icon}</button>
                        {unsafe_text!(source_file)}
                    </div>
                )
            }
            InstallMethod::Download { file } => {
                let file = release.artifacts.file(file.to_owned());
                html!(<div><a href=&file.download_url><button class="button primary">{text!("Download")}</button></a></div>)
            }
        };

        let string_idx = idx.0.to_string();
        let html = html!(
            <li data-id=string_idx data-triple=target class="install-content hidden">
                {html}
            </li>
        );

        list.push(html);
    }

    list
}

/// Build the arch selector.
fn selector_html(platforms: &Platforms) -> Box<select<String>> {
    let mut options = vec![];
    options.push(html!(<option disabled=true selected=true value="">{text!("")}</option>));
    for (target, _) in platforms {
        let os_name = triple_to_display_name(target).unwrap();
        options.push(html!(<option value=target>{text!(os_name.to_owned())}</option>));
    }

    html!(
        <select id="install-arch-select">
            {options}
        </select>
    )
}

/// Only grab platforms that we can actually provide downloadable files for.
fn filter_platforms(release: &Release) -> Platforms {
    let mut platforms = HashMap::new();
    for (target, installer) in release.artifacts.installers_by_target().iter() {
        let has_valid_installer = installer.iter().any(|i| {
            let installer = release.artifacts.installer(i.to_owned());
            matches!(installer.method, InstallMethod::Download { file: _ })
        });
        if has_valid_installer {
            platforms.insert(target.clone(), installer.to_vec());
        }
    }

    platforms
}
