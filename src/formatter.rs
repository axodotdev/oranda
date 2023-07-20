use clap::ValueEnum;
use console::Color::{Green, Magenta, White, Yellow};
use console::Style;
use std::fmt::Debug;
use tracing::field::{Field, Visit};
use tracing::Level;
use tracing::{Event, Subscriber};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;

pub struct OrandaFormatter;

#[derive(Default)]
struct LogMessage {
    pub prefix: Option<String>,
    pub message: String,
    pub success: bool,
}

impl<S, N> FormatEvent<S, N> for OrandaFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let warn_icon = Style::new().bold().fg(Yellow).apply_to("⚠");
        let check_icon = Style::new().bold().fg(Green).apply_to("✓");
        let arrow_icon = Style::new().bold().fg(White).apply_to("↪");
        let mut message = LogMessage::default();
        let metadata = event.metadata();
        event.record(&mut message);
        if metadata.fields().field("message").is_some() {
            // Handle the special success case first
            let output_str = if message.success {
                let style = Style::new().bold().fg(Green);
                format!(
                    "{} >o_o< SUCCESS: {}",
                    check_icon,
                    style.apply_to(message.message)
                )
            } else if matches!(metadata.level(), &Level::INFO) {
                let style = Style::new().bold().fg(White);
                format!(
                    "{} >o_o< INFO: {}",
                    arrow_icon,
                    style.apply_to(message.message)
                )
            } else if matches!(metadata.level(), &Level::WARN) {
                let style = Style::new().bold().fg(Yellow);
                format!(
                    "{} >o_o< WARNING: {}",
                    warn_icon,
                    style.apply_to(message.message)
                )
            } else if matches!(metadata.level(), &Level::DEBUG) {
                let style = Style::new().bold().fg(Magenta);
                format!(
                    "{} >o_o< DEBUG: {}",
                    arrow_icon,
                    style.apply_to(message.message)
                )
            } else {
                format!("TRACE: {}", message.message)
            };

            if message.prefix.is_some() {
                write!(&mut writer, "[{}] {}", message.prefix.unwrap(), output_str)?;
            } else {
                write!(&mut writer, "{}", output_str)?;
            }
        }

        writeln!(writer)
    }
}

impl Visit for LogMessage {
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "prefix" {
            self.prefix = Some(value.to_string());
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
        if field.name() == "prefix" {
            self.prefix = Some(format!("{:?}", value));
        }
        if field.name() == "success" {
            self.success = true;
        }
    }
}

/// Style of output we should produce
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable output
    Human,
    /// Machine-readable JSON output
    Json,
}
