//! Tracing setup utilities
//!
//! Provides helper for distributed tracing integration (e.g., with Jaeger or OpenTelemetry).

use opentelemetry::sdk::trace as sdktrace;
use opentelemetry::sdk::Resource;
use opentelemetry::KeyValue;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

/// Initialize tracing pipeline with optional OTLP exporter.
///
/// The `OTEL_EXPORTER_OTLP_ENDPOINT` env variable is consulted. If not set,
/// tracing will function locally without an exporter.
pub fn init_tracing(service_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // build OpenTelemetry tracer
    let otel_endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok();
    let tracer = if let Some(endpoint) = otel_endpoint {
        let exporter = opentelemetry_otlp::new_exporter().tonic().with_endpoint(endpoint);
        let provider = sdktrace::TracerProvider::builder()
            .with_simple_exporter(exporter)
            .with_resource(Resource::new(vec![KeyValue::new("service.name", service_name)]))
            .build();
        Some(provider.versioned_tracer("infrastructure", Some(env!("CARGO_PKG_VERSION")), None))
    } else {
        None
    };

    // create tracing subscriber
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let mut subscriber = tracing_subscriber::Registry::default().with(env_filter);

    if let Some(tracer) = tracer {
        let layer = tracing_opentelemetry::layer().with_tracer(tracer);
        subscriber = subscriber.with(layer);
    }

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_tracing_no_endpoint() {
        std::env::remove_var("OTEL_EXPORTER_OTLP_ENDPOINT");
        init_tracing("test-service").unwrap();
    }
}
