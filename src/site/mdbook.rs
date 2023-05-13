use axoasset::LocalAsset;
use camino::{Utf8Path, Utf8PathBuf};
use mdbook::MDBook;

use crate::config::theme::Theme;
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

/// variables.css needs us to substitute this with an entry from MDBOOK_THEMES
const KEY_ORANDA_VARS: &str = "/*ORANDA-THEME-VARS*/";
/// index.hbs needs us to substitute this with at least one copy of THEME_BUTTON_HTML_TEMPLATE
const KEY_ORANDA_BUTTONS: &str = "<!--ORANDA-THEME-BUTTONS-->";
/// THEME_BUTTON_HTML_TEMPLATE needs us to substitute this with CLASS_ORANDA_DARK or CLASS_ORANDA_LIGHT
///
/// (yes mdbook has some magic in it where you use a css class as the id of a button in its dropdowns)
const KEY_BUTTON_ID: &str = "{{THEME-ID}}";
/// THEME_BUTTON_HTML_TEMPLATE needs us to substitute this with a user-facing name for the theme
const KEY_BUTTON_NAME: &str = "{{THEME-NAME}}";
/// Template for the HTML for a button in the theme selector
const THEME_BUTTON_HTML_TEMPLATE: &str = r###"                            <li role="none"><button role="menuitem" class="theme" id="{{THEME-ID}}">{{THEME-NAME}}</button></li>"###;

/// the css class used for dark themes
const CLASS_ORANDA_DARK: &str = "oranda-dark";
/// the css class used for light themes
const CLASS_ORANDA_LIGHT: &str = "oranda-light";

// Mappings from AxomdbookThemes to their implementations
const THEME_IMPL_DEFAULT: &str =
    include_str!("../../oranda-css/mdbook-theme/oranda-themes/default.css");
const THEME_IMPL_AXO: &str = include_str!("../../oranda-css/mdbook-theme/oranda-themes/axo.css");
const THEME_IMPL_HACKER: &str =
    include_str!("../../oranda-css/mdbook-theme/oranda-themes/hacker.css");
const THEME_IMPL_CUPCAKE: &str =
    include_str!("../../oranda-css/mdbook-theme/oranda-themes/cupcake.css");
const MDBOOK_THEMES: &[(AxomdbookTheme, &str)] = &[
    (AxomdbookTheme::Default, THEME_IMPL_DEFAULT),
    (AxomdbookTheme::DefaultLight, THEME_IMPL_DEFAULT),
    (AxomdbookTheme::AxoDark, THEME_IMPL_AXO),
    (AxomdbookTheme::AxoLight, THEME_IMPL_AXO),
    (AxomdbookTheme::Hacker, THEME_IMPL_HACKER),
    (AxomdbookTheme::Cupcake, THEME_IMPL_CUPCAKE),
];

// Mappings from SyntaxThemes to their implementations
const THEME_AXO_HIGHLIGHT_CSS_PATH: &str = "oranda-highlight.css";
const SYNTAX_THEMES: &[(SyntaxTheme, &str)] = &[(
    SyntaxTheme::MaterialTheme,
    include_str!("../../oranda-css/mdbook-theme/highlight-js-themes/base16-material.css"),
)];

/// A theme we can inject when building mdbooks
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AxomdbookTheme {
    /// Equivalent to oranda's "dark"
    Default,
    /// Equivalent to oranda's "light"
    DefaultLight,
    /// Equivalent to oranda's "axo_dark"
    AxoDark,
    /// Equivalent to oranda's "axo_ight"
    AxoLight,
    /// Equivalent to oranda's "hacker"
    Hacker,
    /// Equivalent to oranda's "cupcake"
    Cupcake,
    /// Equivalent to oranda's "tui"
    Tui,
}

impl AxomdbookTheme {
    /// Get the equivalent mdbook theme for this oranda theme
    ///
    /// If none exists we won't override themes
    pub fn from_oranda_theme(oranda_theme: &Theme) -> Option<Self> {
        use AxomdbookTheme::*;
        match oranda_theme {
            Theme::Light => Some(DefaultLight),
            Theme::Dark => Some(Default),
            Theme::AxoDark => Some(AxoDark),
            Theme::AxoLight => Some(AxoLight),
            Theme::Hacker => Some(Hacker),
            Theme::Cupcake => Some(Cupcake),
            Theme::Tui => Some(Tui),
        }
    }

    /// Get whether this theme should be presented as a "dark mode" or "light mode"
    pub fn is_dark(&self) -> bool {
        use AxomdbookTheme::*;
        match self {
            Default => true,
            DefaultLight => false,
            AxoDark => true,
            AxoLight => false,
            Hacker => true,
            Cupcake => false,
            Tui => true,
        }
    }

    /// If this theme is two-in-one with a "dark mode" and "light mode", then this
    /// returns the other mode.
    pub fn twin_theme(&self) -> Option<AxomdbookTheme> {
        use AxomdbookTheme::*;
        match self {
            Default => Some(DefaultLight),
            DefaultLight => Some(Default),
            AxoDark => Some(AxoLight),
            AxoLight => Some(AxoDark),
            Hacker => None,
            Cupcake => None,
            Tui => None,
        }
    }

