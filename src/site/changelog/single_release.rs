use crate::errors::*;
use crate::site::markdown::{self, SyntaxTheme};
use axohtml::dom::UnsafeTextNode;
use axohtml::elements::section;
use axohtml::html;
use axohtml::{text, unsafe_text};

use super::types::ReleasesApiResponse;

pub fn build_single_release(
    release: &ReleasesApiResponse,
    syntax_theme: &SyntaxTheme,
) -> Result<Box<section<String>>> {
    let body = match &release.body {
        Some(md) => markdown::to_html(md.to_string(), &syntax_theme)?,
        None => String::new(),
    };

    Ok(html!(
    <section class="release">
        <h2>{text!(&release.name)}</h2>
        <h5 class="flex items-center gap-2">
            {tag_icon()}{text!(&release.tag_name)}
        </h5>
        <div class="release-body">
            {unsafe_text!(body)}
        </div>
    </section>
    ))
}

pub fn tag_icon() -> Box<UnsafeTextNode<String>> {
    unsafe_text!("<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='w-6 h-6'>
    <path stroke-linecap='round' stroke-linejoin='round' d='M9.568 3H5.25A2.25 2.25 0 003 5.25v4.318c0 .597.237 1.17.659 1.591l9.581 9.581c.699.699 1.78.872 2.607.33a18.095 18.095 0 005.223-5.223c.542-.827.369-1.908-.33-2.607L11.16 3.66A2.25 2.25 0 009.568 3z' />
    <path stroke-linecap='round' stroke-linejoin='round' d='M6 6h.008v.008H6V6z' /></svg>")
}
