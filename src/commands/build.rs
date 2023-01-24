use std::path::PathBuf;

use clap::Parser;

use crate::config::Config;
use crate::errors::*;
use crate::message::{self, MessageType};
use crate::site::Site;

#[derive(Debug, Parser)]
pub struct Build {
    #[arg(long, default_value = "./")]
    path: PathBuf,
}

impl Build {
    pub fn run(&self) -> Result<()> {
        println!("{}", message::build(MessageType::Info, "Running build..."));
        let config = Config::build()?;
        tracing::debug!(
            "{}",
            message::build(MessageType::Debug, &config.to_string())
        );
        Site::write(&config)?;
        Ok(())
    }
}
