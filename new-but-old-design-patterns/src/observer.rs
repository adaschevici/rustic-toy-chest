use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use tracing::info;

#[async_trait]
trait Observer: Send + Sync {
    async fn update(&self, event: &ZooEvent);
}

struct Zoo {
    observers: Arc<Mutex<Vec<Box<dyn Observer>>>>,
}

impl Zoo {
    fn new() -> Self {
        Self {
            observers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    async fn add_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.lock().unwrap().push(observer);
    }

    async fn notify_observers(&self, event: &ZooEvent) {
        let observers = self.observers.lock().unwrap();
        for observer in observers.iter() {
            observer.update(&event).await;
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

struct Zookeeper {
    name: String,
}

#[async_trait]
impl Observer for Zookeeper {
    async fn update(&self, event: &ZooEvent) {
        info!(
            "Zookeeper {} received an event {}",
            self.name, event.description
        );
    }
}

struct Veterinarian {
    name: String,
}

#[async_trait]
impl Observer for Veterinarian {
    async fn update(&self, event: &ZooEvent) {
        info!(
            "Veterinarian {} received an event {}",
            self.name, event.description
        );
    }
}

pub async fn run_observer() {
    let mut zoo = Zoo::new();
    let zookeeper = Box::new(Zookeeper {
        name: "Alice".to_string(),
    });

    let vet = Box::new(Veterinarian {
        name: "Bob".to_string(),
    });

    zoo.add_observer(zookeeper).await;
    zoo.add_observer(vet).await;

    let event = ZooEvent::new("Tiger escaped");
    zoo.notify_observers(&event).await;
}
