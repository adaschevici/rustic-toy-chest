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

pub async fn run_command() {
    let command = AddAnimal {
        name: "Leo".to_string(),
        species: "Lion".to_string(),
    };
    command.execute().await;
}
