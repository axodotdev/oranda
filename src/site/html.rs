use crate::config::Config;
use crate::errors::*;
use crate::site::artifacts;
use crate::site::layout;

use axohtml::{html, unsafe_text};

pub fn build(config: &Config, content: String, is_index: bool) -> Result<String> {
    let artifacts_tabs = artifacts::create_header(config)?;
    let home_content = if is_index {
        html!(<div>{artifacts_tabs}{unsafe_text!(content)}</div>)
    } else {
        html!(<div>{unsafe_text!(content)}</div>)
    };

    let doc = layout::build(config, home_content, is_index)?;
    Ok(doc)
}
