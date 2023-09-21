use std::path::Path;

use crate::config::Config;
use crate::errors::*;
use crate::site::markdown::{self, SyntaxTheme};

use crate::paths::determine_path;
use crate::site::templates::Templates;
use axoasset::SourceFile;
use camino::Utf8PathBuf;
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
        context: &T,
    ) -> Result<Self> {
        let contents =
            templates.render_to_string(template_name, Value::from_serializable(context))?;
        Ok(Self {
            contents,
            filename: filename.to_string(),
        })
    }

    /// Creates a new page by rendering a Markdown file into the "markdown page" template. Automatically
    /// determines the output path based on the path to the input Markdown file, diffing it with the
    /// basepath of the project.
    pub fn new_from_markdown(
        path: &str,
        templates: &Templates,
        config: &Config,
        fail_fast: bool,
    ) -> Result<Self> {
        let body = Self::load_and_render_contents(path, &config.styles.syntax_theme)?;
        let contents = if let Some(body) = body {
            templates.render_to_string("markdown_page.html", context!(body))?
        } else {
            if fail_fast {
                return Err(OrandaError::PathDoesNotExist {
                    path: path.to_string(),
                });
            }
            templates.render_to_string("markdown_page.html", context!())?
        };
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
    ///
    /// If the provided path doesn't exist, it _will_ keep rendering the other content, and output
    /// a warning to the console.
    pub fn new_from_both<T: Serialize>(
        path: &str,
        filename: &str,
        templates: &Templates,
        template_name: &str,
        context: T,
        config: &Config,
    ) -> Result<Self> {
        let body = Self::load_and_render_contents(path, &config.styles.syntax_theme)?;
        if body.is_none() {
            tracing::warn!("{} could not be found on disk!", path);
        }
        let template = templates.get(template_name)?;
        let context =
            context!(layout => templates.layout, page => context, markdown_content => body);
        let contents = template.render(context)?;
        Ok(Self {
            contents,
            filename: filename.to_string(),
        })
    }

    fn load_and_render_contents(
        source: &str,
        syntax_theme: &SyntaxTheme,
    ) -> Result<Option<String>> {
        let src_path = Utf8PathBuf::from_path_buf(std::env::current_dir()?)
            .expect("Current directory is not UTF-8");
        let path = determine_path(src_path, &None::<Utf8PathBuf>, source)?;
        if let Some(path) = path {
            let source = SourceFile::load_local(path)?;
            let contents = source.contents();
            Ok(Some(markdown::to_html(contents, syntax_theme)?))
        } else {
            Ok(None)
        }
    }

    pub fn filename(source: &str) -> String {
        let file_stem = Path::new(source).file_stem().expect("source file exists");
        format!("{}.html", file_stem.to_string_lossy())
    }
}
