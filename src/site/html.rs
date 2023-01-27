use crate::config::analytics::{get_analytics, Analytics};
use crate::config::artifacts::{self, create_artifacts_tabs};
use crate::config::{theme, Config};
use crate::errors::*;
use crate::site::css;
use crate::site::head;
use crate::site::header;
use axohtml::elements::div;
use axohtml::{dom::DOMTree, html, text, unsafe_text};

pub fn build_common_html(config: &Config, content: Box<div<String>>) -> Result<String> {
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
    let os_script = match config.artifacts {
        None => None,
        Some(_) => Some(artifacts::get_os_script(config)?),
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

    let additional_css = css::fetch_additional_css(config)?;
    let fringe_css = css::fetch_fringe_css(config)?;

    let doc: DOMTree<String> = html!(
    <html lang="en" id="oranda" class=theme>
        <head>
            <title>{ text!(&config.name) }</title>
            {homepage}
            {favicon}
            {meta_tags}
            // {fringe_css}
            {additional_css}
            <link rel="stylesheet" href="http://localhost:1234/axo-oranda.css"></link>
        </head>
        <body>
        <div class="container">
            {banner}
            <main>{header}
            {content}</main>
        </div>
            {analytics}
            {google_script}
            {os_script}
        </body>
    </html>
    );

    Ok(doc.to_string())
}

pub fn build(config: &Config, content: String) -> Result<String> {
    let artifacts_tabs = create_artifacts_tabs(config)?;

    let home_content = html!(<div>{artifacts_tabs}{ unsafe_text!(content) }
    </div>);

    let doc = build_common_html(config, home_content)?;
    Ok(doc)
}
