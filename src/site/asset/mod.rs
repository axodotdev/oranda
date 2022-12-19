use std::path::PathBuf;

use crate::errors::*;

mod local;
mod remote;

pub fn copy(dist_dir: &str, origin_path: &str, label: &str) -> Result<PathBuf> {
    if is_remote(origin_path) {
        remote::copy(dist_dir, origin_path, label)
    } else {
        local::copy(dist_dir, origin_path, label)
    }
}

fn is_remote(origin_path: &str) -> bool {
    false
}
