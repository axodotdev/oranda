use crate::config::Config;
use axohtml::elements::footer;
use axohtml::{html, text};
use chrono::Datelike;

pub fn create_footer(config: &Config) -> Box<footer<String>> {
    let mut repository = None;
    if let Some(repo) = &config.repository {
        repository = Some(html!(
            <a href=repo>
                <div class="footer github-icon" aria-hidden="true"/>
            </a>
        ));
    }
    let current_date = chrono::Utc::now();
    let year = current_date.year();
    let license_text = if let Some(license) = &config.license {
        format!(", {} license.", license)
    } else {
        String::new()
    };
    let text = format!(
        "{year}, {name}{license}",
        year = year,
        name = &config.name,
        license = license_text
    );

    return html!(
        <footer class="axo-gradient text-slate-50 flex w-full justify-between px-4 py-2 text-xs items-center">
            {repository}
            <span>{text!(text)}</span>
        </footer>
    );
}
