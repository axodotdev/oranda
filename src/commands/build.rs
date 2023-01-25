use std::path::PathBuf;

use clap::Parser;

use oranda::config::Config;
use oranda::errors::*;
use oranda::site::Site;

#[derive(Debug, Parser)]
pub struct Build {
    #[arg(long, default_value = "./")]
    project_root: PathBuf,
    #[arg(long, default_value = "./oranda.json")]
    config_path: PathBuf,
}

impl Build {
    pub fn run(&self) -> Result<()> {
        let config = Config::build(&self.config_path)?;
        Site::write(&config)?;
        Ok(())
    }
}
