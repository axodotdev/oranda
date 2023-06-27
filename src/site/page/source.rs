use camino::Utf8PathBuf;
use std::{ffi::OsStr, path::Path};

pub fn is_markdown(file: &str) -> bool {
    let file_path = Path::new(&file);
    match file_path.extension() {
        None => false,
        Some(ext) => ext.to_string_lossy().to_lowercase() == "md",
    }
}

pub fn get_filename(file: &str) -> Option<&OsStr> {
    let file_path = Path::new(file);
    file_path.file_stem()
}

pub fn get_filename_with_dir(file: &str) -> Option<Utf8PathBuf> {
    let cur_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return None,
    };
    // Try diffing with the execution directory in case the user has provided an absolute-ish
    // path, in order to obtain the relative-to-dir path segment
    let path = if let Some(path) =
        pathdiff::diff_utf8_paths(file, Utf8PathBuf::from_path_buf(cur_dir).unwrap())
    {
        path
    } else {
        Utf8PathBuf::from(file)
    };

    Some(path.with_extension(""))
}
