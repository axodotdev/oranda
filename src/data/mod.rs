use crate::data::github::{GithubRelease, GithubRepo};
use crate::errors::*;
use crate::message::{Message, MessageType};

pub mod cargo_dist;
use cargo_dist::DistRelease;
pub mod github;
mod release;
pub use release::Release;

pub struct Context {
    pub repo: GithubRepo,
    pub releases: Vec<Release>,
    pub has_prereleases: bool,
    pub latest_dist_release: Option<DistRelease>,
}

impl Context {
    pub fn new(repo_url: &str, cargo_dist: bool) -> Result<Self> {
        let repo = GithubRepo::from_url(repo_url)?;
        let (releases, has_prereleases, latest_dist_release) =
            Self::fetch_all_releases(&repo, cargo_dist)?;
        Ok(Self {
            repo,
            releases,
            has_prereleases,
            latest_dist_release,
        })
    }

    pub fn fetch_all_releases(
        repo: &GithubRepo,
        cargo_dist: bool,
    ) -> Result<(Vec<Release>, bool, Option<DistRelease>)> {
        let gh_releases =
            tokio::runtime::Handle::current().block_on(GithubRelease::fetch_all(repo))?;
        let all =
            tokio::runtime::Handle::current().block_on(futures_util::future::try_join_all(
                gh_releases
                    .into_iter()
                    .map(|gh_release| Release::new(gh_release, cargo_dist)),
            ))?;

        let mut has_prereleases = false;
        let mut warned = false;
        let mut latest_dist_release = None;
        let mut latest_dist_prerelease = None;
        let mut latest_stable_release = None;

        for (idx, release) in all.iter().enumerate() {
            let is_prerelease = release.source.prerelease;
            // Make note of whether we've found prereleases or stable releases yet
            if is_prerelease {
                if !has_prereleases {
                    let msg = "Detected pre-releases...";
                    Message::new(MessageType::Info, msg).print();
                    has_prereleases = true;
                }
            } else if latest_stable_release.is_none() {
                latest_stable_release = Some(idx);
            }

            // Special handling of dist-manifest.json
            if release.manifest.is_some() {
                if cargo_dist {
                    // cargo-dist is enabled, so we want to find the latest stable release
                    // or, failing that, the latest prerelease.
                    if is_prerelease {
                        if latest_dist_prerelease.is_none() && latest_dist_release.is_none() {
                            latest_dist_prerelease = Some(idx);
                        }
                    } else if latest_dist_release.is_none() {
                        latest_dist_release = Some(idx);
                    }
                } else if !warned {
                    // We found a dist-manifest but they didn't enable cargo-dist support, encourage them to do so
                    let msg = "You have not configured cargo-dist yet we detected dist-manifests in your releases. Is this intended?";
                    Message::new(MessageType::Warning, msg).print();
                    warned = true;
                }
            }
        }

        // If we found any kind of stable release, then throw out any prerelease fallbacks.
        // This prevents us recommending an unstable dist-release over a stable non-dist-release.
        if latest_stable_release.is_some() {
            latest_dist_prerelease = None;
        }

        // If we found a stable cargo-dist release, but there's even newer stable releases
        // that don't use cargo-dist, we're going to prefer the cargo-dist one, but we should
        // warn the user that things are wonky
        if latest_dist_release.is_some() && latest_dist_release != latest_stable_release {
            let dist_rel = &all[latest_dist_prerelease.unwrap()].source.tag_name;
            let stable_rel = &all[latest_stable_release.unwrap()].source.tag_name;
            let msg = format!("You have newer stable Github Releases ({}) than your latest cargo-dist Release ({}). Is this intended? (We're going to prefer the cargo-dist one.)", stable_rel, dist_rel);
            Message::new(MessageType::Warning, &msg).print();
        }

        // If we found a stable dist release, use that
        // otherwise use an unstable one, if we found it
        let dist_release = latest_dist_release.or(latest_dist_prerelease).map(|idx| {
            let release = &all[idx];
            DistRelease {
                manifest: release.manifest.as_ref().unwrap().clone(),
                source: release.source.clone(),
            }
        });

        Ok((all, has_prereleases, dist_release))
    }
}
