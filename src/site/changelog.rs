use axoproject::{Version, WorkspaceInfo, WorkspaceKind};
use serde::Serialize;

use crate::config::Config;
use crate::data::{Context, Release};
use crate::errors::*;
use crate::site::{javascript, markdown};

#[derive(Serialize, Debug)]
pub struct ChangelogContext {
    pub releases: Vec<ChangelogRelease>,
    pub has_prereleases: bool,
    pub has_rss_feed: bool,
    pub os_script: String,
}

#[derive(Serialize, Debug)]
pub struct ChangelogRelease {
    pub is_prerelease: bool,
    pub version_tag: String,
    pub name: Option<String>,
    pub formatted_date: Option<String>,
    pub body: String,
}

pub fn index_context(
    context: &Context,
    config: &Config,
    project: Option<&WorkspaceInfo>,
) -> Result<ChangelogContext> {
    // Render an empty page if we're probably dealing with the "dummy" release generated as a
    // fallback.
    if context.releases.len() == 1 && context.releases[0].source.is_current_state() {
        return Ok(ChangelogContext {
            releases: Vec::new(),
            has_prereleases: false,
            has_rss_feed: config.components.changelog.clone().is_some_and(|c| c.generate_rss_feed),
            os_script: javascript::build_os_script_path(&config.build.path_prefix),
        });
    }
    let releases = context
        .releases
        .iter()
        .map(|release| single_context(release, config, project))
        .collect();
    Ok(ChangelogContext {
        releases,
        has_prereleases: context.has_prereleases,
        has_rss_feed: config.components.changelog.clone().is_some_and(|c| c.generate_rss_feed),
        os_script: javascript::build_os_script_path(&config.build.path_prefix),
    })
}

pub fn single_context(
    release: &Release,
    config: &Config,
    project: Option<&WorkspaceInfo>,
) -> ChangelogRelease {
    ChangelogRelease {
        is_prerelease: release.source.is_prerelease(),
        version_tag: release.source.version_tag().to_string(),
        name: release.source.name().map(|s| s.to_string()),
        formatted_date: release.source.formatted_date(),
        body: build_release_body(project, release, config).unwrap_or("".to_string()),
    }
}

// Unwrap that we can't avoid without adding an extra if let block, since if let chains aren't stable
#[allow(clippy::unnecessary_unwrap)]
fn build_release_body(
    project: Option<&WorkspaceInfo>,
    release: &Release,
    config: &Config,
) -> Result<String> {
    let contents = if config
        .components
        .changelog
        .as_ref()
        .is_some_and(|c| c.read_changelog_file)
        && project.is_some()
    {
        let project = project.unwrap();
        let version = release.source.version_tag();
        let changelog = project
            .changelog_for_version(&parse_version(version, project)?)
            .map_err(|e| OrandaError::ChangelogParseFailed {
                name: config.project.name.clone(),
                version: version.to_owned(),
                details: e,
            })?;
        if let Some(changelog) = changelog {
            changelog.body
        } else {
            "".to_owned()
        }
    } else {
        release.source.body().unwrap_or_default().to_owned()
    };

    markdown::to_html(&contents, &config.styles.syntax_theme)
}

/// Parses a version string into an axoproject-compatible version.
fn parse_version(version_str: &str, project: &WorkspaceInfo) -> Result<Version> {
    let version_str = if version_str.starts_with('v') {
        version_str.strip_prefix('v').unwrap()
    } else {
        version_str
    };

    match project.kind {
        WorkspaceKind::Rust => {
            let version = semver::Version::parse(version_str).map_err(|_| {
                OrandaError::PackageVersionParse {
                    version: version_str.to_owned(),
                }
            })?;
            Ok(Version::Cargo(version))
        }
        WorkspaceKind::Javascript => {
            let version = node_semver::Version::parse(version_str).map_err(|_| {
                OrandaError::PackageVersionParse {
                    version: version_str.to_owned(),
                }
            })?;
            Ok(Version::Npm(version))
        }
    }
}
