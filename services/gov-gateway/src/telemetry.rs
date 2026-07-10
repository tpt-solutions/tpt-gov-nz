//! Tracing + OpenTelemetry initialisation for the gateway.
//!
//! Always installs a JSON `tracing` subscriber. When
//! `OTEL_EXPORTER_OTLP_ENDPOINT` is set, spans are additionally exported to an
//! OTLP collector over gRPC (e.g. an OpenTelemetry Collector, Jaeger, Tempo).
//! When it is unset, tracing runs locally with no exporter so the service still
//! starts cleanly in development.

use opentelemetry::KeyValue;
use opentelemetry_sdk::{trace::SdkTracerProvider, Resource};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

const SERVICE_NAME: &str = "gov-gateway";

/// Guard that flushes and shuts down the OTLP tracer provider on drop.
pub struct TelemetryGuard {
    provider: Option<SdkTracerProvider>,
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        if let Some(provider) = self.provider.take() {
            if let Err(e) = provider.shutdown() {
                eprintln!("error shutting down tracer provider: {e}");
            }
        }
    }
}

fn env_filter() -> EnvFilter {
    EnvFilter::try_from_env("RUST_LOG")
        .unwrap_or_else(|_| EnvFilter::new("gov_gateway=info,tower_http=debug"))
}

/// Initialise tracing and (optionally) OpenTelemetry export.
pub fn init() -> anyhow::Result<TelemetryGuard> {
    let fmt_layer = tracing_subscriber::fmt::layer().json();

    let endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .ok()
        .filter(|s| !s.is_empty());

    match endpoint {
        Some(endpoint) => {
            let provider = build_otlp_provider(&endpoint)?;
            let tracer = opentelemetry::trace::TracerProvider::tracer(&provider, SERVICE_NAME);
            let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

            tracing_subscriber::registry()
                .with(env_filter())
                .with(fmt_layer)
                .with(otel_layer)
                .init();

            tracing::info!(%endpoint, "OpenTelemetry OTLP tracing enabled");
            Ok(TelemetryGuard {
                provider: Some(provider),
            })
        }
        None => {
            tracing_subscriber::registry()
                .with(env_filter())
                .with(fmt_layer)
                .init();

            tracing::info!(
                "OpenTelemetry disabled (set OTEL_EXPORTER_OTLP_ENDPOINT to enable OTLP export)"
            );
            Ok(TelemetryGuard { provider: None })
        }
    }
}

fn build_otlp_provider(endpoint: &str) -> anyhow::Result<SdkTracerProvider> {
    use opentelemetry_otlp::WithExportConfig;

    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()?;

    let resource = Resource::builder()
        .with_attribute(KeyValue::new("service.name", SERVICE_NAME))
        .build();

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource)
        .build();

    opentelemetry::global::set_tracer_provider(provider.clone());
    Ok(provider)
}
