mod build;
mod cargo_dist;
mod package_managers;

use crate::config::Config;
use crate::errors::*;
use axohtml::elements::div;

pub fn build_header(config: &Config) -> Result<Option<Box<div<String>>>> {
    if let Some(artifact) = &config.artifacts {
        build::build_page(config)?;
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
