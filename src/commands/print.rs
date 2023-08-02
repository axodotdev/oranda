use clap::Parser;
use oranda::errors::*;

#[derive(Debug, Parser)]
pub struct ConfigSchema {}

impl ConfigSchema {
    pub fn run(&self) -> Result<()> {
        let schema = schemars::schema_for!(oranda::config::OrandaLayer);
        let json_schema =
            serde_json::to_string_pretty(&schema).expect("failed to stringify schema!?");
        println!("{json_schema}");
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct GenerateCss {}

impl GenerateCss {
    pub fn run(&self) -> Result<()> {
        oranda::site::layout::css::build_css("dist")?;
        tracing::info!("CSS placed in dist/oranda.css");
        Ok(())
    }
}
