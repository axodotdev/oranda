use crate::site::logo;
use axohtml::{dom::DOMTree, html, text, unsafe_text};

use crate::config::{theme, Config};

pub fn build(config: &Config, content: String) -> String {
    let theme = theme::css_class(&config.theme);
    let classlist: &str = &format!("body container {}", theme)[..];
    let description = &config.description;
    let logo_dist_path = if let Some(logo_origin_path) = &config.logo {
        Some(logo::fetch(&config.dist_dir, logo_origin_path))
    } else {
        None
    };

    let doc: DOMTree<String> = html!(
    <html lang="en" id="oranda">
        <head>
            <title>{ text!(&config.name) }</title>
            <meta charset="utf-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <meta name="description" content=description />
            <link rel="stylesheet" href="styles.css"></link>
        </head>
        <body>
            <div class=classlist>
                { unsafe_text!(content) }
            </div>
        </body>
    </html>
     );
    doc.to_string()
}
