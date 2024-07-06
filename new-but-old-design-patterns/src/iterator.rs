struct Animal {
    species: String,
    name: String,
}

struct Zoo {
    animals: Vec<Animal>,
}

impl Zoo {
    fn new() -> Zoo {
        Zoo {
            animals: Vec::new(),
        }
    }

    fn add_animal(&mut self, animal: Animal) {
        self.animals.push(animal);
    }
}

impl Iterator for Zoo {
    type Item = Animal;
    fn next(&mut self) -> Option<Self::Item> {
        self.animals.pop()
    }
}

pub async fn run_iterator() {
    let mut zoo = Zoo::new();
    zoo.add_animal(Animal {
        species: "Lion".to_string(),
        name: "Leo".to_string(),
    });
    zoo.add_animal(Animal {
        species: "Tiger".to_string(),
        name: "Tigger".to_string(),
    });
    while let Some(animal) = zoo.next() {
        println!("We have a {} named {}", animal.species, animal.name);
    }
}
