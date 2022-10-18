use std::panic;

use clap::Parser;
use cli::{Cli, OutputFormat};
use console::Term;
use miette::IntoDiagnostic;
use thiserror::Error;

use report::Report;

mod cli;
mod report;

fn main() {
    let cli = Cli::parse();

    // Init the logger
    tracing_subscriber::fmt::fmt()
        .with_max_level(cli.verbose)
        .with_target(false)
        .without_time()
        .with_ansi(console::colors_enabled_stderr())
        .init();

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
    panic::set_hook(Box::new(move |panic_info| {
        let payload = panic_info.payload();
        let message = if let Some(msg) = payload.downcast_ref::<&str>() {
            msg
        } else if let Some(msg) = payload.downcast_ref::<String>() {
            &msg[..]
        } else {
            "something went wrong"
        };

        #[derive(Debug, Error, miette::Diagnostic)]
        #[error("{message}")]
        pub struct PanicError {
            pub message: String,
            #[help]
            pub help: Option<String>,
        }

        Report::error(
            &miette::Report::from(PanicError {
                message: message.to_owned(),
                help: panic_info
                    .location()
                    .map(|loc| format!("at {}:{}:{}", loc.file(), loc.line(), loc.column())),
            })
            .wrap_err("oranda panicked"),
        );
    }));

    // If we're outputting JSON, replace the error report method such that it
    // writes errors out to the normal output stream as JSON.
    if cli.output_format == OutputFormat::Json {
        Report::as_json();
    }

    let main_result = real_main(&cli);

    let _ = main_result.map_err(|e| {
        Report::error(&e);
        std::process::exit(-1);
    });
}

fn real_main(cli: &Cli) -> Result<(), miette::Report> {
    let report = oranda::exec()?;
    let mut out = Term::stdout();

    match cli.output_format {
        OutputFormat::Human => report.print_human(&mut out).into_diagnostic()?,
        OutputFormat::Json => report.print_json(&mut out).into_diagnostic()?,
    }

    Ok(())
}
