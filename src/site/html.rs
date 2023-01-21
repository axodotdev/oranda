use crate::config::analytics::{get_analytics, Analytics};
use crate::config::{theme, Config};
use crate::errors::*;
use crate::site::header;
use axohtml::elements::div;

use axohtml::{dom::DOMTree, html, text, unsafe_text};

use super::head::create_head;

pub fn build(config: &Config, content: String) -> Result<String> {
    let theme = theme::css_class(&config.theme);
    let analytics = get_analytics(config);
    let google_script = match &config.analytics {
        Some(Analytics::Google(g)) => Some(g.get_script()),
        _ => None,
    };
    let header = match config.no_header {
        true => None,
        false => Some(header::create(config)?),
    };

    let head = create_head(config)?;
    let banner = repo_banner(config);

    let doc: DOMTree<String> = html!(
    <html lang="en" id="oranda" class=theme>
        {head}
        <body>
        <div class="container">
            {banner}
            <main>{header}{ unsafe_text!(content) }</main>
        </div>
            {analytics}
            {google_script}
        </body>
    </html>
    );
    Ok(doc.to_string())
}

fn repo_banner(config: &Config) -> Option<Box<div<String>>> {
    config.repository.as_ref().map(|repository| {
        html!(
        <div class="repo_banner">
            <a href=repository>
                <div class="icon" aria-hidden="true"/>
                {text!("Check out our GitHub")}
            </a>
        </div>
        )
    })
}
