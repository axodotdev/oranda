use std::path::PathBuf;

use crate::errors::*;
use crate::site::asset;

pub fn fetch(dist_dir: &str, origin_path: &str) -> Result<PathBuf> {
    Ok(asset::copy(origin_path, "logo", dist_dir)?)
}
