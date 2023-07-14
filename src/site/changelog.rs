use serde::Serialize;

use crate::config::Config;
use crate::data::{Context, Release};
use crate::errors::*;
use crate::site::{javascript, markdown};

#[derive(Serialize, Debug)]
pub struct ChangelogContext {
    pub releases: Vec<ChangelogRelease>,
    pub has_prereleases: bool,
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

pub fn index_context(context: &Context, config: &Config) -> Result<ChangelogContext> {
    let releases = context
        .releases
        .iter()
        .map(|release| single_context(release, config))
        .collect();
    Ok(ChangelogContext {
        releases,
        has_prereleases: context.has_prereleases,
        os_script: javascript::build_os_script_path(&config.build.path_prefix),
    })
}

pub fn single_context(release: &Release, config: &Config) -> ChangelogRelease {
    ChangelogRelease {
        is_prerelease: release.source.is_prerelease(),
        version_tag: release.source.version_tag().to_string(),
        name: release.source.name().map(|s| s.to_string()),
        formatted_date: release.source.formatted_date(),
        body: build_release_body(release, config).unwrap_or("".to_string()),
    }
}

fn build_release_body(release: &Release, config: &Config) -> Result<String> {
    let contents = if let Some(manifest) = &release.manifest {
        manifest.announcement_changelog.clone().unwrap_or_default()
    } else {
        release.source.body().unwrap_or_default().to_owned()
    };

    markdown::to_html(&contents, &config.styles.syntax_theme)
}
