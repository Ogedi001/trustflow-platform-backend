//! Logging helpers
//!
//! Wraps `tracing-subscriber` setup and provides convenience initialization.

use tracing_subscriber::{layer::SubscriberExt, EnvFilter, fmt, Registry};

/// Initialize logging with environment variable support.
///
/// `RUST_LOG` environment variable is respected for log level filtering.
pub fn init_logging() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_ansi(true);

    let subscriber = Registry::default().with(env_filter).with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_init_logging() {
        init_logging();
        tracing::info!("logging initialized");
    }
}
