use camino::Utf8PathBuf;

use clap::Parser;

use crate::commands::{Build, Serve};
use oranda::errors::*;

#[derive(Debug, Parser)]
pub struct Dev {
    #[arg(long)]
    port: Option<u16>,
    #[arg(long)]
    project_root: Option<Utf8PathBuf>,
    #[arg(long)]
    config_path: Option<Utf8PathBuf>,
}

impl Dev {
    pub fn run(&self) -> Result<()> {
        Build::new(self.project_root.clone(), self.config_path.clone()).run()?;
        Serve::new(self.port).run()
    }
}
