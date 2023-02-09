use std::path::Path;

use crate::config::Config;
use crate::errors::*;
use crate::site::artifacts;
use crate::site::layout;
use crate::site::markdown::{self, SyntaxTheme};

use axohtml::elements::div;
use axohtml::{html, unsafe_text};

#[derive(Debug)]
pub struct Page {
    pub contents: String,
    pub filename: String,
    pub is_index: bool,
}

impl Page {
    pub fn new_from_file(config: &Config, source: &str) -> Result<Self> {
        let is_index = source == config.readme_path;
        Ok(Page {
            contents: Self::load_and_render_contents(source, &config.syntax_theme)?,
            filename: Self::filename(source, is_index),
            is_index,
        })
    }

    pub fn new_from_contents(contents: String, filename: &str) -> Self {
        Page {
            contents,
            filename: filename.to_string(),
            is_index: false,
        }
    }

    fn load_and_render_contents(source: &str, syntax_theme: &SyntaxTheme) -> Result<String> {
        let contents = axoasset::local::LocalAsset::load_string(source)?;
        markdown::to_html(contents, syntax_theme)
    }

    pub fn build(self, config: &Config) -> Result<String> {
        let page_contents = if self.is_index {
            let artifacts_header = artifacts::build_header(config)?;
            html!(<div>{artifacts_header}{unsafe_text!(self.contents)}</div>).to_string()
        } else {
            let html: Box<div<String>> = html!(<div>{unsafe_text!(self.contents)}</div>);
            html.to_string()
        };
        layout::build(config, page_contents)
    }

    pub fn filename(source: &str, is_index: bool) -> String {
        let file_name = if is_index {
            "index.html".to_string()
        } else {
            let file_stem = Path::new(source).file_stem().expect("source file exists");
            format!("{}.html", file_stem.to_string_lossy())
        };

        file_name
    }
}
