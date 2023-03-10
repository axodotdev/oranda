use crate::config::Config;
use axohtml::elements::footer;
use axohtml::{html, text};

pub fn create_footer(config: &Config) -> Box<footer<String>> {
    let mut repository = None;
    if let Some(repo) = &config.repository {
        repository = Some(html!(
            <a href=repo>
                <div class="github-icon" aria-hidden="true"/>
            </a>
        ));
    }
    let license_text = if let Some(license) = &config.license {
        format!(", {} license.", license)
    } else {
        String::new()
    };
    let text = format!(
        "{name}{license}",
        name = &config.name,
        license = license_text
    );

    html!(
        <footer class="footer">
            {repository}
            <span>{text!(text)}</span>
        </footer>
    )
}
