pub mod utils;

use std::collections::HashMap;

use utils::syntax_highlight::syntax_highlight;

use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};

use crate::utils::make_footer::make_footer;
use crate::utils::make_head::make_head;
use grass::{Options, OutputStyle};

fn initialize_comrak_options() -> ComrakOptions {
    let mut options = ComrakOptions::default();

    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.extension.description_lists = true;

    options
}

pub struct Site {
    pub html: String,
    pub css: String,
}

pub fn create_site(md: &str) -> Site {
    let options = initialize_comrak_options();
    let mut plugins = ComrakPlugins::default();

    pub struct MockAdapter {}
    impl SyntaxHighlighterAdapter for MockAdapter {
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

    let adapter = MockAdapter {};
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let head = make_head();
    let footer = make_footer();
    let css_options = Options::default();

    let css = grass::from_path(
        "src/css/style.scss",
        &css_options.style(OutputStyle::Compressed),
    )
    .unwrap_or("There was a problem parsing the CSS".to_string());

    let body = markdown_to_html_with_plugins(md, &options, &plugins);
    let html = format!("{}{}{}", head, body, footer);

    Site { html, css }
}
