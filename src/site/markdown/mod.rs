use std::collections::HashMap;
use std::io::BufWriter;

mod syntax_highlight;
pub use syntax_highlight::syntax_themes::SyntaxTheme;
pub use syntax_highlight::{dump_syntax_themes, syntax_highlight};

use crate::errors::*;

use ammonia::Builder;
use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::{self, Arena, ComrakOptions, ComrakPlugins};

pub struct Adapters<'a> {
    syntax_theme: &'a SyntaxTheme,
}
impl SyntaxHighlighterAdapter for Adapters<'_> {
    fn highlight(&self, lang: Option<&str>, code: &str) -> String {
        let highlighted_code = syntax_highlight(lang, code, self.syntax_theme);

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

pub fn to_html(
    markdown: &str,
    syntax_theme: &SyntaxTheme,
    path_prefix: &Option<String>,
) -> Result<String> {
    let options = initialize_comrak_options();

    let mut plugins = ComrakPlugins::default();
    let adapter = Adapters { syntax_theme };
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    // Build the markdown AST
    let arena = Arena::new();
    let root: &comrak::arena_tree::Node<'_, std::cell::RefCell<comrak::nodes::Ast>> =
        comrak::parse_document(&arena, markdown, &options);

    // Edit links in the markdown AST
    for node in root.descendants() {
        let mut node = node.data.borrow_mut();
        if let comrak::nodes::NodeValue::Link(link) = &mut node.value {
            if link.url.contains("./SECURITY.md") {
                link.url = crate::site::link::generate(path_prefix, "SECURITY/");
            }
        }
    }

    // Render the markdown AST to HTML
    let mut bw = BufWriter::new(Vec::new());
    comrak::format_html_with_plugins(root, &options, &mut bw, &plugins).unwrap();
    let unsafe_html = String::from_utf8(bw.into_inner().unwrap()).unwrap();

    // Sanitize the html
    let safe_html = Builder::new()
        .add_generic_attributes(&["style", "class", "id"])
        .clean(&unsafe_html)
        .to_string();
    Ok(safe_html)
}
