use crate::errors::{OrandaError, Result};
use camino::{Utf8Path, Utf8PathBuf};

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

/// Creates a workspace-safe relative path. Takes the following arguments:
/// - The root path of the workspace (or single project)
/// - An optional workspace member path
/// - The path itself, usually extracted from the configuration
/// Member path and the path itself can be relative or absolute .
/// The function will attempt to lazily build the smallest possible absolute and canonicalized path,
/// before diffing it with the root path to create a path that's always relative to the workspace root.
///
/// Some example scenarios:
/// 1. root path = "/my/directory", member path = None, path = "myfile.md"
///    Output = "myfile.md"
/// 2. root path = "/my/directory", member path = "member", path = "myfile.md"
///    Output = "member/myfile.md"
/// 3. root path= "/my/directory", member path = "/my/directory/member", path = "../root.md"
///    Output = "root.md"
pub fn determine_path(
    root_path: impl AsRef<Utf8Path>,
    member_path: &Option<impl AsRef<Utf8Path>>,
    path: impl AsRef<Utf8Path>,
) -> Result<Utf8PathBuf> {
    let root_path = root_path.as_ref();
    let member_path = member_path.as_ref().map(|p| p.as_ref());
    let path = path.as_ref();
    if path.is_absolute() {
        // If absolute, return the path
        return Ok(path.to_owned());
    }

    // If the member path exists and is absolute, construct `member_path/path`.
    // If the member path exists and isn't absolute, construct `root_path/member_path/path`.
    // If the member path doesn't exist, construct `root_path/path`.
    let path_plus_member = if let Some(member_path) = member_path {
        if member_path.is_absolute() {
            let mut owned = Utf8PathBuf::new();
            owned.push(member_path);
            owned.push(path);
            owned.canonicalize_utf8()
        } else {
            let mut owned = Utf8PathBuf::new();
            owned.push(root_path);
            owned.push(member_path);
            owned.push(path);
            owned.canonicalize_utf8()
        }
    } else {
        let mut owned = Utf8PathBuf::new();
        owned.push(root_path);
        owned.push(path);
        owned.canonicalize_utf8()
    };

    match path_plus_member {
        Ok(path) => {
            // Create a relative path from difference between root and created path.
            Ok(
                pathdiff::diff_utf8_paths(&path, root_path).ok_or(OrandaError::PathdiffError {
                    root_path: root_path.to_string(),
                    path: path.to_string(),
                })?,
            )
        }
        Err(_) => {
            // The path probably doesn't exist, return an empty path
            Ok(Utf8PathBuf::new())
        }
    }
}
