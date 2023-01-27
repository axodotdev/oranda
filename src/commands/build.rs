use std::path::PathBuf;

use clap::Parser;

use crate::message::{Message, MessageType};
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
        Message::new(MessageType::Info, "Running build...").print();
        tracing::info!("Running build...");
        let config = Config::build(&self.config_path)?;
        Site::write(&config)?;
        let msg = format!(
            "Successfully built your site in the `{}` directory. To view, run `oranda serve`.",
            { config.dist_dir }
        );
        Message::new(MessageType::Success, &msg).print();
        Ok(())
    }
}
