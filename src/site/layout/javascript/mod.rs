mod detect_os;

use crate::errors::*;
use crate::site::path;
use axohtml::elements::script;
use axohtml::html;

pub fn get_os_script(
    dist_dir: &String,
    path_prefix: &Option<String>,
) -> Result<Box<script<String>>> {
    const FILE_NAME: &str = "detect_os.js";
    let script_path = format!("{}/{}", dist_dir, FILE_NAME);
    let asset = axoasset::local::LocalAsset::new(&script_path, detect_os::SCRIPT.into());

    axoasset::local::LocalAsset::write(&asset, dist_dir.as_str())?;
    let script_url = path::generate_prefix_link(path_prefix, FILE_NAME.to_string());
    Ok(html!(<script src=script_url />))
}
