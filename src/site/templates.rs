//! Fundamentals for oranda's templating system.
//!
//! At the core, oranda uses a minijinja (Jinja2 template)-based system, where all templates are
//! loaded from memory at the start of runtime. Templates can then be referenced from anywhere within
//! the application, provided the `Templates` struct is properly passed around. Templates themselves
//! can also use features such as imports, inheritance, extends, and so on.

use crate::config::Config;
use crate::data::artifacts::inference::triple_to_display_name;
use crate::errors::Result;
use crate::site::layout::LayoutContext;
use crate::site::markdown::SyntaxTheme;
use crate::site::{link, markdown};
use include_dir::{include_dir, Dir};
use minijinja::value::Value;
use minijinja::{context, AutoEscape, Environment, Template};
use std::collections::HashMap;

const TEMPLATE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");

/// Main templates struct that gets passed around in the application.
pub struct Templates<'a> {
    /// Minijinja environment that contains all loaded templates
    pub env: Environment<'a>,
    /// Layout context used for every render
    pub layout: LayoutContext,
}

impl<'a> Templates<'a> {
    pub fn new(config: &Config) -> Result<Self> {
        let mut env = Environment::new();
        let mut files = HashMap::new();
        // These two `expects` should never happen in production, because all of these things are
        // are baked into the binary. If this fails at all it should presumably *always* fail, and
        // so these unwraps will only show up when someone's messing with the templates locally
        // during development and presumably wrote some malformed jinja2 markup.
        Self::load_files(&TEMPLATE_DIR, &mut files)
            .expect("failed to load jinja2 templates from binary");
        for (path, contents) in files {
            env.add_template_owned(path, contents)
                .expect("failed to add jinja2 template");
        }
        env.add_filter("generate_link", Self::generate_link);
        env.add_filter("syntax_highlight", Self::syntax_highlight);
        env.add_filter("triple_to_display_name", Self::triple_to_display_name);
        // Use opt-in autoescape
        env.set_auto_escape_callback(|_| AutoEscape::None);
        let layout = LayoutContext::new(config)?;
        Ok(Self { env, layout })
    }

    pub fn new_for_workspace_index(workspace_config: &Config) -> Result<Self> {
        let mut env = Environment::new();
        let mut files = HashMap::new();
        let dir = &TEMPLATE_DIR
            .get_dir("workspace_index")
            .expect("workspace_index directory not found");
        Self::load_files(dir, &mut files).expect("failed to load jinja2 templates from binary");
        for (path, contents) in files {
            env.add_template_owned(path, contents)
                .expect("failed to add jinja2 template");
        }
        env.add_filter("generate_link", Self::generate_link);
        let layout = LayoutContext::new_for_workspace_index(workspace_config)?;
        Ok(Self { env, layout })
    }

    pub fn get(&self, name: &str) -> Result<Template> {
        Ok(self.env.get_template(name)?)
    }

    pub fn render_to_string(&self, name: &str, context: Value) -> Result<String> {
        let context_with_layout = context!(layout => self.layout, page => context);
        let template = self.env.get_template(name)?;
        Ok(template.render(context_with_layout)?)
    }

    fn load_files(dir: &Dir, files: &mut HashMap<String, String>) -> Result<()> {
        for entry in dir.entries() {
            if let Some(file) = entry.as_file() {
                let file_path = file.path();
                // Remove the .j2 extension
                let file_name = file_path.with_extension("");
                files.insert(
                    file_name.display().to_string(),
                    file.contents_utf8()
                        .expect("non-utf8 jinja2 template")
                        .to_string(),
                );
            }
            if let Some(dir) = entry.as_dir() {
                Self::load_files(dir, files).expect("failed to load jinja2 templates from binary");
            }
        }

        Ok(())
    }

    fn generate_link(base: String, path_prefix: String) -> String {
        // Weird Jinja serialization handling
        let path_prefix = if path_prefix == "none" {
            None
        } else {
            Some(path_prefix)
        };
        link::generate(&path_prefix, &base)
    }

    fn syntax_highlight(code: String, lang: String, _syntax_theme: String) -> String {
        // TODO: Fix when we support more syntax themes.
        let syntax_theme = SyntaxTheme::MaterialTheme;
        match markdown::syntax_highlight(Some(&lang), &code, &syntax_theme) {
            Ok(res) => res,
            Err(_) => format!("<code class='inline-code'>{code}</code>"),
        }
    }

    // Turn a triple into a pretty display name, or return the original triple
    fn triple_to_display_name(target: String) -> String {
        triple_to_display_name(&target)
            .map(|t| t.to_string())
            .unwrap_or(target)
    }
}
