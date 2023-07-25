use camino::Utf8PathBuf;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::config::{ApplyLayer, ApplyOptExt, ApplyValExt};
use crate::errors::*;

/// Config for us building and integrating your mdbook (complete version)
#[derive(Debug, Clone)]
pub struct MdBookConfig {
    /// Path to the mdbook
    ///
    /// If not set we will attempt to auto-detect
    pub path: Option<String>,
    /// Whether to enable the custom oranda/axo theme
    pub theme: bool,
}

/// The config for building and embedding an mdbook on your site
#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MdBookLayer {
    /// Path to the mdbook (the directory containing book.toml)
    ///
    /// If not set we will attempt to auto-detect this by trying
    ///  "./", "./book/", and "./docs/".
    pub path: Option<String>,
    /// Whether to enable oranda's customized mdbook theme that unifies
    /// with your oranda theme.
    ///
    /// If enabled we will use mdbook's custom themeing system to overwrite
    /// most of the hbs/css/js file mdbook defines to add hooks for our
    /// custom themes to be enabled and defaulted on. The existing mdbook
    /// themes will still be available and should work normally.
    ///
    /// Unfortunately this means that `mdbook build` won't produce the same
    /// results as `oranda build`. In the future we may introduce a way
    /// to "vendor" the changes oranda makes so that `mdbook build` behaves
    /// the same. This should be possible because we mostly use officially
    /// supported mdbook settings when changing the theme (the only exception
    /// being we add an extra css file for our theme's syntax highlighter).
    ///
    /// Any other mdbook settings should ideally be preserved/respected.
    ///
    /// If the theme has a paired dark/light variant, that variant will
    /// also be made available, although we won't respect mdbook's builtin
    /// preferred dark-mode setting, to ensure the rest of your oranda site
    /// always looks the same (this may be improved when the rest of oranda
    /// gets richer support for light/dark-mode).
    ///
    /// defaults to true
    pub theme: Option<bool>,
}

impl Default for MdBookConfig {
    fn default() -> Self {
        MdBookConfig {
            path: None,
            theme: true,
        }
    }
}
impl ApplyLayer for MdBookConfig {
    type Layer = MdBookLayer;
    fn apply_layer(&mut self, layer: Self::Layer) {
        // This is intentionally written slightly cumbersome to make you update this
        let MdBookLayer { path, theme } = layer;
        self.path.apply_opt(path);
        self.theme.apply_val(theme);
    }
}

impl MdBookConfig {
    /// If mdbook is enabled but the path isn't set, we try to find it
    ///
    /// If we fail, we set mdbook to None to disable it.
    pub fn find_paths(config: &mut Option<MdBookConfig>) -> Result<()> {
        // If this is None, we were force-disabled and shouldn't auto-detect
        let Some(this) = config else {
            return Ok(());
        };

        if this.path.is_none() {
            // Ok time to auto-detect, try these dirs for a book.toml
            let possible_paths = vec!["./", "./book/", "./docs/"];
            for book_dir in possible_paths {
                let book_path = Utf8PathBuf::from(book_dir).join("book.toml");
                if book_path.exists() {
                    // nice, use it
                    this.path = Some(book_dir.to_owned());
                    return Ok(());
                }
            }
        }

        // This is intentionally written slightly cumbersome to make you update this
        let MdBookConfig { path, theme } = this;
        let cant_find_files = path.is_none();
        let has_user_config = *theme != MdBookConfig::default().theme;
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
