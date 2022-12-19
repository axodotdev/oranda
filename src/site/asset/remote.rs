use std::path::{Path, PathBuf};

use crate::errors::*;

pub fn copy(dist_dir: &str, origin_path: &str, label: &str) -> Result<PathBuf> {
    let dist_path = dist_path(dist_dir, origin_path)?;
    let mut file = std::fs::File::create("image.png")?;
    reqwest::blocking::get(origin_path)?.copy_to(&mut file)?;
    Ok(dist_path)
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
