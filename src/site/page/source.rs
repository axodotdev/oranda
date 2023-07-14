use crate::errors::{OrandaError, Result};
use camino::Utf8PathBuf;
use std::path::Path;

pub fn is_markdown(file: &str) -> bool {
    let file_path = Path::new(&file);
    match file_path.extension() {
        None => false,
        Some(ext) => ext.to_string_lossy().to_lowercase() == "md",
    }
}

pub fn get_filename_with_dir(file: &str) -> Result<Option<Utf8PathBuf>> {
    // Try diffing with the execution directory in case the user has provided an absolute-ish
    // path, in order to obtain the relative-to-dir path segment
    let cur_dir = Utf8PathBuf::from_path_buf(std::env::current_dir()?)
        .map_err(|_| OrandaError::Other("Unable to read your current working directory.".into()))?;
    let path = if let Some(path) = pathdiff::diff_utf8_paths(file, cur_dir) {
        path
    } else {
        Utf8PathBuf::from(file)
    };

    Ok(Some(path.with_extension("")))
}
