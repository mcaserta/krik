use tracing_subscriber::{EnvFilter, FmtSubscriber};

/// Initialize logging with the specified verbosity level
pub fn init_logging(verbose: bool) {
    let env_filter = if verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
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

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
}

/// Get a logger for a specific module
pub fn get_logger(module: &str) -> tracing::Span {
    tracing::info_span!("krik", module = module)
}
