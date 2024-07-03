use tracing::info;

trait AnimalState {
    async fn on_enter(&self);
    async fn on_exit(&self);
    async fn behavior(&self);
}

struct Sleeping;
struct Eating;
struct Playing;

impl AnimalState for Sleeping {
    async fn on_enter(&self) {
        info!("Entering sleeping state");
    }

    async fn on_exit(&self) {
        info!("Exiting sleeping state");
    }

    async fn behavior(&self) {
        info!("Sleeping");
    }
}

impl AnimalState for Eating {
    async fn on_enter(&self) {
        info!("Entering eating state");
    }

    async fn on_exit(&self) {
        info!("Exiting eating state");
    }

    async fn behavior(&self) {
        info!("Eating");
    }
}

impl AnimalState for Playing {
    async fn on_enter(&self) {
        info!("Entering playing state");
    }

    async fn on_exit(&self) {
        info!("Exiting playing state");
    }

    async fn behavior(&self) {
        info!("Playing");
    }
}

struct Animal {
    state: Box<dyn AnimalState>,
}

impl Animal {
    fn new() -> Self {
        Self {
            state: Box::new(Sleeping),
        }
    }

    async fn change_state(&mut self, new_state: Box<dyn AnimalState>) {
        self.state.on_exit().await;
        self.state = new_state;
        self.state.on_enter().await;
    }

    async fn run(&self) {
        self.state.behavior().await;
    }
}

pub async fn run_state_pattern() {
    info!("Running state pattern example");
}
