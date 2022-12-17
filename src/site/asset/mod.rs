use std::fs;
use std::path::{Path, PathBuf};

use crate::errors::*;

pub fn copy(dist_dir: &str, origin_path: &str, label: &str) -> Result<Option<PathBuf>> {
    let dist_path = dist_path(dist_dir, origin_path)?;
    if Path::new(&origin_path).exists() {
        fs::copy(origin_path, &dist_path)?;
    } else {
        return Err(OrandaError::FileNotFound {
            filedesc: label.to_owned(),
            path: origin_path.to_string(),
        });
    }
    Ok(Some(dist_path))
}

fn filename(origin_path: &str) -> Result<PathBuf> {
    if let Some(filename) = Path::new(origin_path).file_name() {
        Ok(filename.into())
    } else {
        Err(OrandaError::Other(
            "provided path has no filename".to_string(),
        ))
    }
}

fn dist_path(dist_dir: &str, origin_path: &str) -> Result<PathBuf> {
    let filename = filename(origin_path)?;
    Ok(Path::new(&dist_dir).join(filename))
}
