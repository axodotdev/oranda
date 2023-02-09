use crate::config::Config;
use crate::errors::*;
use crate::site::markdown::{syntax_highlight, SyntaxTheme};
use linked_hash_map::LinkedHashMap;

use axohtml::elements::div;
use axohtml::{html, text, unsafe_text};

use crate::site::artifacts::get_copyicon;

fn create_package_install_code(code: &str, syntax_theme: &SyntaxTheme) -> String {
    let highlighted_code = syntax_highlight(Some("sh"), code, syntax_theme);
    match highlighted_code {
        Ok(code) => code,
        Err(_) => format!("<code class='text-center break-all'>{}</code>", code),
    }
}
// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
pub fn build_list(managers: &LinkedHashMap<String, String>, config: &Config) -> Box<div<String>> {
    let mut list = vec![];
    for (manager, install_code) in managers.iter() {
        let copy_icon = get_copyicon();
        list.extend(html!(<li class="list-none"><h5>{text!(manager)}</h5> 
        <div class="install-code-wrapper">
        {unsafe_text!(create_package_install_code(install_code, &config.syntax_theme))}
        <button
            data-copy={install_code}
            class="business-button primary button">
            {copy_icon}
        </button>
    </div>
        
        
        </li>))
    }

    html!(
    <div>
        <h3>{text!("Install via package manager")}</h3>
        <ul>
            {list}
        </ul>
    </div>
    )
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
