use std::path::Path;

pub mod markdown;

use crate::config::Config;
use crate::errors::*;
use crate::site::artifacts;
use crate::site::layout;
use crate::site::page::markdown::syntax_highlight::SyntaxTheme;

use axohtml::{html, unsafe_text};

#[derive(Debug)]
pub struct Page {
    pub filename: String,
    pub contents: String,
    pub is_index: bool,
}

impl Page {
    pub async fn new_from_source(config: &Config, source: &str) -> Result<Self> {
        let is_index = source == config.readme_path;
        if let Some(filename) = Self::filename(is_index, source) {
            Ok(Page {
                filename,
                contents: Self::render(config, source, is_index, config.syntax_theme),
                is_index,
            })
        } else {
            Err(OrandaError::Other("oops".to_string()))
        }
    }

    pub fn new_from_contents(config: &Config, contents: String, filename: String) -> Self {
        Page {
            filename,
            contents,
            is_index: filename == config.readme_path,
        }
    }

    async fn render(
        config: &Config,
        source: &str,
        is_index: bool,
        syntax_theme: SyntaxTheme,
    ) -> Result<String> {
        let markdown = axoasset::load_string(&source).await?;
        let unsafe_html = markdown::render(markdown, syntax_theme)?;
        let safe_html = Self::sanitize(&unsafe_html);
        let rendered_contents = if is_index {
            let artifacts = artifacts::build_section(config)?;
            html!(<div>{artifacts}{unsafe_text!(safe_html)}</div>)
        } else {
            html!(<div>{unsafe_text!(safe_html)}</div>)
        };
        let doc = layout::build(config, rendered_contents)?;
        Ok(doc)
    }

    fn filename(is_index: bool, source: &str) -> Option<String> {
        if is_index {
            Some("index.html".to_string())
        } else {
            Some(format!(
                "{}.html",
                Path::new(source).file_stem()?.to_string_lossy()
            ))
        }
    }

    fn sanitize(unsafe_html: &str) -> String {
        ammonia::Builder::new()
            .add_generic_attributes(&["style", "class", "id"])
            .clean(unsafe_html)
            .to_string()
    }
}
