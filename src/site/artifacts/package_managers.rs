use crate::config::Config;
use crate::errors::*;
use axohtml::elements::div;
use axohtml::{html, text, unsafe_text};

use crate::site::markdown::syntax_highlight::syntax_highlight;
use crate::site::markdown::syntax_highlight::syntax_themes::SyntaxTheme;
use linked_hash_map::LinkedHashMap;

pub fn create_package_install_code(code: &str, syntax_theme: &SyntaxTheme) -> String {
    let highlighted_code = syntax_highlight(Some("sh"), code, &syntax_theme);
    match highlighted_code {
        Ok(code) => code,
        Err(_) => format!("<code class='text-center break-all'>{}</code>", code),
    }
}

pub fn build(
    config: &Config,
    package_managers: &LinkedHashMap<String, String>,
) -> Result<Box<div<String>>> {
    let (manager, hint) = if let Some((manager, hint)) = package_managers.front() {
        (manager, hint)
    } else {
        return Err(OrandaError::Other(String::from(
            "You are using package managers but none is present, please add one.",
        )));
    };
    let install_code = create_package_install_code(hint.as_str(), &config.syntax_theme);

    Ok(html!(<div>
    <h4 class="text-center">{text!(format!("Install with {}", manager))}</h4>
    {unsafe_text!(install_code)}
    <div>
        <a href="/artifacts.html" class="download-all">{text!("View all downloads")}</a>
    </div>
</div>))
}
