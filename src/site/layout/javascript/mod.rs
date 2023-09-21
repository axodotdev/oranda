use axoasset::LocalAsset;
use camino::Utf8Path;

use crate::errors::*;
use crate::site::link;

pub mod analytics;

const ARTIFACTS_SCRIPT_SOURCE: &str = include_str!("./artifacts.js");

pub fn build_os_script_path(path_prefix: &Option<String>) -> String {
    link::generate_relative(path_prefix, "artifacts.js")
}

pub fn write_os_script(dist_dir: &Utf8Path) -> Result<()> {
    LocalAsset::write_new(ARTIFACTS_SCRIPT_SOURCE, dist_dir.join("artifacts.js"))?;
    Ok(())
}
