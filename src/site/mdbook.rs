use axoasset::LocalAsset;
use camino::{Utf8Path, Utf8PathBuf};
use mdbook::MDBook;

use crate::config::MdBookConfig;
use crate::errors::*;
use crate::message::{Message, MessageType};
use crate::site::Site;

use super::markdown::SyntaxTheme;

// Files we're importing
const THEME_GENERAL_CSS_PATH: &str = "css/general.css";
const THEME_GENERAL_CSS: &str = include_str!("../../oranda-css/mdbook-theme/css/general.css");
const THEME_VARIABLES_CSS_PATH: &str = "css/variables.css";
const THEME_VARIABLES_CSS: &str = include_str!("../../oranda-css/mdbook-theme/css/variables.css");
const THEME_CHROME_CSS_PATH: &str = "css/chrome.css";
const THEME_CHROME_CSS: &str = include_str!("../../oranda-css/mdbook-theme/css/chrome.css");
const THEME_FONTS_CSS_PATH: &str = "fonts/fonts.css";
const THEME_FONTS_CSS: &str = include_str!("../../oranda-css/mdbook-theme/fonts/fonts.css");
const THEME_BOOK_JS_PATH: &str = "book.js";
const THEME_BOOK_JS: &str = include_str!("../../oranda-css/mdbook-theme/book.js");
const THEME_INDEX_HBS_PATH: &str = "index.hbs";
const THEME_INDEX_HBS: &str = include_str!("../../oranda-css/mdbook-theme/index.hbs");

const THEME_AXO_HIGHLIGHT_CSS_PATH: &str = "axo-highlight.css";
const THEMES: &[(&str, &str)] = &[(
    "MaterialTheme",
    include_str!("../../oranda-css/mdbook-theme/highlight-js-themes/base16-material.css"),
)];

/// Get a proper absolute path to the mdbook's dir (the one containing book.toml)
///
/// This needs to be absolute because some mdbook renderers flip out with relative paths.
///
/// FIXME: this is broken if pwd is not the same dir as oranda.json. Our config code
/// should do this mapping for us, when it still knows where oranda.json is!
pub fn mdbook_dir(book_cfg: &MdBookConfig) -> Result<Utf8PathBuf> {
    let pwd = axoasset::LocalAsset::current_dir()?;
    Ok(pwd.join(&book_cfg.path))
}

/// Gets whether we want to add the custom oranda/axo themes to mdbooks
pub fn has_custom_theme(book_cfg: &MdBookConfig) -> bool {
    book_cfg.theme.unwrap_or(true)
}

/// Gets the dir where we should write custom theme files
pub fn custom_theme_dir(_book_cfg: &MdBookConfig, dist: &Utf8Path) -> Result<Utf8PathBuf> {
    let pwd = axoasset::LocalAsset::current_dir()?;
    Ok(pwd.join(dist).join("mdbook_theme"))
}

/// Build and write the mdbook to the dist dir
pub fn build_mdbook(
    dist: &Utf8Path,
    book_cfg: &MdBookConfig,
    syntax_theme: &SyntaxTheme,
) -> Result<()> {
    Message::new(MessageType::Info, "Building mdbook...").print();
    tracing::info!("Building mdbook...");

    // Read mdbook's config to inherit the user's setup
    let book_dir = mdbook_dir(book_cfg)?;
    let mut md = load_mdbook(&book_dir)?;

    // If custom theme is enabled, set that up
    let custom_theme = has_custom_theme(book_cfg);
    let theme_dir = custom_theme_dir(book_cfg, dist)?;
    if custom_theme {
        init_theme_dir(&theme_dir)?;
        md.config.set("output.html.default-theme", "axo").unwrap();
        md.config
            .set("output.html.preferred-dark-theme", "axo")
            .unwrap();
        md.config.set("output.html.theme", &theme_dir).unwrap();
    }

    // Build the mdbook
    let build_dir =
        Utf8PathBuf::from_path_buf(md.build_dir_for("html")).expect("mdbook path wasn't utf8");
    md.build().map_err(|e| OrandaError::MdBookBuild {
        path: book_dir.to_string(),
        details: e,
    })?;

    if custom_theme {
        // If custom theme is enabled, add the axo syntax highlighting theme to the output
        add_custom_syntax_theme_to_output(syntax_theme, &build_dir)?;
        // See docs of this function for why we delete this dir
        delete_theme_dir(&theme_dir)?;
    }

    // Copy the contents to "public/book/"
    // FIXME: make this something they can set in the MdBookConfig
    let book_dist = dist.join("book");
    Site::copy_static(&book_dist, build_dir.as_str())?;

    Ok(())
}

