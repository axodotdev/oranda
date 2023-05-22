use axoasset::{LocalAsset, SourceFile};
use camino::{Utf8Path, Utf8PathBuf};
use mdbook::MDBook;

use crate::config::MdBookConfig;
use crate::errors::*;
use crate::message::{Message, MessageType};
use crate::site::Site;

use toml_edit::value;

/// Name of the config for mdbook
const MDBOOK_TOML: &str = "book.toml";

// Files we're importing
const THEME_GENERAL_CSS_PATH: &str = "css/general.css";
const THEME_GENERAL_CSS: &str = include_str!("../../oranda-css/mdbook-theme/css/general.css");
const THEME_VARIABLES_CSS_PATH: &str = "css/variables.css";
const THEME_VARIABLES_CSS: &str = include_str!("../../oranda-css/mdbook-theme/css/variables.css");
const THEME_FONTS_CSS_PATH: &str = "fonts/fonts.css";
const THEME_FONTS_CSS: &str = include_str!("../../oranda-css/mdbook-theme/fonts/fonts.css");
const THEME_BOOK_JS_PATH: &str = "book.js";
const THEME_BOOK_JS: &str = include_str!("../../oranda-css/mdbook-theme/book.js");
const THEME_INDEX_HBS_PATH: &str = "index.hbs";
const THEME_INDEX_HBS: &str = include_str!("../../oranda-css/mdbook-theme/index.hbs");

/// Build and write mdbook to the dist dir
pub fn build_mdbook(dist: &Utf8Path, book_cfg: &MdBookConfig) -> Result<()> {
    Message::new(MessageType::Info, "Building mdbook...").print();
    tracing::info!("Building mdbook...");

    // Read mdbook's config to find the right dirs
    let book_dir = Utf8PathBuf::from(&book_cfg.path);

    if book_cfg.theme.unwrap_or(true) {
        let md = load_mdbook(&book_dir)?;
        let theme_dir =
            Utf8PathBuf::from_path_buf(md.theme_dir()).expect("mdbook theme path wasn't utf8");
        init_theme(&book_dir, &theme_dir)?;
    }

    let md = load_mdbook(&book_dir)?;

    let build_dir =
        Utf8PathBuf::from_path_buf(md.build_dir_for("html")).expect("mdbook path wasn't utf8");

    // Build the mdbook
    md.build().map_err(|e| OrandaError::MdBookBuild {
        path: book_dir.to_string(),
        inner: e,
    })?;

    // Copy the contents to "public/book/"
    // FIXME: make this something they can set in the MdBookConfig
    let book_dist = dist.join("book");
    Site::copy_static(&book_dist, build_dir.as_str())?;

    Ok(())
}

fn load_mdbook(book_dir: &Utf8Path) -> Result<MDBook> {
    let md = MDBook::load(book_dir).map_err(|e| OrandaError::MdBookLoad {
        path: book_dir.to_string(),
        inner: e,
    })?;

    Ok(md)
}

fn init_theme(book_dir: &Utf8Path, theme_dir: &Utf8Path) -> Result<()> {
    Message::new(MessageType::Info, "Adding oranda mdbook theme...").print();
    tracing::info!("Adding oranda mdbook theme...");

    init_theme_files(theme_dir)?;
    add_theme_to_book_toml(book_dir)?;

    Ok(())
}

fn init_theme_files(theme_dir: &Utf8Path) -> Result<()> {
    if theme_dir.exists() {
        LocalAsset::remove_dir_all(theme_dir.as_str())?;
    }

    let files = vec![
        (THEME_GENERAL_CSS_PATH, THEME_GENERAL_CSS),
        (THEME_VARIABLES_CSS_PATH, THEME_VARIABLES_CSS),
        (THEME_FONTS_CSS_PATH, THEME_FONTS_CSS),
        (THEME_BOOK_JS_PATH, THEME_BOOK_JS),
        (THEME_INDEX_HBS_PATH, THEME_INDEX_HBS),
    ];

    for (subpath, contents) in files {
        let path = theme_dir.join(subpath);
        LocalAsset::create_dir_all(theme_dir.as_str())?;
        LocalAsset::write_new_all(contents, path)?;
    }

    Ok(())
}

fn add_theme_to_book_toml(book_dir: &Utf8Path) -> Result<()> {
    let book_toml_path = book_dir.join(MDBOOK_TOML);
    let book_toml_src = SourceFile::load_local(&*book_toml_path)?;
    let mut book_toml = deserialize_toml_edit(&book_toml_src)?;

    let html_table = &mut book_toml["output"]["html"];
    html_table["default-theme"] = value("axo");
    html_table["preferred-dark-theme"] = value("axo");

    let new_toml_contents = book_toml.to_string();
    LocalAsset::write_new(&new_toml_contents, book_toml_path)?;
    Ok(())
}

fn deserialize_toml_edit(src: &SourceFile) -> Result<toml_edit::Document> {
    let toml = src
        .contents()
        .parse::<toml_edit::Document>()
        .map_err(|details| {
            let span = details.span().map(|span| span.into());
            OrandaError::TomlEdit {
                source: src.clone(),
                span,
                details,
            }
        })?;
    Ok(toml)
}
