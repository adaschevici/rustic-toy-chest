use async_trait::async_trait;

#[async_trait]
trait AnimalFactory {
    async fn create_animal(&self) -> Box<dyn Animal + Send + Sync>;
}

struct LionFactory;
struct GiraffeFactory;

#[async_trait]
impl AnimalFactory for LionFactory {
    async fn create_animal(&self) -> Box<dyn Animal + Send + Sync> {
        Box::new(Lion)
    }
}

#[async_trait]
impl AnimalFactory for GiraffeFactory {
    async fn create_animal(&self) -> Box<dyn Animal + Send + Sync> {
        Box::new(Giraffe)
    }
}

#[async_trait]
trait Animal {
    async fn make_sound(&self);
}

struct Lion;
struct Giraffe;

#[async_trait]
impl Animal for Lion {
    async fn make_sound(&self) {
        println!("Roar!");
    }
}

#[async_trait]
impl Animal for Giraffe {
    async fn make_sound(&self) {
        println!("Neck snap!");
    }
}

pub async fn run_factory() {
    let lion_factory = LionFactory;
    let giraffe_factory = GiraffeFactory;
    let lion = lion_factory.create_animal().await;
    let giraffe = giraffe_factory.create_animal().await;
    lion.make_sound().await; // Outputs: Roar!
    giraffe.make_sound().await; // Outputs: Neck snap!
}
