use crate::config::types::syntax::SyntaxThemes;
use crate::config::Config;
use crate::errors::*;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::{SyntaxReference, SyntaxSet};

fn find_syntax<'a>(ps: &'a SyntaxSet, language: &'a str) -> Result<&'a SyntaxReference> {
    let syntax_extension = ps.find_syntax_by_extension(language);
    let syntax_name = ps.find_syntax_by_token(language);

    if syntax_extension.is_some() {
        return Ok(syntax_extension.unwrap());
    }

    if syntax_name.is_some() {
        return Ok(syntax_name.unwrap());
    }

    Err(OrandaError::Other(
        "Please add the language to your code snippets".to_owned(),
    ))
}

pub fn syntax_highlight(lang: Option<&str>, code: &str) -> Result<String> {
    let config = Config::build()?;
    let ps = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_from_folder("src/site/markdown/syntax_themes").unwrap();
    let language = match lang {
        None | Some("") => "rs",
        Some(l) => l,
    };
    let syntax = find_syntax(&ps, language)?;

    Ok(highlighted_html_for_string(
        code,
        &ps,
        syntax,
        &theme_set.themes[SyntaxThemes::as_str(&config.syntax_theme)],
    )?)
}

#[test]
fn creates_syntax() {
    assert!(syntax_highlight(Some("js"), "console.log(5)")
        .unwrap()
        .contains("<span style=\"color:#addb67;\">console</span>"));
}
