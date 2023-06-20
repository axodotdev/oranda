use camino::Utf8PathBuf;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyValExt};
use crate::errors::*;

const FUNDING_YML_PATH: &str = "./.github/funding.yml";

/// User-facing options for building and integrating your mdbook
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MdBookOpts {
    /// Path to the mdbook
    ///
    /// If not set we will attempt to auto-detect
    pub path: Option<String>,
    /// Whether to enable the custom oranda/axo theme
    pub theme: Option<bool>,
}

/// Config for us building and integrating your mdbook
#[derive(Debug)]
pub struct MdBookConfig {
    pub path: String,
    /// Whether to enable the custom oranda/axo theme
    pub theme: bool,
}

impl Default for MdBookConfig {
    fn default() -> Self {
        Self {
            path: FUNDING_YML_PATH.to_string(),
            theme: true,
        }
    }
}

impl ApplyLayer for MdBookConfig {
    fn apply_layer(&mut self, layer: MdBookOpts) {
        self.path.apply_val(layer.path);
        self.theme.apply_val(layer.theme)
    }
}

impl MdBookOpts {
    /// If mdbook is enabled but the path isn't set, we try to find it
    ///
    /// If we fail, we set mdbook to None to disable it.
    pub fn find(config: &mut Option<Self>) -> Result<()> {
        // if a user hasn't specifically turned this feature off
        // (defaults to Some)
        let Some(this) = config else {
            return Ok(())
        };

        if this.path.is_none() {
            // Ok time to auto-detect, try these dirs for a book.toml
            let possible_paths = vec!["./", "./book/", "./docs/"];
            for book_dir in possible_paths {
                let book_path = Utf8PathBuf::from(book_dir).join("book.toml");
                if book_path.exists() {
                    // nice, use it
                    this.path = Some(book_dir.to_owned());
                }
            }
        }

        let cant_find_file = this.path.is_none();
        let has_user_config = this.theme.is_some();

        if cant_find_files {
            // The config is unusable.
            //
            // * If the user customized stuff, error out because they clearly wanted this to work
            // * Otherwise, just disable the feature
            if has_user_config {
                return Err(OrandaError::MdBookConfigInvalid);
            } else {
                *config = None;
            }
        }
        Ok(())
    }
}
