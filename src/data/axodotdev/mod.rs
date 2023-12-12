use crate::errors::*;

use axoproject::GithubRepo;
use gazenot::{Gazenot, PublicRelease, ReleaseAsset};
use serde::{Deserialize, Serialize};

use super::artifacts::{File, ReleaseArtifacts};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AxoResponse {
    pub success: bool,
    pub result: Vec<AxoRelease>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AxoRelease {
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub version: String,
    pub prerelease: bool,
    pub created_at: String,
    pub assets: Vec<AxoReleaseAsset>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AxoReleaseAsset {
    pub browser_download_url: String,
    pub name: String,
    pub uploaded_at: String,
}

impl AxoRelease {
    pub async fn fetch_all(package_name: &str, repo: &GithubRepo) -> Result<Vec<AxoRelease>> {
        let abyss = Gazenot::new_unauthed("github".to_string(), repo.owner.clone())?;
        let list = abyss
            .list_releases_many(vec![package_name.to_string()])
            .await?;
        let list = list
            .into_iter()
            .find(|r| r.package_name == package_name)
            .ok_or(OrandaError::AxoReleasesFetchError)?;

        Ok(list.releases.into_iter().map(|r| r.into()).collect())
    }

    pub fn has_dist_manifest(&self) -> bool {
        self.assets.iter().any(|a| a.name == "dist-manifest.json")
    }

    pub fn asset_url<'a>(&'a self, asset_name: &'a str) -> Option<&'a str> {
        for asset in &self.assets {
            if asset.name == asset_name {
                return Some(&asset.browser_download_url);
            }
        }
        None
    }

    pub fn repo_has_releases(name: &str, repo: &GithubRepo) -> Result<bool> {
        if let Ok(releases) =
            tokio::runtime::Handle::current().block_on(AxoRelease::fetch_all(name, repo))
        {
            if releases.is_empty() {
                Ok(false)
            } else {
                Ok(true)
            }
        } else {
            let warning = OrandaError::ReleasesCheckFailed {
                repo: repo.to_string(),
            };
            eprintln!("{:?}", miette::Report::new(warning));
            Ok(false)
        }
    }
}

impl ReleaseArtifacts {
    pub fn add_axodotdev(&mut self, release: &AxoRelease) {
        for asset in &release.assets {
            let file = File {
                name: asset.name.clone(),
                download_url: asset.browser_download_url.clone(),
                view_path: None,
                checksum_file: None,
                infer: true,
            };
            self.add_file(file);
        }
    }
}

impl From<ReleaseAsset> for AxoReleaseAsset {
    fn from(value: ReleaseAsset) -> Self {
        let ReleaseAsset {
            browser_download_url,
            name,
            uploaded_at,
        } = value;

        Self {
            browser_download_url,
            name,
            uploaded_at,
        }
    }
}
impl From<PublicRelease> for AxoRelease {
    fn from(value: PublicRelease) -> Self {
        let PublicRelease {
            tag_name,
            version,
            name,
            body,
            prerelease,
            created_at,
            assets,
        } = value;

        let assets: Vec<AxoReleaseAsset> = assets.into_iter().map(|a| a.into()).collect();

        Self {
            tag_name,
            version,
            name,
            body,
            prerelease,
            created_at,
            assets,
        }
    }
}
