use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;

#[async_trait]
trait AnimalState: Send + Sync {
    async fn on_enter(&self);
    async fn on_exit(&self);
    async fn behavior(&self);
}

struct Sleeping;
struct Eating;
struct Playing;

#[async_trait]
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

#[async_trait]
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

#[async_trait]
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
    state: Arc<dyn AnimalState>,
}

impl Animal {
    fn new(initial_state: Arc<dyn AnimalState>) -> Animal {
        initial_state.on_enter();
        Animal {
            state: initial_state,
        }
    }

    async fn change_state(&mut self, new_state: Arc<dyn AnimalState>) {
        self.state.on_exit().await;
        self.state = new_state;
        self.state.on_enter().await;
    }

    async fn exhibit_behavior(&self) {
        self.state.behavior().await;
    }
}

pub async fn run_state_pattern() {
    let sleeping_state = Arc::new(Sleeping);
    let eating_state = Arc::new(Eating);
    let playing_state = Arc::new(Playing);
    let mut animal = Animal::new(sleeping_state);
    animal.exhibit_behavior().await;
    animal.change_state(eating_state).await;
    animal.exhibit_behavior().await;
    animal.change_state(playing_state).await;
    animal.exhibit_behavior();
    info!("Running state pattern example");
}
