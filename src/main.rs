use std::fs;
use std::path::Path;

use clap::{Parser, Subcommand};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod commands;
use commands::{Build, Serve};

pub mod message;
use message::{Message, MessageType};

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
fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .max_blocking_threads(128)
        .enable_all()
        .build()
        .expect("Initializing tokio runtime failed");
    let _guard = runtime.enter();
    let cli = Cli::parse();

    // Build a subscriber and appender for printing messages to the screen and file on error
    let appender = tracing_appender::rolling::never("./", "oranda-debug.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(appender);
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_writer(non_blocking)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let oranda_output = match &cli.command {
        Command::Build(cmd) => cmd.run(),
        Command::Serve(cmd) => cmd.run(),
    };

    match oranda_output {
        Ok(_) => {
            if Path::new("./oranda-debug.log").exists() {
                fs::remove_file("./oranda-debug.log")
                    .expect("Encountered an error removing debug log file.");
            }
            Message::new(MessageType::Success, "Completed successfully.").print();
        }
        Err(e) => {
            Message::new(MessageType::Error, &e.to_string()).print();
            tracing::error!("{}", e.to_string());
        }
    };
}
