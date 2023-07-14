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
