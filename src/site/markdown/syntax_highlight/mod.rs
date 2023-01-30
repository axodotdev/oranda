pub mod syntax_themes;

use std::collections::BTreeMap;

use crate::errors::*;
use crate::site::markdown::syntax_highlight::syntax_themes::SyntaxTheme;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::{SyntaxReference, SyntaxSet};

// The reason for this function is that find_syntax_by_extension will work when your
// snippet uses rs but not when it uses rust as the language.
// The other one works backwards so trash code it is

fn find_syntax<'a>(ps: &'a SyntaxSet, language: &'a str) -> Result<&'a SyntaxReference> {
    let syntax_extension = ps.find_syntax_by_extension(language);
    let syntax_name = ps.find_syntax_by_token(language);

    if let Some(syntax_extension) = syntax_extension {
        return Ok(syntax_extension);
    }

    if let Some(syntax_name) = syntax_name {
        return Ok(syntax_name);
    }

    Err(OrandaError::Other(
        "Please add the language to your code snippets".to_owned(),
    ))
}

const THEMES: &[(&str, &str)] = &[
    (
        "AgilaClassicOceanicNext",
        include_str!("syntax_themes/AgilaClassicOceanicNext.tmTheme"),
    ),
    (
        "AgilaCobalt",
        include_str!("syntax_themes/AgilaCobalt.tmTheme"),
    ),
    (
        "AgilaLightSolarized",
        include_str!("syntax_themes/AgilaLightSolarized.tmTheme"),
    ),
    (
        "AgilaMonokaiExtended",
        include_str!("syntax_themes/AgilaMonokaiExtended.tmTheme"),
    ),
    (
        "AgilaNeonMonocyanide",
        include_str!("syntax_themes/AgilaNeonMonocyanide.tmTheme"),
    ),
    (
        "AgilaOceanicNext",
        include_str!("syntax_themes/AgilaOceanicNext.tmTheme"),
    ),
    (
        "AgilaOriginOceanicNext",
        include_str!("syntax_themes/AgilaOriginOceanicNext.tmTheme"),
    ),
    (
        "Base16EightiesDark",
        include_str!("syntax_themes/Base16EightiesDark.tmTheme"),
    ),
    (
        "Base16MochaDark",
        include_str!("syntax_themes/Base16MochaDark.tmTheme"),
    ),
    (
        "Base16OceanDark",
        include_str!("syntax_themes/Base16OceanDark.tmTheme"),
    ),
    (
        "Base16OceanLight",
        include_str!("syntax_themes/Base16OceanLight.tmTheme"),
    ),
    (
        "Darkmatter",
        include_str!("syntax_themes/Darkmatter.tmTheme"),
    ),
    ("Dracula", include_str!("syntax_themes/Dracula.tmTheme")),
    (
        "GitHubLight",
        include_str!("syntax_themes/GitHubLight.tmTheme"),
    ),
    (
        "MaterialTheme",
        include_str!("syntax_themes/MaterialTheme.tmTheme"),
    ),
    (
        "MaterialThemeDarker",
        include_str!("syntax_themes/MaterialThemeDarker.tmTheme"),
    ),
    (
        "MaterialThemeLighter",
        include_str!("syntax_themes/MaterialThemeLighter.tmTheme"),
    ),
    (
        "MaterialThemePalenight",
        include_str!("syntax_themes/MaterialThemePalenight.tmTheme"),
    ),
    ("NightOwl", include_str!("syntax_themes/NightOwl.tmTheme")),
    ("OneDark", include_str!("syntax_themes/OneDark.tmTheme")),
];

pub fn syntax_highlight(
    lang: Option<&str>,
    code: &str,
    syntax_theme: &SyntaxTheme,
) -> Result<String> {
    let ps = SyntaxSet::load_defaults_newlines();
    let themes = THEMES
        .iter()
        .map(|(name, body)| {
            use std::io::Cursor;
            let mut buff = Cursor::new(body);
            let theme = ThemeSet::load_from_reader(&mut buff).unwrap();
            Ok((name.to_string(), theme))
        })
        .collect::<Result<BTreeMap<String, Theme>>>()?;
    let theme_set = ThemeSet { themes };
    let language = match lang {
        None | Some("") => "rs",
        Some(l) => l,
    };
    let syntax = find_syntax(&ps, language)?;

    Ok(highlighted_html_for_string(
        code,
        &ps,
        syntax,
        &theme_set.themes[&syntax_theme.as_str()],
    )?)
}

#[test]
fn creates_syntax() {
    let highlight = syntax_highlight(
        Some("js"),
        "console.log(5)",
        &SyntaxTheme::AgilaClassicOceanicNext,
    )
    .unwrap();

    assert!(highlight.contains("<span style=\"color:#fac863;\">console</span>"));
}
