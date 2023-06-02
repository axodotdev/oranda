pub mod syntax_themes;

use std::collections::BTreeMap;

use crate::errors::*;
use crate::message::{Message, MessageType};
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
        Ok(syntax_extension)
    } else if let Some(syntax_name) = syntax_name {
        Ok(syntax_name)
    } else {
        // if we end up here it means that we could not find the provided language
        // by name or extension. either this means that no language was provided
        // or the language is not supported. we check if the language str is empty
        // to see if there was an annotation at all, and if so, warn that it's
        // unsupported and being overridden as plain text.
        if !language.is_empty() {
            let msg = format!("Found syntax highlight language annotation `{language}` which is not currently supported. The annotated block will be shown as plaintext. Please file an issue https://github.com/axodotdev/oranda/issues/new to let us know you'd like to see it supported.");
            Message::new(MessageType::Warning, &msg).print();
            tracing::warn!("{}", &msg);
        }

        // this syntax will always be found as it's part of the default set
        // https://github.com/sublimehq/Packages
        let plain_text = ps
            .syntaxes()
            .iter()
            .find(|syntax| syntax.name == "Plain Text")
            .unwrap();

        Ok(plain_text)
    }
}

const THEMES: &[(&str, &str)] = &[("MaterialTheme", include_str!("MaterialTheme.tmTheme"))];

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
        Some("text") => "txt",
        Some("shell") => "sh",
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
