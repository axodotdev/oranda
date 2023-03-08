use std::{ffi::OsStr, path::Path};

pub fn is_markdown(file: &str) -> bool {
    let file_path = Path::new(&file);
    let extension = file_path.extension().and_then(OsStr::to_str);

    extension == Some("md") || extension == Some("MD")
}
