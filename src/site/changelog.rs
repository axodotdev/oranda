use axohtml::elements::{div, li, section};
use axohtml::html;
use axohtml::{text, unsafe_text};

use crate::config::Config;
use crate::data::{Context, Release};
use crate::errors::*;
use crate::site::{icons, markdown};

pub fn build(context: &Context, config: &Config) -> Result<String> {
    let mut releases_html: Vec<Box<section<String>>> = vec![];
    let mut releases_nav: Vec<Box<li<String>>> = vec![];

    for release in context.releases.iter() {
        let classnames = if release.source.is_prerelease() {
            "pre-release hidden"
        } else {
            ""
        };

        let link = format!("{}/", &release.source.version_tag());

        releases_html.extend(build_page_preview(release, config, true)?);
        releases_nav.extend(
            html!(<li class=classnames><a href=link>{text!(release.source.version_tag())}</a></li>),
        )
    }

    Ok(html!(
        <div>
            <h1>{text!("Releases")}</h1>
            <div class="releases-wrapper">
                <nav class="releases-nav">
                    {build_prerelease_toggle(context.has_prereleases)}
                    <ul>
                        {releases_nav}
                    </ul>
                </nav>
                <div class="releases-list">{releases_html}</div>
            </div>
        </div>
    )
    .to_string())
}

/// Builds a page for every release. Returns a vec of tuples, the first element being the
/// release name to be used for the filename, and the second element being the content of
/// the page itself.
pub fn build_all(context: &Context, config: &Config) -> Result<Vec<(String, String)>> {
    let mut releases = vec![];
    for release in context.releases.iter() {
        releases.push((
            release.source.version_tag().to_owned(),
            build_single_release(config, release)?,
        ))
    }

    Ok(releases)
}

/// Builds a single, standalone release page.
pub fn build_single_release(config: &Config, release: &Release) -> Result<String> {
    let preview = build_page_preview(release, config, false);
    let title = release
        .source
        .name()
        .unwrap_or(release.source.version_tag());

    Ok(html!(
         <div>
            <h1>{text!("{}", title)}</h1>
            <div class="releases-body">
                {preview}
            </div>
        </div>
    )
    .to_string())
}

pub fn build_page_preview(
    release: &Release,
    config: &Config,
    is_page: bool,
) -> Result<Box<section<String>>> {
    let tag_name = release.source.version_tag();
    let title = release.source.name().unwrap_or(tag_name);

    // We need to prefix the id with `tag-` to not break on things like "0.14.0" (no v prefix)
    let id: axohtml::types::Id = axohtml::types::Id::new(format!("tag-{tag_name}"));
    let date_html = release.source.formatted_date().map(|date| {
        html!(
            <span class="flex items-center gap-2">
                {icons::date()}{text!(&date)}
            </span>
        )
    });

    let classnames = if release.source.is_prerelease() {
        "release pre-release hidden"
    } else {
        "release"
    };
    let link = if is_page {
        format!("{}/", &tag_name)
    } else {
        format!("#{}", &tag_name)
    };
    let body = build_release_body(release, config)?;
    let header_class = if is_page { "" } else { "hidden" };

    Ok(html!(
        <section class=classnames>
            <h2 class=header_class id=id><a href=link>{text!(title)}</a></h2>
            <div class="release-info">
                <span class="flex items-center gap-2">
                    {icons::tag()}{text!(tag_name)}
                </span>
                {date_html}
            </div>
            <div class="release-body mb-6">
                {unsafe_text!(body)}
            </div>
        </section>
    ))
}

fn build_release_body(release: &Release, config: &Config) -> Result<String> {
    let contents = if let Some(manifest) = &release.manifest {
        manifest.announcement_changelog.clone().unwrap_or_default()
    } else {
        release.source.body().unwrap_or_default().to_owned()
    };

    markdown::to_html(
        &contents,
        &config.styles.syntax_theme,
        &config.build.path_prefix,
    )
}

fn build_prerelease_toggle(has_prereleases: bool) -> Option<Box<div<String>>> {
    if has_prereleases {
        Some(html!(
    <div class="prereleases-toggle">
        <div class="flex h-6 items-center">
            <input id="show-prereleases" type="checkbox" />
        </div>
        <div class="ml-3">
            <label for="show-prereleases">{text!("Show prereleases")}</label>
        </div>
    </div>))
    } else {
        None
    }
}
