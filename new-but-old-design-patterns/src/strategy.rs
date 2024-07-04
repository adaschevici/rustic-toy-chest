trait CareRoutine {
    fn execute(&self);
}

struct FeedingRoutine;
struct CleaningRoutine;
struct EntertainmentRoutine;

impl CareRoutine for FeedingRoutine {
    fn execute(&self) {
        println!("Feeding the animals with their favorite snacks.");
    }
}
impl CareRoutine for CleaningRoutine {
    fn execute(&self) {
        println!("Cleaning the animal enclosures for a fresh environment.");
    }
}
impl CareRoutine for EntertainmentRoutine {
    fn execute(&self) {
        println!("Playing interactive games with the animals.");
    }
}

struct Animal {
    name: String,
    care_routine: Box<dyn CareRoutine>,
}

impl Animal {
    fn new(name: String, care_routine: Box<dyn CareRoutine>) -> Animal {
        Animal { name, care_routine }
    }

    fn perform_care(&self) {
        println!("{}: ", self.name);
        self.care_routine.execute();
    }

    fn set_care_routine(&mut self, new_routine: Box<dyn CareRoutine>) {
        self.care_routine = new_routine;
    }
}

pub async fn run_strategy() {
    let feeding = Box::new(FeedingRoutine);
    let cleaning = Box::new(CleaningRoutine);
    let entertainment = Box::new(EntertainmentRoutine);
    let mut leo = Animal::new("Leo the Lion".to_string(), feeding);
    leo.perform_care();
    leo.set_care_routine(cleaning);
    leo.perform_care();
    leo.set_care_routine(entertainment);
    leo.perform_care();
}
