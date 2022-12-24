use std::path::PathBuf;

pub mod error;
pub(crate) mod local;
pub(crate) mod remote;

use error::*;

pub enum Asset {
    LocalAsset(local::LocalAsset),
    RemoteAsset(remote::RemoteAsset),
}

pub fn load(origin_path: &str, label: &str) -> Result<Asset> {
    if is_remote(origin_path, label)? {
        Ok(Asset::RemoteAsset(remote::RemoteAsset::load(
            origin_path,
            label,
        )?))
    } else {
        Ok(Asset::LocalAsset(local::LocalAsset::load(
            origin_path,
            label,
        )?))
    }
}

pub fn copy(origin_path: &str, label: &str, dist_dir: &str) -> Result<PathBuf> {
    if is_remote(origin_path, label)? {
        remote::RemoteAsset::copy(origin_path, label, dist_dir)
    } else {
        local::LocalAsset::copy(origin_path, label, dist_dir)
    }
}

fn is_remote(origin_path: &str, label: &str) -> Result<bool> {
    if origin_path.starts_with("http") {
        match origin_path.parse() {
            Ok(url) => {
                if is_http(url) {
                    Ok(true)
                } else {
                    Err(AxoassetError::RemoteAssetPathSchemeNotSupported {
                        label: label.to_string(),
                        origin_path: origin_path.to_string(),
                    })
                }
            }
            Err(details) => Err(AxoassetError::RemoteAssetPathParseError {
                label: label.to_string(),
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
