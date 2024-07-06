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

pub async fn run_factory() {
    let lion_factory = LionFactory;
    let lion = lion_factory.create_animal();
    lion.speak().await;
}
