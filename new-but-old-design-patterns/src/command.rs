use async_trait::async_trait;

#[async_trait]
trait Command {
    async fn execute(&self);
}

struct AddAnimal {
    name: String,
    species: String,
}

#[async_trait]
impl Command for AddAnimal {
    async fn execute(&self) {
        println!("Adding a {} named {}", self.species, self.name);
    }
}

struct FeedAnimal {
    name: String,
    food: String,
}

#[async_trait]
impl Command for FeedAnimal {
    async fn execute(&self) {
        println!("Feeding {} some {}", self.name, self.food);
    }
}

struct MoveAnimal {
    name: String,
    enclosure: String,
}

#[async_trait]
impl Command for MoveAnimal {
    async fn execute(&self) {
        println!("Moving {} to the {} enclosure", self.name, self.enclosure);
    }
}

struct Zookeeper {
    commands: Vec<Box<dyn Command + Send + Sync>>,
}

impl Zookeeper {
    async fn new() -> Zookeeper {
        Zookeeper {
            commands: Vec::new(),
        }
    }

    async fn add_command(&mut self, command: Box<dyn Command + Send + Sync>) {
        self.commands.push(command);
    }

    async fn execute_commands(&self) {
        for command in &self.commands {
            command.execute().await;
        }
    }
}

pub async fn run_command() {
    let mut zookeeper = Zookeeper::new().await;
    zookeeper
        .add_command(Box::new(AddAnimal {
            name: "Leo".to_string(),
            species: "Lion".to_string(),
        }))
        .await;
    zookeeper
        .add_command(Box::new(FeedAnimal {
            name: "Leo".to_string(),
            food: "steak".to_string(),
        }))
        .await;
    zookeeper
        .add_command(Box::new(MoveAnimal {
            name: "Leo".to_string(),
            enclosure: "Savannah Exhibit".to_string(),
        }))
        .await;
    zookeeper.execute_commands().await;
}
