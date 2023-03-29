#![allow(clippy::uninlined_format_args)]

use std::path::Path;
use std::{fs, sync::Mutex};

use clap::{Parser, Subcommand};
use tracing::{error, Level};
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

type ReportErrorFunc = dyn Fn(&miette::Report) + Send + Sync + 'static;

static REPORT_ERROR: Mutex<Option<Box<ReportErrorFunc>>> = Mutex::new(None);

#[allow(dead_code)]
fn set_report_errors_as_json() {
    *REPORT_ERROR.lock().unwrap() = Some(Box::new(move |error| {
        use std::io::Write;

        // Still write a human-friendly version to stderr
        writeln!(&mut std::io::stderr(), "{error:?}").unwrap();
        // Manually invoke JSONReportHandler to format the error as a report
        // to out_.
        let mut report = String::new();
        miette::JSONReportHandler::new()
            .render_report(&mut report, error.as_ref())
            .unwrap();
        writeln!(&mut std::io::stdout(), r#"{{"error": {report}}}"#).unwrap();
    }));
}

fn report_error(error: &miette::Report) {
    {
        let guard = REPORT_ERROR.lock().unwrap();
        if let Some(do_report) = &*guard {
            do_report(error);
            return;
        }
    }

    let message = format!("{:?}", error);
    Message::new(MessageType::Error, &message).print();
    error!("{}", message);
}

fn main() {
    // Control how errors are formatted by setting the miette hook. This will
    // only be used for errors presented to humans, when formatting an error as
    // JSON, it will be handled by a custom `report_error` override, bypassing
    // the hook.
    let using_log_file = false;
    miette::set_hook(Box::new(move |_| {
        let graphical_theme = if console::colors_enabled_stderr() && !using_log_file {
            miette::GraphicalTheme::unicode()
        } else {
            miette::GraphicalTheme::unicode_nocolor()
        };
        Box::new(
            miette::MietteHandlerOpts::new()
                .graphical_theme(graphical_theme)
                .build(),
        )
    }))
    .expect("failed to initialize error handler");

    // Now that miette is set up, use it to format panics.
    std::panic::set_hook(Box::new(move |panic_info| {
        use miette::Diagnostic;
        use thiserror::Error;

        let payload = panic_info.payload();
        let message = if let Some(msg) = payload.downcast_ref::<&str>() {
            msg
        } else if let Some(msg) = payload.downcast_ref::<String>() {
            &msg[..]
        } else {
            "something went wrong"
        };

        #[derive(Debug, Error, Diagnostic)]
        #[error("{message}")]
        pub struct PanicError {
            pub message: String,
            #[help]
            pub help: Option<String>,
        }

        report_error(
            &miette::Report::from(PanicError {
                message: message.to_owned(),
                help: panic_info
                    .location()
                    .map(|loc| format!("at {}:{}:{}", loc.file(), loc.line(), loc.column())),
            })
            .wrap_err("oranda panicked"),
        );
    }));

    // Now finally setup tokio's runtime
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
            report_error(&miette::Report::from(e));
        }
    };
}
