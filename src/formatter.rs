use clap::ValueEnum;
use console::Color::{Green, Magenta, White, Yellow};
use console::Style;
use std::fmt::Debug;
use tracing::field::{Field, Visit};
use tracing::span::Attributes;
use tracing::{Event, Subscriber};
use tracing::{Id, Level};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

/// Our custom `FormatEvent` implementation.
pub struct OrandaFormatter;

/// Our custom layer that takes care of recording span information.
pub struct CaptureFieldsLayer;

/// Visitor implementation for _span_ fields.
#[derive(Default, Clone)]
struct SpanVisitor {
    pub prefix: Option<String>,
}

/// Visitor implementation for _event_ fields.
#[derive(Default)]
struct EventVisitor {
    pub message: String,
    pub success: bool,
}

/// Storage struct for our spans, accomplished by shoving it into the span `extensions` field.
struct SpanFieldStorage(SpanVisitor);

impl<S> Layer<S> for CaptureFieldsLayer
where
    S: Subscriber,
    S: for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let mut visitor = SpanVisitor::default();
        attrs.record(&mut visitor);
        let storage = SpanFieldStorage(visitor);
        let span = ctx.span(id).unwrap();
        if span.name() == "workspace_page" {
            let mut extensions = span.extensions_mut();
            extensions.insert::<SpanFieldStorage>(storage);
        }
    }
}

impl<S, N> FormatEvent<S, N> for OrandaFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    /// oranda's custom tracing formatter. We apply our own custom formatting, but we also check
    /// if the event has been fired inside of a `workspace_page` span, from which we get a prefix
    /// to attach to the log message.
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let warn_icon = Style::new().bold().fg(Yellow).apply_to("⚠");
        let check_icon = Style::new().bold().fg(Green).apply_to("✓");
        let arrow_icon = Style::new().bold().fg(White).apply_to("↪");
        let mut message = EventVisitor::default();
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

            // Fetch our custom span fields, if any.
            let fields = if let Some(span) = ctx.lookup_current() {
                let extensions = span.extensions();
                if let Some(storage) = extensions.get::<SpanFieldStorage>() {
                    let field_data = storage.0.clone();
                    Some(field_data)
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(fields) = fields {
                if let Some(prefix) = fields.prefix {
                    write!(&mut writer, "[{}] {}", prefix, output_str)?;
                } else {
                    write!(&mut writer, "{}", output_str)?;
                }
            } else {
                write!(&mut writer, "{}", output_str)?;
            }
        }

        writeln!(writer)
    }
}

impl Visit for EventVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
        if field.name() == "success" {
            self.success = true;
        }
    }
}

impl Visit for SpanVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "prefix" {
            self.prefix = Some(value.to_string());
        }
    }

    fn record_debug(&mut self, _field: &Field, _value: &dyn Debug) {}
}

/// Style of output we should produce
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable output
    Human,
    /// Machine-readable JSON output
    Json,
}
