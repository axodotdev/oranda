use crate::errors::*;
use crate::site::link;
use crate::site::markdown::{self, SyntaxTheme};
use axohtml::dom::UnsafeTextNode;
use axohtml::elements::{a, section};
use axohtml::html;
use axohtml::{text, unsafe_text};
use chrono::DateTime;

use super::types::ReleasesApiResponse;

pub fn build_single_release(
    release: &ReleasesApiResponse,
    syntax_theme: &SyntaxTheme,
    version: &Option<String>,
    path_prefix: &Option<String>,
) -> Result<Box<section<String>>> {
    let cutoff = "\r\n\r\n## Install ";
    let body = match &release.body {
        Some(md) => {
            let cut_body = md.split(cutoff).collect::<Vec<&str>>()[0];

            markdown::to_html(cut_body.to_string(), syntax_theme)?
        }
        None => String::new(),
    };
    let id: axohtml::types::Id = axohtml::types::Id::new(release.tag_name.as_str());
    let formatted_date = DateTime::parse_from_rfc3339(&release.published_at)?
        .format("%b %e %Y at %R UTC")
        .to_string();

    let classnames = if release.prerelease {
        "release pre-release hidden"
    } else {
        "release"
    };

    let link = format!("#{}", &release.tag_name);
    Ok(html!(
    <section class=classnames>

        <h2 id=id><a href=link>{text!(&release.name)}</a></h2>
        <div class="release-info">
            <span class="flex items-center gap-2">
                {tag_icon()}{text!(&release.tag_name)}
            </span>
            <span class="flex items-center gap-2">
                {date_icon()}{text!(&formatted_date)}
            </span>
        </div>
        <div class="release-body mb-6">
            {unsafe_text!(body)}
        </div>
        {build_install_button(version, release, path_prefix)}
    </section>
    ))
}

fn build_install_button(
    version: &Option<String>,
    release: &ReleasesApiResponse,
    path_prefix: &Option<String>,
) -> Option<Box<a<String>>> {
    let is_installable = if let Some(v) = &version {
        release.tag_name.eq(&format!("v{}", v))
    } else {
        false
    };
    let downloads_href = link::generate(path_prefix, String::from("artifacts.html"));

    if is_installable {
        Some(
            html!(<a href=downloads_href><button class="business-button primary">{text!("Install")}</button></a>),
        )
    } else {
        None
    }
}

fn tag_icon() -> Box<UnsafeTextNode<String>> {
    unsafe_text!("<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-6 h-6'>
    <path stroke-linecap='round' stroke-linejoin='round' d='M9.568 3H5.25A2.25 2.25 0 003 5.25v4.318c0 .597.237 1.17.659 1.591l9.581 9.581c.699.699 1.78.872 2.607.33a18.095 18.095 0 005.223-5.223c.542-.827.369-1.908-.33-2.607L11.16 3.66A2.25 2.25 0 009.568 3z' />
    <path stroke-linecap='round' stroke-linejoin='round' d='M6 6h.008v.008H6V6z' /></svg>")
}

fn date_icon() -> Box<UnsafeTextNode<String>> {
    unsafe_text!("<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-6 h-6'>
    <path stroke-linecap='round' stroke-linejoin='round' d='M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5' /></svg>")
}
