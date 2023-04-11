#![allow(clippy::uninlined_format_args)]

use clap::builder::{PossibleValuesParser, TypedValueParser};
use clap::{Parser, Subcommand};
use miette::Report;
use tracing::level_filters::LevelFilter;

mod commands;
use commands::{Build, Dev, Serve};

pub mod message;
use message::OutputFormat;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    /// How verbose logging should be (log level)
    #[clap(long)]
    #[clap(default_value_t = LevelFilter::WARN)]
    #[clap(value_parser = PossibleValuesParser::new(["off", "error", "warn", "info", "debug", "trace"]).map(|s| s.parse::<LevelFilter>().expect("possible values are valid")))]
    #[clap(help_heading = "GLOBAL OPTIONS", global = true)]
    pub verbose: LevelFilter,

    /// The format of the output
    #[clap(long, value_enum)]
    #[clap(default_value_t = OutputFormat::Human)]
    #[clap(help_heading = "GLOBAL OPTIONS", global = true)]
    pub output_format: OutputFormat,
}

#[derive(Subcommand, Debug)]
enum Command {
    Build(Build),
    Dev(Dev),
    Serve(Serve),
}

fn main() {
    let cli = Cli::parse();

    axocli::CliAppBuilder::new("oranda")
        .verbose(cli.verbose)
        .json_errors(cli.output_format == OutputFormat::Json)
        .start(cli, run);
}

fn run(cli: &axocli::CliApp<Cli>) -> Result<(), Report> {
    match &cli.config.command {
        Command::Build(cmd) => cmd.run()?,
        Command::Dev(cmd) => cmd.clone().run()?,
        Command::Serve(cmd) => cmd.run()?,
    };
    Ok(())
}
