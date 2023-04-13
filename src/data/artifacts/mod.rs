pub mod cargo_dist;
mod package_managers;
pub mod page;

use crate::config::Config;
use crate::errors::*;
use axohtml::dom::UnsafeTextNode;
use axohtml::elements::div;
use axohtml::unsafe_text;

pub fn get_copyicon() -> Box<UnsafeTextNode<String>> {
    // axohtml does not support SVG for now
    let copy_icon   = unsafe_text!("<svg stroke='currentColor' fill='currentColor' stroke-width='0' viewBox='0 0 20 20' height='1em' width='1em' xmlns='http://www.w3.org/2000/svg'><path d='M8 2a1 1 0 000 2h2a1 1 0 100-2H8z'></path><path d='M3 5a2 2 0 012-2 3 3 0 003 3h2a3 3 0 003-3 2 2 0 012 2v6h-4.586l1.293-1.293a1 1 0 00-1.414-1.414l-3 3a1 1 0 000 1.414l3 3a1 1 0 001.414-1.414L10.414 13H15v3a2 2 0 01-2 2H5a2 2 0 01-2-2V5zM15 11h2a1 1 0 110 2h-2v-2z'></path></svg>");

    copy_icon
}

pub fn build_header(config: &Config) -> Result<Option<Box<div<String>>>> {
    if let Some(artifact) = &config.artifacts {
        page::build(config)?;
        if artifact.cargo_dist.is_some() {
            Ok(Some(cargo_dist::build(config)?))
        } else if let Some(package_managers) = &artifact.package_managers {
            Ok(Some(package_managers::build(config, package_managers)?))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
