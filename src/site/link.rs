pub fn generate(path_prefix: &Option<String>, file_name: &str) -> String {
    if let Some(prefix) = &path_prefix {
        format!("/{}/{}", prefix, file_name)
    } else {
        format!("/{}", file_name)
    }
}
