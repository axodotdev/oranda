use crate::config::ArtifactsConfig;
use crate::data::github::{GithubRelease, GithubRepo};
use crate::errors::*;
use crate::message::{Message, MessageType};

pub mod artifact_inference;
pub mod artifacts;
pub mod cargo_dist;
pub mod funding;
pub mod github;
mod release;

pub use release::Release;

pub struct Context {
    pub repo: GithubRepo,
    pub releases: Vec<Release>,
    pub has_prereleases: bool,
    pub latest_release: Option<usize>,
    pub has_artifacts: bool,
}

impl Context {
    pub fn new(repo_url: &str, artifacts_config: &ArtifactsConfig) -> Result<Self> {
        let repo = GithubRepo::from_url(repo_url)?;
        let (releases, has_prereleases, has_artifacts, latest_release) =
            Self::fetch_all_releases(&repo, artifacts_config)?;

        Ok(Self {
            repo,
            releases,
            has_prereleases,
            has_artifacts,
            latest_release,
        })
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

    /// Fetch and process all the Github Releases
    ///
    /// Returned values are:
    ///
    /// * list of releases (newest to oldest)
    /// * whether there are any prereleases (enables more complex UI)
    /// * whether there are any artifacts (enables more complex UI)
    /// * the release that should be considered "latest" (shown on the front page)
    #[allow(clippy::unnecessary_unwrap)]
    pub fn fetch_all_releases(
        repo: &GithubRepo,
        artifacts_config: &ArtifactsConfig,
    ) -> Result<(Vec<Release>, bool, bool, Option<usize>)> {
        let gh_releases =
            tokio::runtime::Handle::current().block_on(GithubRelease::fetch_all(repo))?;
        let all =
            tokio::runtime::Handle::current().block_on(futures_util::future::try_join_all(
                gh_releases
                    .into_iter()
                    .map(|gh_release| Release::new(gh_release, repo, artifacts_config)),
            ))?;

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

        for (idx, release) in all.iter().enumerate() {
            // Make note of whether anything has artifacts
            if release.has_installers() {
                has_artifacts = true;
            }

            let is_prerelease = release.source.prerelease;
            // Make note of whether we've found prereleases or stable releases yet
            if is_prerelease {
                if !has_prereleases {
                    let msg = "Detected pre-releases...";
                    Message::new(MessageType::Info, msg).print();
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
                if artifacts_config.cargo_dist() {
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
                    Message::new(MessageType::Warning, msg).print();
                    warned = true;
                }
            }
        }

        // If we found a stable cargo-dist release, but there's even newer stable releases
        // that don't use cargo-dist, we're going to prefer the cargo-dist one, but we should
        // warn the user that things are wonky
        if latest_dist_stable_release.is_some()
            && latest_dist_stable_release != latest_stable_release
        {
            let dist_rel = &all[latest_dist_stable_release.unwrap()].source.tag_name;
            let stable_rel = &all[latest_stable_release.unwrap()].source.tag_name;
            let msg = format!("You have newer stable Github Releases ({}) than your latest cargo-dist Release ({}). Is this intended? (We're going to prefer the cargo-dist one.)", stable_rel, dist_rel);
            Message::new(MessageType::Warning, &msg).print();
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
        Ok((all, has_prereleases, has_artifacts, latest_release))
    }
}
