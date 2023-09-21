use crate::config::Config;
use crate::errors::{OrandaError, Result};
use camino::{Utf8Path, Utf8PathBuf};

pub fn generate_relative(path_prefix: &Option<String>, file_name: &str) -> String {
    // NOTE: intentionally no leading `/` here because it makes camino add a phantom `/` or `\`
    // to the front of the path when getting its components (because it's trying to tell us
    // the path is absolute, and we already know that).
    let path = if let Some(prefix) = &path_prefix {
        format!("{}/{}", prefix, file_name)
    } else {
        file_name.to_owned()
    };

    sanitize_path(&path, file_name)
}

/// Generates an absolute path to the end-user hosted version of a file. Returns an option, in case
/// the `url` configuration option wasn't set.
pub fn generate_absolute(config: &Config, file_name: &str) -> Option<String> {
    let url = "http://127.0.0.1:7979"; // FIXME: wip
    let url = url.trim_end_matches('/');
    let path = if let Some(prefix) = &config.build.path_prefix {
        format!("{}/{}", prefix, file_name)
    } else {
        file_name.to_owned()
    };

    let sanitized_path = sanitize_path(&path, file_name);
    Some(format!("{}{}", url, sanitized_path))
}

fn sanitize_path(path: &str, file_name: &str) -> String {
    // Break the url up into its segments, and precent-encode each part,
    // prepending a `/` before each part to make the resulting URL absolute
    let path = Utf8PathBuf::from(path);
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
