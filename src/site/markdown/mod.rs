pub mod syntax_highlight;

use crate::errors::*;
use crate::site::markdown::syntax_highlight::syntax_highlight;
use ammonia::clean;
use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::{self, ComrakOptions, ComrakPlugins};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct Adapters {}
impl SyntaxHighlighterAdapter for Adapters {
    fn highlight(&self, lang: Option<&str>, code: &str) -> String {
        let highlighted_code = syntax_highlight(lang, code);

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

fn load(readme_path: &Path) -> Result<String> {
    if readme_path.exists() {
        let readme = fs::read_to_string(readme_path)?;
        let safe_html = clean(&*readme);
        Ok(safe_html)
    } else {
        Err(OrandaError::FileNotFound {
            filedesc: String::from("README"),
            path: readme_path.display().to_string(),
        })
    }
}

pub fn body(readme_path: &Path) -> Result<String> {
    let readme = load(readme_path)?;
    let options = initialize_comrak_options();

    let mut plugins = ComrakPlugins::default();
    let adapter = Adapters {};
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    Ok(comrak::markdown_to_html_with_plugins(
        &readme, &options, &plugins,
    ))
}
