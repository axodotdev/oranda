mod detect_os;

use crate::errors::*;
use axohtml::elements::script;
use axohtml::html;

pub fn build_os_script() -> Result<Box<script<String>>> {
    const FILE_NAME: &str = "detect_os.js";
    Ok(html!(<script src=FILE_NAME />))
}

pub fn write_os_script(dist_dir: &String) -> Result<()> {
    const FILE_NAME: &str = "detect_os.js";
    let script_path = format!("{}/{}", dist_dir, FILE_NAME);
    let asset = axoasset::local::LocalAsset::new(&script_path, detect_os::SCRIPT.into());
    axoasset::local::LocalAsset::write(&asset, dist_dir.as_str())?;
    Ok(())
}
