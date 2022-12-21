use std::path::PathBuf;

use crate::errors::*;

mod local;
mod remote;

pub fn copy(dist_dir: &str, origin_path: &str, label: &str) -> Result<PathBuf> {
    if is_remote(origin_path, label)? {
        remote::copy(dist_dir, origin_path, label)
    } else {
        local::copy(dist_dir, origin_path, label)
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
