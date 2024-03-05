use axum::axum;
use configs::Configurations;
use std::net::SocketAddr;
use tracing::info;

mod app;
mod configs;

#[tokio::main]
async fn main() {
    let config = Configurations::new().expect("Failed to read configuration.");
    let address: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .expect("Failed to bind address");
    info!("Starting server on {:?}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(tokio::signal::ctrl_c())
        .await
        .expect("Failed to run server");
}
