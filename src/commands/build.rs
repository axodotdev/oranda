use camino::Utf8PathBuf;
use clap::Parser;

use crate::message::{Message, MessageType};
use oranda::config::Config;
use oranda::errors::*;
use oranda::site::Site;

#[derive(Debug, Parser)]
pub struct Build {
    /// DO NOT USE: Path to the root dir of the project
    ///
    /// This flag exists for internal testing. It is incorrectly implemented for actual
    /// end-users and will make you very confused and sad.
    #[clap(hide = true)]
    #[arg(long, default_value = "./")]
    project_root: Utf8PathBuf,
    /// DO NOT USE: Path to the oranda.json
    ///
    /// This flag exists for internal testing. It is incorrectly implemented for actual
    /// end-users and will make you very confused and sad.
    #[clap(hide = true)]
    #[arg(long, default_value = "./oranda.json")]
    config_path: Utf8PathBuf,
}

impl Build {
    pub fn new(project_root: Option<Utf8PathBuf>, config_path: Option<Utf8PathBuf>) -> Self {
        Build {
            project_root: project_root.unwrap_or(Utf8PathBuf::from("./")),
            config_path: config_path.unwrap_or(Utf8PathBuf::from("./oranda.json")),
        }
    }

    pub fn run(&self) -> Result<()> {
        Message::new(MessageType::Info, "Running build...").print();
        tracing::info!("Running build...");
        let config = Config::build(&self.config_path)?;
        Site::build(&config)?.write(&config)?;
        let msg = format!(
            "Successfully built your site in the `{}` directory. To view, run `oranda serve`.",
            { config.build.dist_dir }
        );
        Message::new(MessageType::Success, &msg).print();
        Ok(())
    }
}
