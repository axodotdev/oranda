use std::path::Path;

use axohtml::{dom::DOMTree, html, text, unsafe_text};

use crate::config::{theme, Config};
use axohtml::elements::{div, header, li};

fn create_header(config: &Config) -> Box<header<String>> {
    let nav = match config.additional_pages.as_ref() {
        Some(pages) => {
            let mut html: Vec<Box<li<String>>> = vec![];
            html.extend(html!(<li><a href="/">"Home"</a></li>));
            for page in pages.into_iter() {
                let file_name = Path::new(page).file_stem().unwrap().to_string_lossy();
                let path = format!("/{}", file_name);
                html.extend(html!(<li><a href=path>{text!(file_name)}</a></li>));
            }
            Some(html!(
            <nav>
                <ul>
                     {html}
                </ul>
            </nav>
            ))
        }
        None => None,
    };

    html!(<header>{nav}<h1>{text!(&config.name)}</h1></header>)
}

pub fn build(config: &Config, content: String) -> String {
    let theme = theme::css_class(&config.theme);
    let classlist: &str = &format!("body {}", theme)[..];
    let description = &config.description;
    let header = create_header(config);
    let homepage = config.homepage.as_ref().map(|homepage| {
        html!(
          <meta property="og:url" content=homepage/>
        )
    });
    let banner = repo_banner(config);

    let doc: DOMTree<String> = html!(
    <html lang="en" id="oranda">
    <head>
    <title>{ text!(&config.name) }</title>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    { homepage }
    <meta name="description" content=description />
    <meta property="og:description" content=description/>
    <link rel="stylesheet" href="styles.css"></link>
    </head>
    <body>
    <div class=classlist>
        {banner}
        <div class="container">{header}{ unsafe_text!(content) }</div>
    </div>
    </body>
    </html>
     );
    doc.to_string()
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
