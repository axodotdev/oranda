use clap::Parser;
use oranda::errors::*;

#[derive(Debug, Parser)]
pub struct ConfigSchema {}

impl ConfigSchema {
    pub fn run(&self) -> Result<()> {
        let schema = schemars::schema_for!(oranda::config::oranda_config::OrandaConfig);
        let json_schema = serde_json::to_string_pretty(&schema).unwrap();
        println!("{json_schema}");
        Ok(())
    }
}
