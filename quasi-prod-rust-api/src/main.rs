use configs::Configurations;
use opentelemetry::global::shutdown_tracer_provider;
use std::net::SocketAddr;
use tokio::signal;
use tracing::info;

mod app;
mod configs;
mod database;
mod models;
mod schema;

#[tokio::main]
async fn main() {
    let config = Configurations::new().expect("Failed to read configuration.");
    let app_state = database::get_connection_pool(&config);
    let app = app::create_app(app_state);

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
