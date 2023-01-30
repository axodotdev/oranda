use crate::errors::*;
use crate::site::javascript::detect_os::OS_SCRIPT;
use axohtml::elements::script;
use axohtml::html;
pub mod detect_os;

pub fn get_os_script(dist_dir: &String) -> Result<Box<script<String>>> {
    const FILE_NAME: &str = "detect_os.js";
    let script_path = format!("{}/{}", dist_dir, FILE_NAME);
    let asset = axoasset::local::LocalAsset::new(&script_path, OS_SCRIPT.into());

    axoasset::local::LocalAsset::write(&asset, dist_dir.as_str())?;
    Ok(html!(<script src=FILE_NAME />))
}
