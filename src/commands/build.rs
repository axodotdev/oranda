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
    /// Only build the artifacts JSON file (if applicable) and other files that may be used to
    /// support it, such as installer source files.
    #[arg(long)]
    json_only: bool,
}

impl Build {
    pub fn new(project_root: Option<Utf8PathBuf>, config_path: Option<Utf8PathBuf>) -> Self {
        Build {
            project_root: project_root.unwrap_or(Utf8PathBuf::from("./")),
            config_path: config_path.unwrap_or(Utf8PathBuf::from("./oranda.json")),
            json_only: false,
        }
    }

    pub fn run(&self) -> Result<()> {
        if let Some(config) = Site::get_workspace_config()? {
            let sites = Site::build_multi(&config, self.json_only)?;
            if config.workspace.generate_index && !self.json_only {
                tracing::info!("Building workspace index page...");
                let mut member_data = Vec::new();
                for site in &sites {
                    // Unwrap here because `Site::build_multi` always sets `workspace_data = Some(_)`.
                    // It's only set to `None` on a _single_ page build, which can't happen in this
                    // code path.
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
            if self.json_only {
                Site::build_single_json_only(&config, None)?;
            } else {
                Site::build_single(&config, None)?.write(Some(&config))?;
            }
            let msg = format!("Your site build is located in `{}`.", {
                config.build.dist_dir
            });
            tracing::info!(success = true, "{}", &msg);
        }
        Ok(())
    }
}
