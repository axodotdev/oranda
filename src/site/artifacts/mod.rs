mod build;
mod cargo_dist;
mod package_managers;
use crate::config::Config;
use crate::errors::*;
use axohtml::elements::div;
use cargo_dist_schema::DistManifest;

fn fetch_manifest(config: &Config) -> std::result::Result<DistManifest, reqwest::Error> {
    let url = cargo_dist::create_download_link(config, &String::from("dist-manifest.json"));

    let resp = reqwest::blocking::get(url)?;

    resp.json::<DistManifest>()
}

pub fn build_header(config: &Config) -> Result<Option<Box<div<String>>>> {
    if let Some(artifact) = &config.artifacts {
        build::build_page(config)?;
        if artifact.cargo_dist.is_some() {
            Ok(Some(cargo_dist::build(&config)?))
        } else if let Some(package_managers) = &artifact.package_managers {
            Ok(Some(package_managers::build(&config, package_managers)?))
        } else {
            Ok(None)
        }
    } else {
        return Ok(None);
    }
}
