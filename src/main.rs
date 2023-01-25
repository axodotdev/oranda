use clap::{Parser, Subcommand};

mod commands;
mod config;
mod errors;
mod site;
pub mod tests;

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
fn main() -> Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .max_blocking_threads(128)
        .enable_all()
        .build()
        .expect("Initializing tokio runtime failed");
    let _guard = runtime.enter();
    let cli = Cli::parse();

    match &cli.command {
        Command::Build(cmd) => cmd.run(),
        Command::Serve(cmd) => cmd.run(),
    }
}
