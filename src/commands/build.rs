use camino::Utf8PathBuf;
use clap::Parser;

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
        if let Ok(Some(config)) = Site::get_workspace_config() {
            let sites = Site::build_multi(&config)?;
            // FIXME: Let the user turn this off
            if true {
                tracing::info!("Building workspace index page...");
                let mut member_data = Vec::new();
                for site in &sites {
                    member_data.push(site.workspace_data.clone().unwrap());
                }
                Site::build_and_write_workspace_index(&config, &member_data)?;
            }

            for site in sites {
                site.write(None)?;
            }
            let msg = format!(
                "Your site builds are located in `{}`.",
                config.build.dist_dir
            );
            tracing::info!(success = true, "{}", &msg);
        } else {
            let config = Config::build(&self.config_path)?;
            Site::build_single(&config, None)?.write(Some(&config))?;
            let msg = format!("Your site build is located in `{}`.", {
                config.build.dist_dir
            });
            tracing::info!(success = true, "{}", &msg);
        }
        Ok(())
    }
}
