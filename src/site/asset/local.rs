use std::fs;
use std::path::{Path, PathBuf};

use crate::errors::*;

pub fn copy(dist_dir: &str, origin_path: &str, label: &str) -> Result<PathBuf> {
    let dist_path = dist_path(dist_dir, origin_path, label)?;
    match Path::new(&origin_path).try_exists() {
        Ok(_) => match fs::copy(origin_path, &dist_path) {
            Ok(_) => Ok(dist_path),
            Err(details) => Err(OrandaError::LocalAssetCopyFailed {
                asset: label.to_string(),
                origin_path: origin_path.to_string(),
                dist_path: dist_path.display().to_string(),
                details: details.to_string(),
            }),
        },
        Err(details) => Err(OrandaError::LocalAssetNotFound {
            asset: label.to_string(),
            origin_path: origin_path.to_string(),
            details: details.to_string(),
        }),
    }
}

fn filename(origin_path: &str, label: &str) -> Result<PathBuf> {
    if let Some(filename) = Path::new(origin_path).file_name() {
        Ok(filename.into())
    } else {
        Err(OrandaError::LocalAssetMissingFilename {
            asset: label.to_string(),
            origin_path: origin_path.to_string(),
        })
    }
}

fn dist_path(dist_dir: &str, origin_path: &str, label: &str) -> Result<PathBuf> {
    let filename = filename(origin_path, label)?;
    Ok(Path::new(&dist_dir).join(filename))
}
