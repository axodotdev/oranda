use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub fn syntax_highlight(lang: Option<&str>, code: &str) -> String {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let language = if lang.is_none() { "rs" } else { "js" };
    let syntax = ps.find_syntax_by_extension(language).unwrap();

    highlighted_html_for_string(code, &ps, syntax, &ts.themes["base16-ocean.dark"]).unwrap()
}
