use async_trait::async_trait;
use tracing::info;

#[async_trait]
trait Observer {
    async fn update(&self);
}

pub async fn run_observer() {
    info!("Running observer pattern example");
}
