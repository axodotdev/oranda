pub mod syntax_themes;

use crate::errors::*;
use crate::site::markdown::syntax_highlight::syntax_themes::SyntaxTheme;
use syntect::highlighting::ThemeSet;
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

pub fn syntax_highlight(
    lang: Option<&str>,
    code: &str,
    syntax_theme: &SyntaxTheme,
) -> Result<String> {
    let ps = SyntaxSet::load_defaults_newlines();
    let theme_set =
        ThemeSet::load_from_folder("src/site/markdown/syntax_highlight/syntax_themes").unwrap();
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
