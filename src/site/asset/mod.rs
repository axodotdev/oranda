use std::path::PathBuf;

use crate::errors::*;

mod local;
mod remote;

pub enum Asset {
    LocalAsset(local::LocalAsset),
    RemoteAsset(remote::RemoteAsset),
}

pub fn copy(dist_dir: &str, origin_path: &str, label: &str) -> Result<PathBuf> {
    if is_remote(origin_path, label)? {
        remote::RemoteAsset {
            dist_dir: dist_dir.to_string(),
            origin_path: origin_path.to_string(),
            label: label.to_string(),
        }
        .copy()
    } else {
        local::LocalAsset {
            dist_dir: dist_dir.to_string(),
            origin_path: origin_path.to_string(),
            label: label.to_string(),
        }
        .copy()
    }
}

fn is_remote(origin_path: &str, label: &str) -> Result<bool> {
    if origin_path.starts_with("http") {
        match origin_path.parse() {
            Ok(url) => {
                if is_http(url) {
                    Ok(true)
                } else {
                    Err(OrandaError::RemoteAssetPathSchemeNotSupported {
                        asset: label.to_string(),
                        origin_path: origin_path.to_string(),
                    })
                }
            }
            Err(details) => Err(OrandaError::RemoteAssetPathParseError {
                asset: label.to_string(),
                origin_path: origin_path.to_string(),
                details: details.to_string(),
            }),
        }
    } else {
        Ok(false)
    }
}

fn is_http(url: url::Url) -> bool {
    url.scheme() == "https" || url.scheme() == "http"
}
