use log::*;

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info,myapp=debug");
    }

    env_logger::init();

    info!("Before calling add");
    let result = trace_me(5, 3).await;
    info!("After calling add: {}", result);
}

async fn trace_me(a: i32, b: i32) -> i32 {
    debug!("Adding {} and {}", a, b);
    a + b
}
