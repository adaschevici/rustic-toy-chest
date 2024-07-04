trait Observer {
    fn update(&self, event: &ZooEvent);
}

struct Zoo {
    observers: Vec<Box<dyn Observer>>,
}
impl Zoo {
    fn new() -> Zoo {
        Zoo {
            observers: Vec::new(),
        }
    }

    fn add_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn notify_observers(&self, event: &ZooEvent) {
        for observer in &self.observers {
            observer.update(event);
        }
    }
}

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
impl Observer for Zookeeper {
    fn update(&self, event: &ZooEvent) {
        println!("{} received an event: {}", self.name, event.description);
    }
}
struct Veterinarian {
    name: String,
}
impl Observer for Veterinarian {
    fn update(&self, event: &ZooEvent) {
        println!("{} received an event: {}", self.name, event.description);
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

    zoo.add_observer(zookeeper);
    zoo.add_observer(vet);

    let event = ZooEvent::new("The lion has been fed.");
    zoo.notify_observers(&event);
}
