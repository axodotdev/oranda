use crate::config::Config;
use crate::report::Report;
use crate::site::Site;
use errors::*;

pub mod config;
pub mod errors;
pub mod report;
pub mod site;

pub fn exec() -> Result<Report> {
    let config = Config::build()?;
    Site::write(&config)?;
    Ok(Report {})
}
