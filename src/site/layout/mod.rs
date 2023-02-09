pub mod css;
mod footer;
mod head;
mod header;
pub mod javascript;

use crate::config::{analytics, theme, Config};
use crate::errors::*;
use axohtml::{html, text, unsafe_text};

pub fn build(config: &Config, content: String) -> Result<String> {
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
        Some(_) => Some(javascript::build_os_script(&config.path_prefix)?),
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

    let additional_css = if !config.additional_css.is_empty() {
        Some(css::build_additional())
    } else {
        None
    };
    let fringe_css = css::build_fringe();

    let doc: String = html!(
    <html lang="en" id="oranda" class=theme>
        <head>
            <title>{ text!(&config.name) }</title>
            {homepage}
            {favicon}
            {meta_tags}
            // {fringe_css}
            {additional_css}
            <link href="http://localhost:42673/axo-oranda.css" rel="stylesheet" />
        </head>
        <body>
        <div class="container">
            {banner}
            <main>
                {header}
                {unsafe_text!(content)}
            </main>
            {footer}
        </div>
            {analytics}
            {google_script}
            {os_script}
        </body>
    </html>
    )
    .to_string();

    Ok(doc)
}
