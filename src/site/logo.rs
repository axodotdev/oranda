use std::path::PathBuf;

use crate::errors::*;
use crate::site::asset;

pub fn fetch(dist_dir: &str, origin_path: &str) -> Result<Option<PathBuf>> {
    asset::copy(dist_dir, origin_path, "Logo")
}
