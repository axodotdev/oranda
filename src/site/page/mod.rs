use std::path::Path;

use crate::config::Config;
use crate::errors::*;
use crate::site::markdown::{self, SyntaxTheme};

use crate::site::templates::Templates;
use axoasset::SourceFile;
use minijinja::context;
use minijinja::value::Value;
use serde::Serialize;

pub mod source;

#[derive(Debug)]
pub struct Page {
    pub contents: String,
    pub filename: String,
}

impl Page {
    /// Creates a new page by rendering a template, using the provided template name and template context,
    /// and using the filename parameter as the output file name.
    pub fn new_from_template<T: Serialize>(
        filename: &str,
        templates: &Templates,
        template_name: &str,
        context: T,
    ) -> Result<Self> {
        let contents =
            templates.render_to_string(template_name, Value::from_serializable(&context))?;
        Ok(Self {
            contents,
            filename: filename.to_string(),
        })
    }

    /// Creates a new page by rendering a Markdown file into the "markdown page" template. Automatically
    /// determines the output path based on the path to the input Markdown file, diffing it with the
    /// basepath of the project.
    pub fn new_from_markdown(path: &str, templates: &Templates, config: &Config) -> Result<Self> {
        let body = Self::load_and_render_contents(path, &config.styles.syntax_theme)?;
        let contents = templates.render_to_string("markdown_page.html", context!(body))?;
        // Try diffing with the execution directory in case the user has provided an absolute-ish
        // path, in order to obtain the relative-to-dir path segment
        let relpath = if let Some(path) = pathdiff::diff_paths(path, std::env::current_dir()?) {
            path
        } else {
            path.into()
        };
        Ok(Self {
            contents,
            filename: relpath.display().to_string(),
        })
    }

    /// Combines both above functions by rendering a Markdown file into an arbitrary template. The markdown
    /// content itself will be available under the "markdown_content" key in the template itself.
    pub fn new_from_both<T: Serialize>(
        path: &str,
        filename: &str,
        templates: &Templates,
        template_name: &str,
        context: T,
        config: &Config,
    ) -> Result<Self> {
        let body = Self::load_and_render_contents(path, &config.styles.syntax_theme)?;
        let template = templates.get(template_name)?;
        let context =
            context!(layout => templates.layout, page => context, markdown_content => body);
        let contents = template.render(context)?;
        Ok(Self {
            contents,
            filename: filename.to_string(),
        })
    }

    fn load_and_render_contents(source: &str, syntax_theme: &SyntaxTheme) -> Result<String> {
        let source = SourceFile::load_local(source)?;
        let contents = source.contents();
        markdown::to_html(contents, syntax_theme)
    }

    pub fn filename(source: &str) -> String {
        let file_stem = Path::new(source).file_stem().expect("source file exists");
        format!("{}.html", file_stem.to_string_lossy())
    }
}
