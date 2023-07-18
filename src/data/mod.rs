use crate::config::{ArtifactsConfig, ProjectConfig};
use crate::data::github::{GithubRelease, GithubRepo};
use crate::data::release::CurrentStateRelease;
use crate::errors::*;

pub mod artifacts;
pub mod cargo_dist;
pub mod funding;
pub mod github;
mod release;
pub mod workspaces;

pub use release::Release;

use self::release::ReleaseSource;

#[derive(Debug)]
pub struct Context {
    /// Info from Github
    pub repo: Option<GithubRepo>,
    /// All of the releases, currently from newest to oldest
    pub releases: Vec<Release>,
    /// Whether any of the 'releases` are prereleases
    ///
    /// (enables extra UI)
    pub has_prereleases: bool,
    /// Index into `releases` for the "best" latest release
    ///
    /// (e.g. prefers stable releases over prereleases)
    pub latest_release: Option<usize>,
    /// Whether any of the `releases` have anything useful for
    /// the artifacts subsystem.
    pub has_artifacts: bool,
}

impl Context {
    /// Make a Context with a faux-release for the current project state
    pub fn new_current(
        project_config: &ProjectConfig,
        artifacts_config: Option<&ArtifactsConfig>,
    ) -> Result<Self> {
        let releases = tokio::runtime::Handle::current().block_on(Self::make_current_release(
            None,
            project_config,
            artifacts_config,
        ))?;
        Ok(Self::with_releases(None, releases, artifacts_config))
    }
    /// Get releases using github
    pub fn new_github(
        repo_url: &str,
        project_config: &ProjectConfig,
        artifacts_config: Option<&ArtifactsConfig>,
    ) -> Result<Self> {
        let repo = GithubRepo::from_url(repo_url)?;
        let mut releases = Self::fetch_all_releases(&repo, artifacts_config)?;
        if releases.is_empty() {
            releases = tokio::runtime::Handle::current().block_on(Self::make_current_release(
                Some(&repo),
                project_config,
                artifacts_config,
            ))?;
        }
        Ok(Self::with_releases(Some(repo), releases, artifacts_config))
    }

    /// Get the latest release, if it exists
    pub fn latest(&self) -> Option<&Release> {
        self.latest_release.and_then(|idx| self.releases.get(idx))
    }

    /// Mutably get the latest release, if it exists
    pub fn latest_mut(&mut self) -> Option<&mut Release> {
        self.latest_release
            .and_then(|idx| self.releases.get_mut(idx))
    }

    /// Fetch and process all the Github Releases to produce a final result
    pub fn fetch_all_releases(
        repo: &GithubRepo,
        artifacts_config: Option<&ArtifactsConfig>,
    ) -> Result<Vec<Release>> {
        let gh_releases =
            tokio::runtime::Handle::current().block_on(GithubRelease::fetch_all(repo))?;
        let all = tokio::runtime::Handle::current().block_on(
            futures_util::future::try_join_all(gh_releases.into_iter().map(|gh_release| {
                Release::new(
                    ReleaseSource::Github(gh_release),
                    Some(repo),
                    artifacts_config,
                )
            })),
        )?;
        Ok(all)
    }

    fn with_releases(
        repo: Option<GithubRepo>,
        releases: Vec<Release>,
        artifacts_config: Option<&ArtifactsConfig>,
    ) -> Self {
        // Walk through all the releases (from newest to oldest) to find the latest ones
        //
        // FIXME?: I think this is essentially deferring to Release Date over Version Number.
        // In most cases those things are in agreement, but anyone who does patch releases
        // of older versions will have a more chaotic history. When we have more robust
        // handling of release tags we should try to sort/filter this better.
        let mut has_prereleases = false;
        let mut has_artifacts = false;
        let mut warned = false;
        let mut latest_dist_stable_release = None;
        let mut latest_dist_prerelease = None;
        let mut latest_stable_release = None;
        let mut latest_prerelease = None;

        for (idx, release) in releases.iter().enumerate() {
            // Make note of whether anything has artifacts
            if release.has_installers() {
                has_artifacts = true;
            }

            let is_prerelease = release.source.is_prerelease();
            // Make note of whether we've found prereleases or stable releases yet
            if is_prerelease {
                if !has_prereleases {
                    has_prereleases = true;
                }
                if latest_prerelease.is_none() {
                    latest_prerelease = Some(idx);
                }
            } else if latest_stable_release.is_none() {
                latest_stable_release = Some(idx);
            }

            // Special handling of dist-manifest.json
            if release.manifest.is_some() {
                if artifacts_config.map(|a| a.cargo_dist).unwrap_or(false) {
                    // cargo-dist is enabled, so we want to find the latest stable release
                    // or, failing that, the latest prerelease.
                    if is_prerelease {
                        if latest_dist_prerelease.is_none() {
                            latest_dist_prerelease = Some(idx);
                        }
                    } else if latest_dist_stable_release.is_none() {
                        latest_dist_stable_release = Some(idx);
                    }
                } else if !warned {
                    // We found a dist-manifest but they didn't enable cargo-dist support, encourage them to do so
                    let msg = "You have not configured cargo-dist yet we detected dist-manifests in your releases. Is this intended?";
                    tracing::warn!("{}", msg);
                    warned = true;
                }
            }
        }

        // If we found a stable cargo-dist release, but there's even newer stable releases
        // that don't use cargo-dist, we're going to prefer the cargo-dist one, but we should
        // warn the user that things are wonky
        if let (Some(dist_latest), Some(latest)) =
            (latest_dist_stable_release, latest_stable_release)
        {
            if latest_dist_stable_release != latest_stable_release {
                let dist_rel = &releases[dist_latest].source.version_tag();
                let stable_rel = &releases[latest].source.version_tag();
                let msg = format!("You have newer stable Github Releases ({}) than your latest cargo-dist Release ({}). Is this intended? (We're going to prefer the cargo-dist one.)", stable_rel, dist_rel);
                tracing::warn!("{}", msg);
            }
        }

        // To select the latest release, we use the following priority: (first valid entry wins)
        //
        // * dist stable
        // * normal stable
        // * dist prerelease
        // * normal prerelease
        //
        // The reason we single out dist releases as special/better is a bit of
        // a legacy hack to keep things working while we don't have a good mechanism for filtering
        // out "other" github release like oranda-css. In the future we will only care about
        // stable vs unstable.
        let latest_release = latest_dist_stable_release
            .or(latest_stable_release)
            .or(latest_dist_prerelease)
            .or(latest_prerelease);

        Self {
            repo,
            releases,
            has_prereleases,
            has_artifacts,
            latest_release,
        }
    }

    async fn make_current_release(
        repo: Option<&GithubRepo>,
        project_config: &ProjectConfig,
        artifacts_config: Option<&ArtifactsConfig>,
    ) -> Result<Vec<Release>> {
        let release = Release::new(
            ReleaseSource::CurrentState(CurrentStateRelease {
                version: project_config.version.to_owned(),
                date: None,
                prerelease: false,
            }),
            repo,
            artifacts_config,
        )
        .await?;
        Ok(vec![release])
    }
}
