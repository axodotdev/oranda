use axoasset::LocalAsset;
use camino::Utf8PathBuf;
use clap::Parser;
use oranda::errors::*;
use oranda_generate_css::default_css_output_dir;

#[derive(Debug, Parser)]
pub struct ConfigSchema {
    /// Write the config schema to the named file instead of stdout
    #[clap(long)]
    pub output: Option<String>,
}

impl ConfigSchema {
    pub fn run(&self) -> Result<()> {
        let schema = schemars::schema_for!(oranda::config::OrandaLayer);
        let json_schema =
            serde_json::to_string_pretty(&schema).expect("failed to stringify schema!?");

        if let Some(output) = &self.output {
            let contents = json_schema + "\n";
            LocalAsset::write_new(&contents, output)?;
        } else {
            println!("{json_schema}");
        }
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct GenerateCss {
    #[clap(long)]
    out_dir: Option<Utf8PathBuf>,
}

impl GenerateCss {
    pub fn run(&self) -> Result<()> {
        let out_dir = self.out_dir.clone().unwrap_or_else(default_css_output_dir);
        let out_file = out_dir.join("oranda.css");
        oranda_generate_css::build_css(&out_dir)?;
        tracing::info!("CSS placed in {out_file}");
        Ok(())
    }
}
