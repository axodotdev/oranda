use std::collections::HashMap;

use crate::config::Config;
use crate::data::artifacts::InstallMethod;
use crate::data::{Context, Release};
use crate::errors::*;

mod installers;
mod table;

use axohtml::elements::div;
use axohtml::{html, text};

pub fn page(context: &Context, config: &Config) -> Result<String> {
    let Some(release) = context.latest() else {
        return Ok(String::new());
    };

    let header = installers::build_header(release, config)?;
    let installer_scripts = scripts(release, config)?;
    let artifact_table = table::build(release, config)?;

    Ok(html!(
    <div>
        <div>
            {header}
        </div>
        <div class="package-managers-downloads">
            {installer_scripts}
        </div>
        <div>
            {artifact_table}
        </div>
    </div>
    )
    .to_string())
}

pub fn scripts(release: &Release, config: &Config) -> Result<Vec<Box<div<String>>>> {
    // We only display runnable scripts here
    let mut scripts = HashMap::new();
    for (_, installer) in release.artifacts.installers() {
        let InstallMethod::Run { .. } = &installer.method else {
            continue;
        };
        scripts.insert(installer.label.clone(), installer);
    }

    // Sort by label name for now
    let mut scripts: Vec<_> = scripts.into_iter().collect();
    scripts.sort_by(|(label1, _), (label2, _)| label1.cmp(label2));

    let mut output = vec![];
    for (label, installer) in scripts {
        let InstallMethod::Run { file, run_hint } = &installer.method else {
            continue;
        };
        let script = installers::run_html(*file, run_hint, release, config);
        output.push(html!(
        <div>
            <h3>{text!(label)}</h3>
            {script}
        </div>
        ));
    }
    Ok(output)
}

pub fn header(context: &Context, config: &Config) -> Result<String> {
    let Some(release) = context.latest() else {
        return Ok(String::new());
    };

    let header = installers::build_header(release, config)?;
    Ok(header.to_string())
}
