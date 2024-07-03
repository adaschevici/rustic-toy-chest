use tracing::info;

trait AnimalState {
    fn on_enter(&self);
    fn on_exit(&self);
    fn behavior(&self);
}

struct Sleeping;
struct Eating;
struct Playing;

impl AnimalState for Sleeping {
    fn on_enter(&self) {
        info!("Entering sleeping state");
    }

    fn on_exit(&self) {
        info!("Exiting sleeping state");
    }

    fn behavior(&self) {
        info!("Sleeping");
    }
}

impl AnimalState for Eating {
    fn on_enter(&self) {
        info!("Entering eating state");
    }

    fn on_exit(&self) {
        info!("Exiting eating state");
    }

    fn behavior(&self) {
        info!("Eating");
    }
}

impl AnimalState for Playing {
    fn on_enter(&self) {
        info!("Entering playing state");
    }

    fn on_exit(&self) {
        info!("Exiting playing state");
    }

    fn behavior(&self) {
        info!("Playing");
    }
}

struct Animal {
    state: Box<dyn AnimalState>,
}

impl Animal {
    fn new(initial_state: Box<dyn AnimalState>) -> Self {
        initial_state.on_enter();
        Self {
            state: initial_state,
        }
    }

    fn change_state(&mut self, new_state: Box<dyn AnimalState>) {
        self.state.on_exit();
        self.state = new_state;
        self.state.on_enter();
    }

    fn exhibit_behavior(&self) {
        self.state.behavior();
    }
}

pub async fn run_state_pattern() {
    let sleeping_state = Box::new(Sleeping);
    let eating_state = Box::new(Eating);
    let playing_state = Box::new(Playing);
    let mut animal = Animal::new(sleeping_state);
    animal.exhibit_behavior();
    animal.change_state(eating_state);
    animal.exhibit_behavior();
    animal.change_state(playing_state);
    animal.exhibit_behavior();
    info!("Running state pattern example");
}
