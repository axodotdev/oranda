use std::collections::HashMap;
use std::fs;

use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};
use grass::OutputStyle;
use serde::{Deserialize, Serialize};
use utils::create_site_files::create_site_files;

use crate::options::Options;
use crate::utils::make_footer::make_footer;
use crate::utils::make_head::make_head;
use crate::utils::syntax_highlight::syntax_highlight;
use errors::*;

pub mod errors;
pub mod options;
#[cfg(test)]
mod tests;
pub mod utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    // TODO: report useful paths/details for other tools
}

pub fn exec() -> Result<Report> {
    let opts = Options::build();
    let readme_data = fs::read_to_string(&opts.readme_path)?;
    let site = create_site(&readme_data, &opts);
    match create_site_files(opts, site) {
        Err(_) => Err(OrandaError::Other(
            "There was a problem creating your website files".to_owned(),
        )),
        Ok(_) => {
            let report = Report {};

            Ok(report)
        }
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

    options
}

pub struct Site {
    pub html: String,
    pub css: String,
}

pub fn create_site(md: &str, options: &Options) -> Site {
    let comrak_options = initialize_comrak_options();
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

    let head = make_head(options);
    let footer = make_footer();
    let css_options = grass::Options::default();

    let css = grass::from_path(
        "src/css/style.scss",
        &css_options.style(OutputStyle::Compressed),
    )
    .unwrap_or_else(|_| "There was a problem parsing the CSS".to_string());

    let body = markdown_to_html_with_plugins(md, &comrak_options, &plugins);
    let html = format!("{}{}{}", head, body, footer);

    Site { html, css }
}
