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
