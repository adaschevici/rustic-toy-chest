use configs::Configurations;
use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry::{trace::TraceError, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
use std::net::SocketAddr;
use tokio::signal;
use tracing::info;
use tracing_subscriber::prelude::*;

mod app;
mod configs;
mod database;
mod models;
mod schema;

fn init_tracer(config: &Configurations) -> Result<opentelemetry_sdk::trace::Tracer, TraceError> {
    let tracer =
        opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(config.tracing.host.clone()),
            )
            .with_trace_config(sdktrace::config().with_resource(Resource::new(vec![
                KeyValue::new("service.name", config.service.name.clone()),
            ])))
            .install_batch(runtime::Tokio);
    // .with_service_name(&config.service.name)
    // .with_tags(vec![KeyValue::new(
    //     "version",
    //     config.service.version.clone(),
    // )])
    // .install_batch(opentelemetry::runtime::Tokio)
    // .expect("Failed to install Jaeger pipeline.");
    tracer
}

#[tokio::main]
async fn main() {
    let config = Configurations::new().expect("Failed to read configuration.");
    let app_state = database::get_connection_pool(&config);
    let app = app::create_app(app_state);
    let tracer = init_tracer(&config).expect("Failed to initialize tracer.");
    let fmt_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from(&config.logger.level))
        .with(fmt_layer)
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();

    let address: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .expect("Failed to bind address");
    info!("Starting server on {:?}", address);

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to run server");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    shutdown_tracer_provider();
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
