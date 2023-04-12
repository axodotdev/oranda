use std::collections::HashMap;

mod syntax_highlight;
use axoasset::SourceFile;
pub use syntax_highlight::syntax_highlight;
pub use syntax_highlight::syntax_themes::SyntaxTheme;

use crate::errors::*;

use ammonia::Builder;
use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::{self, ComrakOptions, ComrakPlugins};

pub struct Adapters<'a> {
    syntax_theme: &'a SyntaxTheme,
    src: &'a SourceFile,
}
impl SyntaxHighlighterAdapter for Adapters<'_> {
    fn highlight(&self, lang: Option<&str>, code: &str) -> String {
        let highlighted_code = syntax_highlight(self.src, lang, code, self.syntax_theme);

        // requires a string to be returned
        match highlighted_code {
            Ok(code) => code,
            Err(_) => String::new(),
        }
    }

    fn build_pre_tag(&self, _attributes: &HashMap<String, String>) -> String {
        String::new()
    }

    fn build_code_tag(&self, _attributes: &HashMap<String, String>) -> String {
        String::new()
    }
}

fn initialize_comrak_options() -> ComrakOptions {
    let mut options = ComrakOptions::default();

    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.render.unsafe_ = true;

    options
}

pub fn to_html(markdown: &SourceFile, syntax_theme: &SyntaxTheme) -> Result<String> {
    let options = initialize_comrak_options();

    let mut plugins = ComrakPlugins::default();
    let adapter = Adapters {
        syntax_theme,
        src: markdown,
    };
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let unsafe_html =
        comrak::markdown_to_html_with_plugins(markdown.contents(), &options, &plugins);
    let safe_html = Builder::new()
        .add_generic_attributes(&["style", "class", "id"])
        .clean(&unsafe_html)
        .to_string();
    Ok(safe_html)
}