/// Load the mdbook config (book.toml) in a given directory
///
/// Note that you can do this as many times as you want and even edit the config
/// and everything will almost certainly work fine. This basically just does the minimal
/// amount of fs/env reads to get raw config values.
///
/// Interesting things only happen when you run `.build()`
pub fn load_mdbook(book_dir: &Utf8Path) -> Result<MDBook> {
    let md = MDBook::load(book_dir).map_err(|e| OrandaError::MdBookLoad {
        path: book_dir.to_string(),
        details: e,
    })?;

    Ok(md)
}

/// Initialize a directory with our custom theme files
///
/// Note that these files assume you will also call [`add_custom_syntax_theme_to_output`][]
/// to add axo-highlight.css to the build dir.
fn init_theme_dir(theme_dir: &Utf8Path) -> Result<()> {
    Message::new(MessageType::Info, "Adding oranda mdbook theme...").print();
    tracing::info!("Adding oranda mdbook theme...");

    // Just to be safe, clear out the theme dir in case it still exists
    delete_theme_dir(theme_dir)?;

    let files = vec![
        (THEME_GENERAL_CSS_PATH, THEME_GENERAL_CSS),
        (THEME_VARIABLES_CSS_PATH, THEME_VARIABLES_CSS),
        (THEME_CHROME_CSS_PATH, THEME_CHROME_CSS),
        (THEME_FONTS_CSS_PATH, THEME_FONTS_CSS),
        (THEME_BOOK_JS_PATH, THEME_BOOK_JS),
        (THEME_INDEX_HBS_PATH, THEME_INDEX_HBS),
    ];

    for (subpath, contents) in files {
        let path = theme_dir.join(subpath);
        LocalAsset::write_new_all(contents, path)?;
    }

    Ok(())
}

/// Delete the custom theme dir
///
/// In the current implementation this folder only needs to exist for mdbook
/// to read during its build, and is otherwise useless. So we should delete
/// it from ./public/ so that it doesn't end up in prod. It gets generated
/// in ./public/ because that's a dir we have carte-blanche to mess around in.
fn delete_theme_dir(theme_dir: &Utf8Path) -> Result<()> {
    LocalAsset::remove_dir_all(theme_dir.as_str())?;
    Ok(())
}

/// Write the syntax highlighting theme we use for oranda to the mdbook output dir
///
/// This is the best way I could find to add this file -- things like additional-css are wonky
/// with how they handle paths not in book_dir, and overriding highlight.css will mess up
/// vanilla themes that assume it works a certain way.
fn add_custom_syntax_theme_to_output(
    syntax_theme: &SyntaxTheme,
    build_dir: &Utf8Path,
) -> Result<()> {
    let theme_name = syntax_theme.as_str();
    let highlight_theme = THEMES
        .iter()
        .find_map(|(name, contents)| {
            if *name == theme_name {
                Some(*contents)
            } else {
                None
            }
        })
        .expect("failed to find highlightjs syntax theme for mdbook!?");

    LocalAsset::write_new_all(
        highlight_theme,
        build_dir.join(THEME_AXO_HIGHLIGHT_CSS_PATH),
    )?;
    Ok(())
}
