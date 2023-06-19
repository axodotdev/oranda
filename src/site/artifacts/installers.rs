#![allow(clippy::vec_box)]

use axohtml::elements::{a, div, li, select};
use axohtml::{html, text, unsafe_text};
use chrono::DateTime;
use std::collections::HashMap;

use crate::config::Config;
use crate::data::artifacts::inference::triple_to_display_name;
use crate::data::artifacts::{FileIdx, InstallMethod, InstallerIdx, TargetTriple};
use crate::data::Release;
use crate::errors::*;
use crate::site::{icons, link, markdown};

type Platforms = HashMap<TargetTriple, Vec<InstallerIdx>>;

pub fn build_header(release: &Release, config: &Config) -> Result<Box<div<String>>> {
    let downloads_href = link::generate(&config.path_prefix, "artifacts/");
    let tag = &release.source.tag_name;
    let platforms_we_want = filter_platforms(release);
    if platforms_we_want.is_empty() {
        return Ok(html!(<div></div>));
    }
    let one_platform = platforms_we_want.len() == 1;

    let formatted_date = match DateTime::parse_from_rfc3339(&release.source.published_at) {
        Ok(date) => date.format("%b %e %Y at %R UTC").to_string(),
        Err(_) => release.source.published_at.to_owned(),
    };

    let arches = build_arches(&platforms_we_want, release, config);
    let selector = selector_html(&platforms_we_want);

    let html = html!(
        <div class="artifact-header target">
            <h4>{text!("Install {}", release.source.tag_name)}</h4>
            <div><small class="published-date">{text!("Published on {}", formatted_date)}</small></div>

            <ul class="arches">
                {arches}
            </ul>
        </div>
    );

    // If there's only one platform we don't need scripts
    let noscript = if one_platform {
        None
    } else {
        Some(
            html!(<noscript><a href=&downloads_href class="backup-download primary">{text!("View all installation options")}</a></noscript>),
        )
    };
    // If there's only one platform we don't need dropdowns
    let selector = if one_platform {
        let target = platforms_we_want.keys().next().unwrap();
        if target == "all" {
            // If we think this is a universal setup, don't mention platforms
            None
        } else {
            // Otherwise mention the platform
            let os_name = triple_to_display_name(target).unwrap();
            let desc = format!("Platform: {os_name}");
            Some(html!(<div class="arch-select">{text!(desc)}</div>))
        }
    } else {
        Some(html!(<div class="arch-select hidden">
            {text!("Platform: ")} {selector}
        </div>))
    };
    let no_autodetect = if one_platform {
        None
    } else {
        Some(html!(
        <div class="no-autodetect hidden">
            <span class="no-autodetect-details">{text!("We weren't able to detect your OS. ")}</span>
            <a href=&downloads_href class="backup-download primary">{text!("View all installation options.")}</a>
        </div>
        ))
    };
    Ok(html!(
    <div class="artifacts" data-tag=tag>
        {html}
        {no_autodetect}
        {selector}
        {noscript}
    </div>
    ))
}

/// Build the tab and content HTML for all arches.
fn build_arches(platforms: &Platforms, release: &Release, config: &Config) -> Vec<Box<li<String>>> {
    let mut html = vec![];
    let one_platform = platforms.len() == 1;

    for (target, installers) in platforms {
        // If there's only one installer, no need for tabs
        let tabs = if installers.len() == 1 {
            None
        } else {
            let tabs = tab_list(target, release, installers, one_platform);
            Some(html!(<ul class="tabs">
                {tabs}
            </ul>))
        };

        let contents = content_list(target, installers, release, config, one_platform);

        // If there's only one entry, make it visible by default (noscript friendly)
        let classes = if one_platform { "arch" } else { "arch hidden" };
        html.push(html!(
            <li class=classes data-arch=target>
                {tabs}
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
    installers: &[InstallerIdx],
    one_platform: bool,
) -> Vec<Box<li<String>>> {
    let mut list = vec![];
    let mut is_first = true;
    for i in installers.iter() {
        let installer = release.artifacts.installer(i.to_owned());
        let string_idx = i.0.to_string();
        let classes = if one_platform && is_first {
            "install-tab selected"
        } else {
            "install-tab"
        };
        list.push(
            html!(<li class=classes data-id=string_idx data-triple=target>{text!(installer.label.clone())}</li>),
        );
        is_first = false;
    }
    list
}

/// Build content for one arch/triple.
fn content_list(
    target: &TargetTriple,
    installers: &Vec<InstallerIdx>,
    release: &Release,
    config: &Config,
    one_platform: bool,
) -> Vec<Box<li<String>>> {
    let mut list = vec![];
    let mut is_first = true;
    for idx in installers {
        let installer = release.artifacts.installer(idx.to_owned());

        let html = match &installer.method {
            InstallMethod::Run { file, run_hint } => run_html(*file, run_hint, release, config),
            InstallMethod::Download { file } => {
                let file = release.artifacts.file(*file);
                html!(<div class="download-wrapper"><a href=&file.download_url><button class="button primary"><span>{text!("Download")}</span><span class="button-subtitle">{text!(&file.name)}</span></button></a></div>)
            }
        };

        // If there's only one platform, auto-show
        let classes = if one_platform && is_first {
            "install-content"
        } else {
            "install-content hidden"
        };
        let string_idx = idx.0.to_string();
        let html = html!(
            <li data-id=string_idx data-triple=target class=classes>
                {html}
            </li>
        );

        list.push(html);
        is_first = false;
    }

    list
}

/// Get the html for an InstallMethod::Run
pub fn run_html(
    file: Option<FileIdx>,
    run_hint: &str,
    release: &Release,
    config: &Config,
) -> Box<div<String>> {
    let code = {
        let highlighted_code =
            markdown::syntax_highlight(Some("sh"), run_hint, &config.styles.syntax_theme());
        match highlighted_code {
            Ok(code) => code,
            Err(_) => format!("<code class='inline-code'>{}</code>", run_hint),
        }
    };
    let source_file = if let Some(file) = file {
        let file = release.artifacts.file(file);
        let url = if let Some(view_path) = &file.view_path {
            link::generate(&config.path_prefix, view_path)
        } else {
            file.download_url.clone()
        };
        let html: Box<a<String>> = html!(<a class="button primary" href=&url>{text!("Source")}</a>);
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

/// Build the arch selector.
fn selector_html(platforms: &Platforms) -> Box<select<String>> {
    let mut options = vec![];
    options.push(html!(<option disabled=true selected=true value="">{text!("")}</option>));
    for target in platforms.keys() {
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
    // First try to select platforms with downloadable artifacts
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

    // If that produce non-empty results, great!
    if !platforms.is_empty() {
        return platforms;
    }
    eprintln!("taking universal path");

    // Otherwise, only show things that are on every platform
    let mut universal_installers = vec![];
    if let Some((_, installers)) = release.artifacts.installers_by_target().iter().next() {
        for installer in installers {
            if release
                .artifacts
                .installers_by_target()
                .iter()
                .all(|(_, installers)| installers.contains(installer))
            {
                universal_installers.push(*installer);
            }
        }
    }
    if !universal_installers.is_empty() {
        let mut platforms = Platforms::default();
        platforms.insert("all".to_owned(), universal_installers);
        return platforms;
    }

    // Otherwise it's empty, oh well
    Platforms::default()
}
