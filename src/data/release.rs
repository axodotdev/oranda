use cargo_dist_schema::DistManifest;

use crate::data::github::GithubRelease;
use crate::errors::*;

#[derive(Clone, Debug)]
pub struct Release {
    pub manifest: Option<DistManifest>,
    pub source: GithubRelease,
}

impl Release {
    pub fn new(gh_release: GithubRelease) -> Result<Self> {
        let manifest = Self::fetch_manifest(gh_release.asset_url("dist_manifest.json"))?;
        Ok(Self {
            manifest,
            source: gh_release,
        })
    }

    fn fetch_manifest(url: Option<&str>) -> Result<Option<DistManifest>> {
        if let Some(manifest_url) = url {
            match reqwest::blocking::get(manifest_url)?.error_for_status() {
                Ok(resp) => match resp.json::<DistManifest>() {
                    Ok(manifest) => Ok(Some(manifest)),
                    Err(e) => Err(OrandaError::CargoDistManifestParseError {
                        url: manifest_url.to_string(),
                        details: e,
                    }),
                },
                Err(e) => Err(OrandaError::CargoDistManifestFetchError {
                    url: manifest_url.to_string(),
                    status_code: e.status().unwrap_or(reqwest::StatusCode::BAD_REQUEST),
                }),
            }
        } else {
            Ok(None)
        }
    }
}
