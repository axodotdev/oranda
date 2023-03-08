use std::{borrow::Cow, path::Path};

pub fn is_markdown(file: &str) -> bool {
    let file_path = Path::new(&file);
    match file_path.extension() {
        None => false,
        Some(ext) => ext.to_string_lossy().to_lowercase() == "md",
    }
}

pub fn get_filename(file: &str) -> Cow<str> {
    let file_path = Path::new(file);
    file_path
        .file_stem()
        .unwrap_or(file_path.as_os_str())
        .to_string_lossy()
}
