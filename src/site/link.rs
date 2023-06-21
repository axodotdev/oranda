use camino::Utf8PathBuf;

pub fn generate(path_prefix: &Option<String>, file_name: &str) -> String {
    // NOTE: intentionally no leading `/` here because it makes camino add a phantom `/` or `\`
    // to the front of the path when getting its components (because it's trying to tell us
    // the path is absolute, and we already know that).
    let url = if let Some(prefix) = &path_prefix {
        format!("{}/{}", prefix, file_name)
    } else {
        file_name.to_owned()
    };

    // Break the url up into its segments, and precent-encode each part,
    // prepending a `/` before each part to make the resulting URL absolute
    let path = Utf8PathBuf::from(url);
    let mut output = String::new();
    for part in path.components() {
        output.push('/');
        output.extend(url::form_urlencoded::byte_serialize(
            part.as_str().as_bytes(),
        ));
    }

    // re-add a trailing slash if original input had it
    if file_name.ends_with('/') {
        output.push('/');
    }

    output
}
