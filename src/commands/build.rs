use std::path::PathBuf;

use clap::Parser;

use crate::config::Config;
use crate::errors::*;
use crate::message::{Message, MessageType};
use crate::site::Site;

#[derive(Debug, Parser)]
pub struct Build {
    #[arg(long, default_value = "./")]
    path: PathBuf,
}

impl Build {
    pub fn run(&self) -> Result<()> {
        Message::new(MessageType::Info, "Running build...").print_and_log();
        let config = Config::build()?;
        Message::new(MessageType::Debug, &config.to_string()).log();
        Site::write(&config)?;
        Ok(())
    }
}
