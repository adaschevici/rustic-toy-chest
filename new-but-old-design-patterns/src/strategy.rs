use async_trait::async_trait;

#[async_trait]
trait CareRoutine {
    async fn execute(&self);
}

struct FeedingRoutine;
struct CleaningRoutine;
struct EntertainmentRoutine;

#[async_trait]
impl CareRoutine for FeedingRoutine {
    async fn execute(&self) {
        println!("Feeding the animals with their favorite snacks.");
    }
}

#[async_trait]
impl CareRoutine for CleaningRoutine {
    async fn execute(&self) {
        println!("Cleaning the animal enclosures for a fresh environment.");
    }
}

#[async_trait]
impl CareRoutine for EntertainmentRoutine {
    async fn execute(&self) {
        println!("Playing interactive games with the animals.");
    }
}

struct Animal {
    name: String,
    care_routine: Box<dyn CareRoutine + Send + Sync>,
}

impl Animal {
    async fn new(name: String, care_routine: Box<dyn CareRoutine + Send + Sync>) -> Animal {
        Animal { name, care_routine }
    }

    async fn perform_care(&self) {
        println!("{}: ", self.name);
        self.care_routine.execute().await;
    }

    async fn set_care_routine(&mut self, new_routine: Box<dyn CareRoutine + Send + Sync>) {
        self.care_routine = new_routine;
    }
}

pub async fn run_strategy() {
    let feeding = Box::new(FeedingRoutine);
    let cleaning = Box::new(CleaningRoutine);
    let entertainment = Box::new(EntertainmentRoutine);
    let mut leo = Animal::new("Leo the Lion".to_string(), feeding).await;
    leo.perform_care().await;
    leo.set_care_routine(cleaning).await;
    leo.perform_care().await;
    leo.set_care_routine(entertainment).await;
    leo.perform_care().await;
}
