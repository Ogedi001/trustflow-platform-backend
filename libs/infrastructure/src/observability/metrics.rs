//! Metrics helpers
//!
//! Sets up global metrics registry and exposes exporter types.

use metrics::{Gauge, Histogram, Key, Recorder, Unit};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

/// Holds exporter handle so that metrics can be scraped
pub struct MetricsExporter {
    handle: PrometheusHandle,
}

impl MetricsExporter {
    /// Expose the prometheus handle for retrieving metrics as text
    pub fn handle(&self) -> &PrometheusHandle {
        &self.handle
    }
}

/// Initialize global metrics recorder and return exporter handle. 
///
/// Usage:
/// ````rust
/// let exporter = infrastructure::observability::init_metrics();
/// // mount exporter.handle().render() to HTTP endpoint
/// ````
pub fn init_metrics() -> Result<MetricsExporter, Box<dyn std::error::Error>> {
    let builder = PrometheusBuilder::new();
    let handle = builder
        .install()?;

    Ok(MetricsExporter { handle })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_metrics() {
        let exporter = init_metrics().unwrap();
        let _ = exporter.handle().render();
    }
}
