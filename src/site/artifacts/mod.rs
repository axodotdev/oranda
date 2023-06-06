use crate::config::Config;
use crate::data::cargo_dist::DistRelease;
use crate::data::Context;
use crate::errors::*;

mod installers;
mod package_managers;
mod table;

use axohtml::html;

fn has_valid_setup(
    cargo_dist: bool,
    latest_dist_release: &Option<DistRelease>,
) -> Option<DistRelease> {
    if cargo_dist {
        if let Some(release) = latest_dist_release {
            return Some(release.clone());
        }
    }
    None
}

pub fn page(context: &Context, config: &Config) -> Result<String> {
    let artifacts = &config.artifacts;
    let release = &context.latest_dist_release;

    let (installer_list, artifact_table) =
        if let Some(release) = has_valid_setup(artifacts.cargo_dist(), release) {
            (
                Some(installers::build_list(&release, config)?),
                Some(table::build(release, config)?),
            )
        } else {
            (None, None)
        };

    let package_manager_list = artifacts
        .package_managers
        .as_ref()
        .map(|managers| package_managers::build_list(managers, config));

    Ok(html!(
        <div>
            <div class="package-managers-downloads">
            {installer_list}
            {package_manager_list}
        </div>
        <div>
            {artifact_table}
        </div>
    </div>
    )
    .to_string())
}

pub fn header(context: &Context, config: &Config) -> Result<String> {
    let artifacts = &config.artifacts;
    if artifacts.cargo_dist() {
        if let Some(release) = &context.latest_dist_release {
            return Ok(installers::build_header(release, config)?.to_string());
        }
    }
    if let Some(package_managers) = &artifacts.package_managers {
        return Ok(package_managers::build_header(config, package_managers)?.to_string());
    }
    Err(OrandaError::Other(
        "Can't have artifacts header with no artifacts".to_string(),
    ))
}