    /// Get the css class / localStorage value for this theme
    ///
    /// **KEEP IN MIND** these values are hardcoded into the
    /// css/js/hbs files for our custom theme, so we always use
    /// the same values, based only on whether the theme is light/dark
    pub fn class(&self) -> &'static str {
        if self.is_dark() {
            CLASS_ORANDA_DARK
        } else {
            CLASS_ORANDA_LIGHT
        }
    }

    pub fn name(&self) -> &'static str {
        use AxomdbookTheme::*;
        match self {
            Default => "Oranda Dark",
            DefaultLight => "Oranda Light",
            AxoDark => "Axo Dark",
            AxoLight => "Axo Light",
            Hacker => "Hacker",
            Cupcake => "Cupcake",
            Tui => "Tui",
        }
    }
}

/// Get a proper absolute path to the mdbook's dir (the one containing book.toml)
///
/// This needs to be absolute because some mdbook renderers flip out with relative paths.
///
/// FIXME: this is broken if pwd is not the same dir as oranda.json. Our config code
/// should do this mapping for us, when it still knows where oranda.json is!
pub fn mdbook_dir(book_cfg: &MdBookConfig) -> Result<Utf8PathBuf> {
    let pwd = axoasset::LocalAsset::current_dir()?;
    let book_path = book_cfg
        .path
        .as_ref()
        .expect("Had no mdbook.path, but config code didn't disable mdbook?");
    Ok(pwd.join(book_path))
}

/// Gets the custom theme to set in an mdbook
pub fn custom_theme(book_cfg: &MdBookConfig, oranda_theme: &Theme) -> Option<AxomdbookTheme> {
    if book_cfg.theme.unwrap_or(true) {
        AxomdbookTheme::from_oranda_theme(oranda_theme)
    } else {
        None
    }
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
    oranda_theme: &Theme,
    syntax_theme: &SyntaxTheme,
) -> Result<()> {
    Message::new(MessageType::Info, "Building mdbook...").print();
    tracing::info!("Building mdbook...");

    // Read mdbook's config to inherit the user's setup
    let book_dir = mdbook_dir(book_cfg)?;
    let mut md = load_mdbook(&book_dir)?;

    // If custom theme is enabled, set that up
    let custom_theme = custom_theme(book_cfg, oranda_theme);
    let theme_dir = custom_theme_dir(book_cfg, dist)?;
    if let Some(theme) = custom_theme {
        // Create all the files for our custom theme
        init_theme_dir(&theme_dir, theme)?;

        // Tell mdbook to default to our theme, forcing both the light and dark modes
        //
        // FIXME(#314): for now we force the same theme as both the "light" and "dark" version
        // to avoid clashes between the main oranda pages and the mdbook when the two
        // disagree on how "dark mode" is detected. In the future we can/should use twin_theme
        // to properly set these values.
        let dark_theme = theme;
        md.config
            .set("output.html.default-theme", theme.class())
            .unwrap();
        md.config
            .set("output.html.preferred-dark-theme", dark_theme.class())
            .unwrap();

        // Tell mdbook where to find our custom theme
        md.config.set("output.html.theme", &theme_dir).unwrap();
    }

    // Build the mdbook
    let build_dir =
        Utf8PathBuf::from_path_buf(md.build_dir_for("html")).expect("mdbook path wasn't utf8");
    md.build().map_err(|e| OrandaError::MdBookBuild {
        path: book_dir.to_string(),
        details: e,
    })?;

    if custom_theme.is_some() {
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
fn init_theme_dir(theme_dir: &Utf8Path, theme: AxomdbookTheme) -> Result<()> {
    Message::new(MessageType::Info, "Adding oranda mdbook theme...").print();
    tracing::info!("Adding oranda mdbook theme...");

    // Just to be safe, clear out the theme dir in case it still exists
    delete_theme_dir(theme_dir)?;

    // Substitute in the css vars for the selected theme
    let theme_vars = MDBOOK_THEMES
        .iter()
        .find_map(
            |(t, contents)| {
                if t == &theme {
                    Some(*contents)
                } else {
                    None
                }
            },
        )
        .expect("failed to find axomdbook theme for mdbook!?");
    let variables = THEME_VARIABLES_CSS.replace(KEY_ORANDA_VARS, theme_vars);

    // Substitute in buttons for the selected theme
    let mut buttons = String::new();
    add_theme_button(&mut buttons, theme);
    if let Some(twin) = theme.twin_theme() {
        add_theme_button(&mut buttons, twin);
    }
    let index = THEME_INDEX_HBS.replace(KEY_ORANDA_BUTTONS, &buttons);

    // Now write all the files
    let files = vec![
        (THEME_GENERAL_CSS_PATH, THEME_GENERAL_CSS),
        (THEME_VARIABLES_CSS_PATH, &variables),
        (THEME_CHROME_CSS_PATH, THEME_CHROME_CSS),
        (THEME_FONTS_CSS_PATH, THEME_FONTS_CSS),
        (THEME_BOOK_JS_PATH, THEME_BOOK_JS),
        (THEME_INDEX_HBS_PATH, &index),
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
    let highlight_theme = SYNTAX_THEMES
        .iter()
        .find_map(|(theme, contents)| {
            if theme == syntax_theme {
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

fn add_theme_button(output: &mut String, theme: AxomdbookTheme) {
    // Yes we use a class as an id, it's an mdbook thing
    let id = theme.class();
    let name = theme.name();
    let button = THEME_BUTTON_HTML_TEMPLATE
        .replace(KEY_BUTTON_ID, id)
        .replace(KEY_BUTTON_NAME, name);
    output.push_str(&button);
    output.push('\n');
}
