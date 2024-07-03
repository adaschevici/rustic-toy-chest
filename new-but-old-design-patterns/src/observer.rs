use async_trait::async_trait;
use tracing::info;

#[async_trait]
trait Observer {
    async fn update(&self);
}

struct Zoo {
    observers: Arc<Vec<Box<dyn Observer>>>,
}

impl Zoo {
    fn new() -> Self {
        Self {
            observers: Arc::new(Vec::new()),
        }
    }

    fn add_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    async fn notify_observers(&self) {
        for observer in self.observers.iter() {
            observer.update().await;
        }
    }
}

struct ZooEvent {
    description: String,
}

impl ZooEvent {
    fn new(description: &str) -> Self {
        Self {
            description: description.to_string(),
        }
    }
}

pub async fn run_observer() {
    info!("Running observer pattern example");
}
