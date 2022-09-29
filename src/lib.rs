use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};
use grass::OutputStyle;
use serde::{Deserialize, Serialize};
use utils::options::{Options, OptionsFilled};

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
    // TODO: report useful paths/details for other tools
}

pub fn create_options(options: Options) -> OptionsFilled {
    let path = match options.file {
        Some(f) => f,
        None => "Readme.md".to_string(),
    };

    let dist = match options.dist {
        Some(f) => f,
        None => "public".to_string(),
    };

    OptionsFilled { file: path, dist }
}

pub fn do_oranda(options: Options) -> Result<Report> {
    let parsed_options = create_options(options);
    let mut file = File::open(parsed_options.file)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let site = create_site(&data);

    let dist = parsed_options.dist;
    std::fs::create_dir_all(&dist)?;
    let html_path = format!("{}/index.html", &dist);
    let css_path = format!("{}/styles.css", &dist);
    let mut html_file = File::create(html_path)?;
    html_file.write_all(site.html.as_bytes())?;

    let mut css_file = File::create(css_path)?;
    css_file.write_all(site.css.as_bytes())?;

    let report = Report {};

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
    let css_options = grass::Options::default();

    let css = grass::from_path(
        "src/css/style.scss",
        &css_options.style(OutputStyle::Compressed),
    )
    .unwrap_or_else(|_| "There was a problem parsing the CSS".to_string());

    let body = markdown_to_html_with_plugins(md, &options, &plugins);
    let html = format!("{}{}{}", head, body, footer);

    Site { html, css }
}
