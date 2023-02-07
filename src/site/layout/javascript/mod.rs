mod detect_os;

use crate::errors::*;
use crate::site::link;
use axohtml::elements::script;
use axohtml::html;

pub fn build_os_script(path_prefix: &Option<String>) -> Result<Box<script<String>>> {
    const FILE_NAME: &str = "detect_os.js";
    let script_url = link::generate(path_prefix, FILE_NAME.to_string());
    Ok(html!(<script src=script_url />))
}

pub fn write_os_script(dist_dir: &String) -> Result<()> {
    const FILE_NAME: &str = "detect_os.js";
    let script_path = format!("{}/{}", dist_dir, FILE_NAME);
    let asset = axoasset::local::LocalAsset::new(&script_path, detect_os::SCRIPT.into());
    axoasset::local::LocalAsset::write(&asset, dist_dir.as_str())?;
    Ok(())
}
