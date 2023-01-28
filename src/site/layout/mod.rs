mod footer;
mod head;
mod header;

use crate::config::analytics::{self, Analytics};
use crate::config::theme;
use crate::config::Config;
use crate::errors::*;
use crate::site::artifacts;

use axohtml::elements;
use axohtml::{html, text};

pub struct Layout {
    head: head::Head,
    theme: String,
    analytics: Option<Box<elements::script<String>>>,
    google_script: Option<Box<elements::script<String>>>,
    header: Option<Box<elements::header<String>>>,
    banner: Option<Box<elements::div<String>>>,
    footer: Box<elements::footer<String>>,
    os_script: Option<Box<elements::script<String>>>,
}

pub fn build(config: &Config, content: Box<elements::div<String>>) -> Result<String> {
    let layout = Layout {
        head: head::Head::build(config)?,
        theme: theme::css_class(&config.theme).to_string(),
        analytics: analytics::build(config),
        google_script: match &config.analytics {
            Some(Analytics::Google(g)) => Some(g.get_script()),
            _ => None,
        },
        header: match config.no_header {
            true => None,
            false => Some(header::build(config)?),
        },
        os_script: match config.artifacts {
            None => None,
            Some(_) => Some(artifacts::get_os_script(config)?),
        },
        banner: header::repo_banner(config),
        footer: footer::build(config),
    };
    Ok(layout.html(content))
}

impl Layout {
    fn html(&self, content: Box<elements::div<String>>) -> String {
        html!(
        <html lang="en" id="oranda" class=self.theme>
            <head>
                <title>{ text!(&self.head.name) }</title>
                {self.head.favicon}
                {self.head.metatags}
                {self.head.fringe_css}
                {self.head.additional_css}
            </head>
            <body>
            <div class="container">
                {self.banner}
                <main>
                    {self.header}
                    {content}
                </main>
                {self.footer}
            </div>
                {self.analytics}
                {self.google_script}
                {self.os_script}
            </body>
        </html>
        )
        .to_string()
    }
}
