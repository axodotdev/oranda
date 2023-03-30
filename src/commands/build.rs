use camino::Utf8PathBuf;
use clap::Parser;

use crate::message::{Message, MessageType};
use oranda::config::Config;
use oranda::errors::*;
use oranda::site::Site;

#[derive(Debug, Default, Parser)]
pub struct Build {
    #[arg(long, default_value = "./")]
    project_root: Utf8PathBuf,
    #[arg(long, default_value = "./oranda.json")]
    config_path: Utf8PathBuf,
}

impl Build {
    pub fn new(project_root: Option<Utf8PathBuf>, config_path: Option<Utf8PathBuf>) -> Self {
        Build {
            project_root: project_root.unwrap_or(Build::default().project_root),
            config_path: config_path.unwrap_or(Build::default().config_path),
        }
    }

    pub fn run(&self) -> Result<()> {
        Message::new(MessageType::Info, "Running build...").print();
        tracing::info!("Running build...");
        let config = Config::build(&self.config_path)?;
        Site::build(&config)?.write(&config)?;
        let msg = format!(
            "Successfully built your site in the `{}` directory. To view, run `oranda serve`.",
            { config.dist_dir }
        );
        Message::new(MessageType::Success, &msg).print();
        Ok(())
    }
}
