use axohtml::elements::div;

use crate::config::Config;
use crate::errors::*;
use crate::site::artifacts::{installers, package_managers};

pub fn build(config: &Config) -> Result<Option<Box<div<String>>>> {
    if let Some(artifact) = &config.artifacts {
        if artifact.cargo_dist.is_some() {
            Ok(Some(installers::build_header(config)?))
        } else if let Some(package_managers) = &artifact.package_managers {
            Ok(Some(package_managers::build_header(
                config,
                package_managers,
            )?))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
