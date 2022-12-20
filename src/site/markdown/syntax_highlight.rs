use crate::errors::*;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

// ["Agila Classic Oceanic Next", "Agila Cobalt", "Agila Light Solarized", "Agila Monokai Extended", "Agila Neon Monocyanide", "Agila Oceanic Next", "Agila Origin Oceanic Next", "Base16 Eighties Dark", "Base16 Mocha Dark", "Base16 Ocean Dark", "Base16 Ocean Light", "Darkmatter", "Dracula", "GitHub Light", "Material-Theme", "Material-Theme-Darker", "Material-Theme-Lighter", "Material-Theme-Palenight", "Night Owl", "One Dark"]

pub fn syntax_highlight(lang: Option<&str>, code: &str) -> Result<String> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_from_folder("src/site/markdown/syntax_themes").unwrap();
    let language = match lang {
        None | Some("") => "rs",
        Some(l) => l,
    };

    let syntax = ps.find_syntax_by_extension(language);

    match syntax {
        None => Err(OrandaError::Other(
            "Please add the language to your code snippets".to_owned(),
        )),
        Some(s) => Ok(highlighted_html_for_string(
            code,
            &ps,
            s,
            &ts.themes["Material-Theme-Palenight"],
        )?),
    }
}

#[test]
fn creates_syntax() {
    assert!(syntax_highlight(Some("js"), "console.log(5)")
        .unwrap()
        .contains("<span style=\"color:#c0c5ce;\">console.</span>"));
}
