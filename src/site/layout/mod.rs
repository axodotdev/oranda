mod css;
mod footer;
mod head;
mod header;
mod javascript;

use crate::config::{analytics, theme, Config};
use crate::errors::*;
use axohtml::dom::DOMTree;
use axohtml::elements::div;
use axohtml::{html, text};

pub fn build(config: &Config, content: Box<div<String>>, is_index: bool) -> Result<String> {
    let theme = theme::css_class(&config.theme);
    let analytics = analytics::get_analytics(config);
    let google_script = match &config.analytics {
        Some(analytics::Analytics::Google(g)) => Some(g.get_script()),
        _ => None,
    };
    let header = match config.no_header {
        true => None,
        false => Some(header::create(config)?),
    };
    let os_script = match config.artifacts {
        None => None,
        Some(_) => {
            if is_index {
                Some(javascript::get_os_script(&config.dist_dir)?)
            } else {
                None
            }
        }
    };
    let homepage = config.homepage.as_ref().map(|homepage| {
        html!(
          <meta property="og:url" content=homepage/>
        )
    });
    let banner = header::repo_banner(config);
    let meta_tags = head::create_meta_tags(config);
    let favicon = if let Some(favicon) = config.favicon.clone() {
        Some(head::get_favicon(favicon, config.dist_dir.clone())?)
    } else {
        None
    };
    let footer = footer::create_footer(config);

    let additional_css = css::fetch_additional(config)?;
    let fringe_css = css::fetch_fringe(config)?;

    let doc: DOMTree<String> = html!(
    <html lang="en" id="oranda" class=theme>
        <head>
            <title>{ text!(&config.name) }</title>
            {homepage}
            {favicon}
            {meta_tags}
            {fringe_css}
            {additional_css}
        </head>
        <body>
        <div class="container">
            {banner}
            <main>
                {header}
                {content}
            </main>
            {footer}
        </div>
            {analytics}
            {google_script}
            {os_script}
        </body>
    </html>
    );

    Ok(doc.to_string())
}
