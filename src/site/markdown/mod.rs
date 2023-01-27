pub mod syntax_highlight;

use crate::errors::*;
use crate::site::markdown::syntax_highlight::syntax_highlight;
use ammonia::Builder;
use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::{self, ComrakOptions, ComrakPlugins};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use self::syntax_highlight::syntax_themes::SyntaxTheme;

pub struct Adapters<'a> {
    syntax_theme: &'a SyntaxTheme,
}
impl SyntaxHighlighterAdapter for Adapters<'_> {
    fn highlight(&self, lang: Option<&str>, code: &str) -> String {
        let highlighted_code = syntax_highlight(lang, code, self.syntax_theme);

        // requires a string to be returned
        match highlighted_code {
            Ok(code) => code,
            Err(_) => String::from(""),
        }
    }

    fn build_pre_tag(&self, _attributes: &HashMap<String, String>) -> String {
        String::from("")
    }

    fn build_code_tag(&self, _attributes: &HashMap<String, String>) -> String {
        String::from("")
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

fn load(file_path: &Path, is_main_readme: bool) -> Result<String> {
    if file_path.exists() {
        let file = fs::read_to_string(file_path)?;
        Ok(file)
    } else {
        let filedesc = if is_main_readme {
            String::from("README")
        } else {
            String::from("additional file")
        };
        Err(OrandaError::FileNotFound {
            filedesc,
            path: file_path.display().to_string(),
        })
    }
}

pub fn body(file_path: &Path, syntax_theme: &SyntaxTheme, is_main_readme: bool) -> Result<String> {
    let readme = load(file_path, is_main_readme)?;
    let options = initialize_comrak_options();

    let mut plugins = ComrakPlugins::default();
    let adapter = Adapters { syntax_theme };
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let unsafe_html = comrak::markdown_to_html_with_plugins(&readme, &options, &plugins);
    let safe_html = Builder::new()
        .add_generic_attributes(&["style", "class", "id"])
        .clean(&unsafe_html)
        .to_string();
    Ok(safe_html)
}
