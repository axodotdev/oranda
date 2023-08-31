use camino::Utf8PathBuf;
use clap::{Parser, Subcommand, ValueEnum};
use oranda::errors::Result;

#[derive(Debug, Subcommand)]
pub enum GenerateCommand {
    /// Generates a CI file that can be used to deploy your site.
    Ci(Ci),
}

#[derive(Debug, Parser)]
pub struct Ci {
    /// What CI to generate a file for.
    #[arg(long, default_value_t = CiType::Github)]
    #[clap(value_enum)]
    ci: CiType,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, ValueEnum)]
pub enum CiType {
    /// Deploy to GitHub Pages using GitHub Actions
    Github,
}

#[derive(Debug, Parser)]
pub struct Generate {
    /// What type of thing to generate.
    #[command(subcommand)]
    kind: GenerateCommand,
    /// Path to the output file.
    #[arg(short, long)]
    #[clap(global = true)]
    output_path: Option<Utf8PathBuf>,
    /// Path to your oranda site
    #[arg(short, long)]
    #[clap(global = true)]
    site_path: Option<Utf8PathBuf>,
}

impl Generate {
    pub fn run(&self) -> Result<()> {
        let path = self
            .output_path
            .clone()
            .unwrap_or_else(|| self.default_path());
        match self.kind {
            // TODO: Pass `CiType` in here when we add another one.
            GenerateCommand::Ci(_) => oranda::generate::generate_ci(path, &self.site_path)?,
        };
        Ok(())
    }

    fn default_path(&self) -> Utf8PathBuf {
        match self.kind {
            // TODO: Match on `CiType` when we add another one.
            GenerateCommand::Ci(_) => Utf8PathBuf::from(".github/workflows/web.yml"),
        }
    }
}
