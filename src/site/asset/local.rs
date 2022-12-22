use std::fs;
use std::path::{Path, PathBuf};

use crate::errors::*;

pub struct LocalAsset {
    dist_dir: String,
    origin_path: String,
    label: String,
}

impl LocalAsset {
    pub fn copy(&self) -> Result<PathBuf> {
        let dist_path = self.dist_path()?;
        match Path::new(&self.origin_path).try_exists() {
            Ok(_) => match fs::copy(self.origin_path, &dist_path) {
                Ok(_) => Ok(dist_path),
                Err(details) => Err(OrandaError::LocalAssetCopyFailed {
                    asset: self,
                    dist_path: dist_path.display().to_string(),
                    details: details.to_string(),
                }),
            },
            Err(details) => Err(OrandaError::LocalAssetNotFound {
                asset: self,
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

    fn dist_path(&self) -> Result<PathBuf> {
        let filename = filename(self.origin_path, self.label)?;
        Ok(Path::new(&dist_dir).join(filename))
    }
}
