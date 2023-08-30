use camino::Utf8PathBuf;
use clap::{Parser, ValueEnum};
use oranda::errors::Result;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, ValueEnum)]
pub enum GenerateType {
    /// Generates a GitHub Actions CI file that can be used to deploy your site to GitHub Pages.
    Ci,
}

#[derive(Debug, Parser)]
pub struct Generate {
    /// What type of thing to generate.
    #[clap(value_enum)]
    kind: GenerateType,
    /// Path to the output file. For `ci`, this is `.github/workflows/web.yml` by default.
    #[arg(short, long)]
    path: Option<Utf8PathBuf>,
}

impl Generate {
    pub fn run(&self) -> Result<()> {
        let path = self.path.clone().unwrap_or_else(|| self.default_path());
        match self.kind {
            GenerateType::Ci => oranda::generate::generate_ci(path)?,
        };
        Ok(())
    }

    fn default_path(&self) -> Utf8PathBuf {
        match self.kind {
            GenerateType::Ci => Utf8PathBuf::from(".github/workflows/web.yml"),
        }
    }
}
