use std::fs;
use std::path::{Path, PathBuf};

use crate::errors::*;

pub fn fetch(dist_dir: &str, origin_path: &str) -> Result<Option<PathBuf>> {
    if let Some(filename) = Path::new(origin_path).file_name() {
        let dist_path = Path::new(&dist_dir).join(filename);
        if Path::new(&origin_path).exists() {
            fs::copy(&origin_path, &dist_path)?;
        } else {
            return Err(OrandaError::FileNotFound {
                filedesc: "Logo".to_owned(),
                path: origin_path.to_string(),
            });
        }
        return Ok(Some(dist_path));
    } else {
        return Err(OrandaError::Other(
            "provided path has no filename".to_string(),
        ));
    }
}
