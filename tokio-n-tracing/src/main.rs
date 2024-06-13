use std::time::Duration;

use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::resource::{
    ResourceDetector, SdkProvidedResourceDetector, TelemetryResourceDetector,
};
use opentelemetry_sdk::{trace as sdktrace, Resource};
use opentelemetry_semantic_conventions::resource as otel_resource;
use tracing::{debug, info};
use tracing_subscriber::{layer::*, util::*, EnvFilter};

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "debug");
        }
    }

    let otlp_endpoint = "http://localhost:8888";
    let resource = otel_resource();
    let tracer = otel_tracer(otlp_endpoint, resource);

    let traces_layer = tracing_opentelemetry::layer()
        .with_tracer(tracer)
        .with_filter(EnvFilter::from_default_env());

    let stdout_layer = tracing_subscriber::fmt::Layer::default()
        .compact()
        .with_filter(EnvFilter::from_default_env());

    tracing_subscriber::registry()
        .with(traces_layer)
        .with(stdout_layer)
        .try_init()
        .expect("Unable to initialize tracing subscriber");

    info!("Prepare for adding");
    let result = trace_me(5, 2).await;
    info!("Result of adding 5 and 2: {}", result);
}

#[tracing::instrument]
async fn trace_me(a: i32, b: i32) -> i32 {
    debug!("Adding {} and {}", a, b);
    a + b
}

fn otel_tracer(endpoint: &str, resource: Resource) -> sdktrace::Tracer {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(endpoint),
        )
        .with_trace_config(sdktrace::config().with_resource(resource))
        .with_batch_config(sdktrace::BatchConfigBuilder::default().build())
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("Unable to initialize OtlpPipeline for traces")
}
pub fn otel_resource() -> Resource {
    // let os_resource = OsResourceDetector.detect(Duration::from_secs(0));
    // let process_resource =
    // ProcessResourceDetector.detect(Duration::from_secs(0));
    let telemetry_resource = TelemetryResourceDetector.detect(Duration::from_secs(0));
    let sdk_resource = SdkProvidedResourceDetector.detect(Duration::from_secs(0));

    let provided = Resource::new(vec![
        KeyValue::new(otel_resource::SERVICE_NAME, "tokio_n_tracing_service"),
        KeyValue::new(
            otel_resource::SERVICE_NAMESPACE,
            "tokio_n_tracing_namespace",
        ),
        KeyValue::new(otel_resource::SERVICE_VERSION, "0.0.1"),
        KeyValue::new(otel_resource::SERVICE_INSTANCE_ID, "127.0.0.1"),
        KeyValue::new(otel_resource::DEPLOYMENT_ENVIRONMENT, "development"),
    ]);

    sdk_resource.merge(&provided).merge(&telemetry_resource)
    // .merge(&os_resource)
    // .merge(&process_resource)
}
