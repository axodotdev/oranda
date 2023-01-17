use std::path::PathBuf;

use clap::Parser;

use crate::config::Config;
use crate::errors::*;
use crate::site::Site;

#[derive(Debug, Parser)]
pub struct Build {
    #[arg(long, default_value = "./")]
    path: PathBuf,
}

impl Build {
    pub fn run(&self) -> Result<()> {
        let config = Config::build()?;
        println!("final config {:?}", config);
        Site::write(&config)?;
        Ok(())
    }
}
