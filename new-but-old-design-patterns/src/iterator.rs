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
