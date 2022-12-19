use axohtml::{dom::DOMTree, html, text, unsafe_text};

use crate::config::{theme, Config};

pub fn build(config: &Config, content: String) -> String {
    let theme = theme::css_class(&config.theme);
    let classlist: &str = &format!("body container {}", theme)[..];
    let description = &config.description;
    let homepage = match &config.homepage {
        Some(homepage) => Some(html!(
            <meta property="og:url" content=homepage/>
        )),
        None => None,
    };

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
    <div class=classlist>{ unsafe_text!(content) }</div>
    </body>
    </html>
     );
    doc.to_string()
}
