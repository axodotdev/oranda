use std::path::Path;

use crate::config::Config;
use crate::data::Context;
use crate::errors::*;
use crate::site::artifacts;
use crate::site::layout::{javascript, Layout};
use crate::site::markdown::{self, SyntaxTheme};

use axoasset::SourceFile;
use axohtml::elements::div;
use axohtml::{html, unsafe_text};

pub mod source;

#[derive(Debug)]
pub struct Page {
    pub contents: String,
    pub filename: String,
}

impl Page {
    pub fn index_with_artifacts(
        context: &Context,
        layout: &Layout,
        config: &Config,
    ) -> Result<Self> {
        let mut body = artifacts::header(context, config)?;
        let readme =
            Self::load_and_render_contents(&config.readme_path, &config.styles.syntax_theme)?;
        body.push_str(&readme);
        let os_script = javascript::build_os_script(&config.path_prefix);
        let contents = layout.render(body, Some(os_script));
        Ok(Page {
            contents,
            filename: "index.html".to_string(),
        })
    }

    pub fn index(layout: &Layout, config: &Config) -> Result<Self> {
        let body =
            Self::load_and_render_contents(&config.readme_path, &config.styles.syntax_theme)?;
        let contents = layout.render(body, None);
        Ok(Page {
            contents,
            filename: "index.html".to_string(),
        })
    }

    pub fn new_from_file(source: &str, layout: &Layout, config: &Config) -> Result<Self> {
        let body = Self::load_and_render_contents(source, &config.styles.syntax_theme)?;
        let contents = layout.render(body, None);
        Ok(Page {
            contents,
            filename: Self::filename(source),
        })
    }

    pub fn new_from_contents(
        body: String,
        filename: &str,
        layout: &Layout,
        config: &Config,
    ) -> Self {
        let os_script = javascript::build_os_script(&config.path_prefix);
        let contents = layout.render(body, Some(os_script));
        Page {
            contents,
            filename: filename.to_string(),
        }
    }

    fn load_and_render_contents(source: &str, syntax_theme: &SyntaxTheme) -> Result<String> {
        let source = SourceFile::load_local(source)?;
        let contents = source.contents();
        markdown::to_html(contents, syntax_theme).map(|html| {
            let html: Box<div<String>> = html!(
                <div class="rendered-markdown">
                    {unsafe_text!(html)}
                </div>
            );
            html.to_string()
        })
    }

    pub fn filename(source: &str) -> String {
        let file_stem = Path::new(source).file_stem().expect("source file exists");
        format!("{}.html", file_stem.to_string_lossy())
    }
}
