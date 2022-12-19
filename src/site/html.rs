use axohtml::{dom::DOMTree, html, text, unsafe_text};

use crate::config::{theme, Config};
use axohtml::elements::div;

pub fn build(config: &Config, content: String) -> String {
    let theme = theme::css_class(&config.theme);
    let classlist: &str = &format!("body container {}", theme)[..];
    let description = &config.description;
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
    <div class=classlist>{banner}{ unsafe_text!(content) }</div>
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
