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
        let gh_releases = GithubRelease::fetch_all(repo)?;
        let mut has_prereleases = false;
        let mut found_latest_dist_release = false;
        let mut latest_dist_release = None;
        let mut all = vec![];
        for gh_release in gh_releases {
            if gh_release.prerelease && !has_prereleases {
                let msg = "Detected pre-releases...";
                Message::new(MessageType::Info, msg).print();
                has_prereleases = true
            }
            if cargo_dist && !found_latest_dist_release && gh_release.has_dist_manifest() {
                let release = Release::new(gh_release.clone(), cargo_dist)?;
                if let Some(manifest) = release.manifest {
                    latest_dist_release = Some(DistRelease {
                        manifest,
                        source: gh_release.clone(),
                    });
                }
                found_latest_dist_release = latest_dist_release.is_some();
            }
            all.push(Release::new(gh_release, cargo_dist)?)
        }
        Ok((all, has_prereleases, latest_dist_release))
    }
}
