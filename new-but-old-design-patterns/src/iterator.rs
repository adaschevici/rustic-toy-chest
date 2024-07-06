use async_trait::async_trait;

struct Animal {
    species: String,
    name: String,
}

struct Zoo {
    animals: Vec<Animal>,
}
#[async_trait]
trait AsyncIterator {
    type Item;

    async fn next(&mut self) -> Option<Self::Item>;
}
impl Zoo {
    async fn new() -> Zoo {
        Zoo {
            animals: Vec::new(),
        }
    }

    async fn add_animal(&mut self, animal: Animal) {
        self.animals.push(animal);
    }
}

#[async_trait]
impl AsyncIterator for Zoo {
    type Item = Animal;
    async fn next(&mut self) -> Option<Self::Item> {
        self.animals.pop()
    }
}

pub async fn run_iterator() {
    let mut zoo = Zoo::new().await;
    zoo.add_animal(Animal {
        species: "Lion".to_string(),
        name: "Leo".to_string(),
    })
    .await;
    zoo.add_animal(Animal {
        species: "Tiger".to_string(),
        name: "Tigger".to_string(),
    })
    .await;
    while let Some(animal) = zoo.next().await {
        println!("We have a {} named {}", animal.species, animal.name);
    }
}
