use log::*;
use tracing_subscriber::{layer::*, util::*};

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::Layer::default().compact())
        .init();

    info!("Prepare for adding");
    let result = trace_me(5, 2).await;
    info!("Result of adding 5 and 2: {}", result);
}

async fn trace_me(a: i32, b: i32) -> i32 {
    debug!("Adding {} and {}", a, b);
    a + b
}
