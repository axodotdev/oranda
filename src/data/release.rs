use cargo_dist_schema::DistManifest;

use crate::data::{cargo_dist, github::GithubRelease};
use crate::errors::*;
use crate::message::{Message, MessageType};

#[derive(Clone, Debug)]
pub struct Release {
    pub manifest: Option<DistManifest>,
    pub source: GithubRelease,
}

impl Release {
    pub fn new(gh_release: GithubRelease, cargo_dist: bool) -> Result<Self> {
        let manifest = if cargo_dist {
            Self::fetch_manifest(gh_release.asset_url(cargo_dist::MANIFEST_FILENAME))?
        } else {
            None
        };
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
                    Err(e) => {
                        let msg = format!("Failed to parse dist-manifest at {manifest_url}.\nDetails:{e}\n\nSkipping...");
                        Message::new(MessageType::Warning, &msg).print();
                        Ok(None)
                    }
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
