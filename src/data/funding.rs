use crate::errors::Result;
use axoasset::LocalAsset;
use camino::Utf8PathBuf;

/// Loads the FUNDING.yml file from the local file system. Returns
/// `Ok(Some(String))` if the file was found, and `Ok(None)` if it
/// wasn't.
pub fn load_funding_file() -> Result<Option<String>> {
    load_generic_file(".github/FUNDING.yml")
}

/// Loads a `funding.md` file from the root directory, to serve as documentation
/// for the generated funding page. Returns the same as the above function.
pub fn load_funding_docs() -> Result<Option<String>> {
    // FIXME: Do we want this file to be FUNDING.md?
    load_generic_file("funding.md")
}

fn load_generic_file(path: &str) -> Result<Option<String>> {
    let path = Utf8PathBuf::from(path);
    if path.exists() {
        Ok(Some(LocalAsset::load_string(path)?))
    } else {
        Ok(None)
    }
}
