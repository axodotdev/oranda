#![allow(clippy::uninlined_format_args)]

use clap::{Parser, Subcommand};
use miette::Report;
use tracing::subscriber::set_default;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;

mod commands;
use commands::{Build, ConfigSchema, Dev, GenerateCss, Serve};

pub mod formatter;
use formatter::OutputFormat;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    /// Whether to output more detailed debug information
    #[clap(short, long)]
    #[clap(help_heading = "GLOBAL OPTIONS", global = true)]
    pub verbose: bool,

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
    #[clap(hide = true)]
    ConfigSchema(ConfigSchema),
    #[clap(hide = true)]
    GenerateCss(GenerateCss),
}

fn main() {
    let cli = Cli::parse();

    axocli::CliAppBuilder::new("oranda")
        .json_errors(cli.output_format == OutputFormat::Json)
        .start(cli, run);
}

fn run(cli: &axocli::CliApp<Cli>) -> Result<(), Report> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .max_blocking_threads(128)
        .enable_all()
        .build()
        .expect("Initializing tokio runtime failed");
    let _guard = runtime.enter();
    let log_level = if cli.config.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    let sub_filter = tracing_subscriber::filter::Targets::new().with_target("oranda", log_level);
    let sub = tracing_subscriber::registry()
        .with(formatter::CaptureFieldsLayer)
        .with(tracing_subscriber::fmt::layer().event_format(formatter::OrandaFormatter))
        .with(sub_filter);
    let _sub_guard = set_default(sub);

    match &cli.config.command {
        Command::Build(cmd) => cmd.run()?,
        Command::Dev(cmd) => cmd.clone().run()?,
        Command::Serve(cmd) => cmd.run()?,
        Command::ConfigSchema(cmd) => cmd.run()?,
        Command::GenerateCss(cmd) => cmd.run()?,
    };
    Ok(())
}
