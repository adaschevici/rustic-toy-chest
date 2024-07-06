trait AnimalFactory {
    fn create_animal(&self) -> Box<dyn Animal>;
}

struct LionFactory;
struct GiraffeFactory;

impl AnimalFactory for LionFactory {
    fn create_animal(&self) -> Box<dyn Animal> {
        Box::new(Lion)
    }
}

impl AnimalFactory for GiraffeFactory {
    fn create_animal(&self) -> Box<dyn Animal> {
        Box::new(Giraffe)
    }
}

trait Animal {
    fn make_sound(&self);
}

struct Lion;
struct Giraffe;

impl Animal for Lion {
    fn make_sound(&self) {
        println!("Roar!");
    }
}

impl Animal for Giraffe {
    fn make_sound(&self) {
        println!("Neck snap!");
    }
}

pub async fn run_factory() {
    let lion_factory = LionFactory;
    let giraffe_factory = GiraffeFactory;
    let lion = lion_factory.create_animal();
    let giraffe = giraffe_factory.create_animal();
    lion.make_sound(); // Outputs: Roar!
    giraffe.make_sound(); // Outputs: Neck snap!
}
