use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};
use grass::{Options, OutputStyle};
use serde::{Deserialize, Serialize};

use crate::utils::make_footer::make_footer;
use crate::utils::make_head::make_head;
use crate::utils::syntax_highlight::syntax_highlight;
use errors::*;

pub mod errors;
#[cfg(test)]
mod tests;
pub mod utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub cats_are_cute: bool,
}

pub fn some_op() -> Result<Report> {
    let mut file = File::open("test.md")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let site = create_site(&data);

    std::fs::create_dir_all("public")?;

    let mut html_file = File::create("public/index.html")?;
    html_file.write_all(site.html.as_bytes())?;

    let mut css_file = File::create("public/styles.css")?;
    css_file.write_all(site.css.as_bytes())?;

    let report = Report {
        cats_are_cute: true,
    };

    Ok(report)
}

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
