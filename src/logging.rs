use tracing_subscriber::{EnvFilter, FmtSubscriber};
use tracing_subscriber::fmt::format::Writer;
use tracing::{Event, Subscriber};
use tracing_subscriber::registry::LookupSpan;

/// Custom event formatter that only prints the message
struct QuietFormatter;

impl<S, N> tracing_subscriber::fmt::FormatEvent<S, N> for QuietFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> tracing_subscriber::fmt::FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        // Create a visitor to extract only the message
        let mut visitor = MessageExtractor::new();
        event.record(&mut visitor);
        
        if let Some(message) = visitor.message {
            writeln!(writer, "{}", message)
        } else {
            writeln!(writer, "")
        }
    }
}

/// Field visitor that extracts only the message field
struct MessageExtractor {
    message: Option<String>,
}

impl MessageExtractor {
    fn new() -> Self {
        Self { message: None }
    }
}

impl tracing::field::Visit for MessageExtractor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" && self.message.is_none() {
            self.message = Some(format!("{:?}", value));
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" && self.message.is_none() {
            self.message = Some(value.to_string());
        }
    }
}

/// Initialize logging with the specified verbosity level
pub fn init_logging(verbose_level: Option<&String>) {
    if let Some(level) = verbose_level {
        // Verbose mode with specified log level
        let env_filter = match level.to_lowercase().as_str() {
            "trace" => EnvFilter::new("trace"),
            "debug" => EnvFilter::new("debug"),
            "info" => EnvFilter::new("info"),
            "warn" => EnvFilter::new("warn"),
            "error" => EnvFilter::new("error"),
            _ => {
                eprintln!("Invalid log level '{}', using 'info' instead", level);
                EnvFilter::new("info")
            }
        };

        let subscriber = FmtSubscriber::builder()
            .with_env_filter(env_filter)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(true)
            .with_line_number(true)
            .with_ansi(atty::is(atty::Stream::Stderr))
            .finish();

        if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
            eprintln!("Logging initialization failed: {e}");
        }
    } else {
        // Default quiet mode - only show messages
        let env_filter = EnvFilter::new("info");

        let subscriber = FmtSubscriber::builder()
            .with_env_filter(env_filter)
            .event_format(QuietFormatter)
            .finish();

        if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
            eprintln!("Logging initialization failed: {e}");
        }
    }
}

/// Get a logger for a specific module
pub fn get_logger(module: &str) -> tracing::Span {
    tracing::info_span!("krik", module = module)
}
