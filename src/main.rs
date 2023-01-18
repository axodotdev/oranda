use clap::{Parser, Subcommand};

mod commands;
mod config;
mod errors;
mod site;

use commands::{Build, Serve};
use errors::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Build(Build),
    Serve(Serve),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Build(cmd) => cmd.run(),
        Command::Serve(cmd) => cmd.run(),
    }
}
