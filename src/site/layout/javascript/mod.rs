mod artifacts;

use crate::errors::*;
use crate::site::link;

use axoasset::LocalAsset;
use axohtml::elements::script;
use axohtml::html;

pub fn build_os_script(path_prefix: &Option<String>) -> Result<Box<script<String>>> {
    let script_url = link::generate(path_prefix, "artifacts.js");
    Ok(html!(<script src=script_url />))
}

pub fn write_os_script(dist_dir: &str) -> Result<()> {
    LocalAsset::write_new(artifacts::SCRIPT, "artifacts.js", dist_dir)?;
    Ok(())
}
