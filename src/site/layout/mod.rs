use axohtml::{html, text};

use crate::config::{theme, Config};
use crate::errors::*;

pub mod css;
mod footer;
mod head;
mod header;
pub mod javascript;
use javascript::Analytics;

#[derive(Debug)]
pub struct Layout {
    template: String,
}

const DOCTYPE: &str = r#"<!doctype html>"#;
const BODY_PLACEHOLDER: &str = "{{{BODY}}}";
const OS_SCRIPT_PLACEHOLDER: &str = "{{{OS_SCRIPT}}}";

impl Layout {
    pub fn render(&self, body: String, os_script: Option<String>) -> String {
        self.template
            .replace(BODY_PLACEHOLDER, &body)
            .replace(OS_SCRIPT_PLACEHOLDER, &os_script.unwrap_or(String::new()))
    }

    pub fn new(config: &Config) -> Result<Self> {
        let theme = theme::css_class(&config.styles.theme);
        let name = &config.name;
        let header = match config.no_header {
            true => None,
            false => Some(header::create(config)?),
        };
        let homepage = config.homepage.as_ref().map(|homepage| {
            html!(
              <meta property="og:url" content=homepage/>
            )
        });
        let banner = header::repo_banner(config);
        let meta_tags = head::create_meta_tags(config);
        let favicon = if let Some(favicon) = config.favicon.clone() {
            Some(head::get_favicon(
                favicon,
                config.dist_dir.clone(),
                &config.path_prefix,
            )?)
        } else {
            None
        };
        let footer = footer::create_footer(config);

        let additional_css = if !config.styles.additional_css.is_empty() {
            Some(css::build_additional(&config.path_prefix))
        } else {
            None
        };
        let oranda_css = css::build_oranda(&config.dist_dir, &config.path_prefix)?;
        let analytics = Analytics::new(config)?;
        let template_html: String = html!(
        <html lang="en" id="oranda" class=theme>
            <head>
                <title>{ text!(name) }</title>
                {homepage}
                {favicon}
                {meta_tags}
                {oranda_css}
                {additional_css}
            </head>
            <body>
            <div class="container">
                {banner}
                <main>
                    {header}
                    <div>{text!(BODY_PLACEHOLDER)}</div>
                </main>
                {footer}
            </div>
                {analytics.snippet}
                {analytics.google_script}
                <div>{text!(OS_SCRIPT_PLACEHOLDER)}</div>
            </body>
        </html>
        )
        .to_string();

        let template = format!("{DOCTYPE}{template_html}");

        Ok(Layout { template })
    }
}
