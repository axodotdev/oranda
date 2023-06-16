use crate::config::Config;
use crate::data::Context;
use crate::errors::*;

mod installers;
mod table;

use axohtml::html;

pub fn page(context: &Context, config: &Config) -> Result<String> {
    let Some(release) = context.latest() else {
        return Ok(String::new());
    };

    let header = installers::build_header(release, config)?;
    let artifact_table = table::build(release, config)?;

    Ok(html!(
    <div>
        <div>
            {header}
        </div>
        <div>
            {artifact_table}
        </div>
    </div>
    )
    .to_string())
}

pub fn header(context: &Context, config: &Config) -> Result<String> {
    let Some(release) = context.latest() else {
        return Ok(String::new());
    };

    let header = installers::build_header(release, config)?;
    Ok(header.to_string())
}
