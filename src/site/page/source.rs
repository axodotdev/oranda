use std::path::Path;

pub fn is_markdown(file: &str) -> bool {
    let file_path = Path::new(&file);
    match file_path.extension() {
        None => false,
        Some(ext) => ext.to_str() == Some("md") || ext.to_str() == Some("MD"),
    }
}
