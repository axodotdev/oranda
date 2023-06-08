use crate::errors::Result;
use axoasset::LocalAsset;
use camino::Utf8PathBuf;

/// Loads the FUNDING.yml file from the local file system. Returns
/// `Ok(Some(String))` if the file was found, and `Ok(None)` if it
/// wasn't.
pub fn load_funding_file() -> Result<Option<String>> {
    let path = Utf8PathBuf::from(".github/FUNDING.yml");
    if path.exists() {
        Ok(Some(LocalAsset::load_string(path)?))
    } else {
        Ok(None)
    }
}
