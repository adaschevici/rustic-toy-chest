use tracing::info;

macro_rules! animal_behaviour {
    ($animal:expr, $behaviour:expr) => {
        Box::pin(async {
            info!("The {} is {}", $animal, $behaviour);
        })
    };
}

pub async fn run_animal_behavior_macro() {
    animal_behaviour!("dog", "barking").await;
}
