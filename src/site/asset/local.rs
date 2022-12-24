use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use crate::site::asset::error::*;

#[derive(Debug)]
pub struct LocalAsset {
    pub origin_path: String,
    pub label: String,
    pub contents: Vec<u8>,
}

impl fmt::Display for LocalAsset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} asset at path, {}, ", self.label, self.origin_path)
    }
}

impl LocalAsset {
    pub fn load(origin_path: &str, label: &str) -> Result<LocalAsset> {
        match Path::new(origin_path).try_exists() {
            Ok(_) => {
                let contents = fs::read(origin_path)?;
                Ok(LocalAsset {
                    origin_path: origin_path.to_string(),
                    label: label.to_string(),
                    contents,
                })
            }
            Err(details) => Err(AxoassetError::LocalAssetNotFound {
                origin_path: origin_path.to_string(),
                label: label.to_string(),
                details: details.to_string(),
            }),
        }
    }

    pub fn write(&self, dist_dir: &str) -> Result<PathBuf> {
        let dist_path = self.dist_path(dist_dir)?;
        match fs::write(&dist_path, &self.contents) {
            Ok(_) => Ok(dist_path),
            Err(details) => Err(AxoassetError::LocalAssetWriteFailed {
                asset: self.to_string(),
                dist_path: dist_path.display().to_string(),
                details: details.to_string(),
            }),
        }
    }

    pub fn copy(origin_path: &str, dist_dir: &str, label: &str) -> Result<PathBuf> {
        LocalAsset::load(origin_path, label)?.write(dist_dir)
    }

    fn filename(&self) -> Result<PathBuf> {
        if let Some(filename) = Path::new(&self.origin_path).file_name() {
            Ok(filename.into())
        } else {
            Err(AxoassetError::LocalAssetMissingFilename {
                asset: self.to_string(),
            })
        }
    }

    fn dist_path(&self, dist_dir: &str) -> Result<PathBuf> {
        let filename = self.filename()?;
        Ok(Path::new(dist_dir).join(filename))
    }
}
