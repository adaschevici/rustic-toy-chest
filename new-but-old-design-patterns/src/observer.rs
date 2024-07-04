use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
trait Observer: Send + Sync {
    async fn update(&self, event: &ZooEvent);
}

struct Zoo {
    observers: Vec<Arc<dyn Observer>>,
}

impl Zoo {
    fn new() -> Zoo {
        Zoo {
            observers: Vec::new(),
        }
    }

    fn add_observer(&mut self, observer: Arc<dyn Observer>) {
        self.observers.push(observer);
    }

    async fn notify_observers(&self, event: &ZooEvent) {
        let mut tasks = vec![];
        for observer in &self.observers {
            let observer = observer.clone();
            let event = event.clone();
            tasks.push(tokio::spawn(async move {
                observer.update(&event).await;
            }));
        }
        for task in tasks {
            task.await.unwrap();
        }
    }
}

#[derive(Clone)]
struct ZooEvent {
    description: String,
}

impl ZooEvent {
    fn new(description: &str) -> ZooEvent {
        ZooEvent {
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
        println!("{} received an event: {}", self.name, event.description);
    }
}

struct Veterinarian {
    name: String,
}

#[async_trait]
impl Observer for Veterinarian {
    async fn update(&self, event: &ZooEvent) {
        println!("{} received an event: {}", self.name, event.description);
    }
}

pub async fn run_observer() {
    let mut zoo = Zoo::new();
    let zookeeper = Arc::new(Zookeeper {
        name: "Alice".to_string(),
    });
    let vet = Arc::new(Veterinarian {
        name: "Bob".to_string(),
    });

    zoo.add_observer(zookeeper);
    zoo.add_observer(vet);

    let event = ZooEvent::new("The lion has been fed.");
    zoo.notify_observers(&event).await;
}
