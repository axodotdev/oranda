use axoasset::LocalAsset;
use axohtml::{elements::script, html};
use camino::Utf8Path;

use crate::errors::*;
use crate::site::link;

pub mod analytics;
mod artifacts;

pub fn build_os_script(path_prefix: &Option<String>) -> String {
    let script_url = link::generate(path_prefix, "artifacts.js");
    let script: Box<script<String>> = html!(<script src=script_url />);
    script.to_string()
}

pub fn write_os_script(dist_dir: &Utf8Path) -> Result<()> {
    LocalAsset::write_new(artifacts::SCRIPT, dist_dir.join("artifacts.js"))?;
    Ok(())
}
