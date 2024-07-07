use tracing::info;

macro_rules! animal_behaviour_expr {
    ($animal:expr, $behaviour:expr) => {
        Box::pin(async {
            info!("The {} is {}", $animal, $behaviour);
        })
    };
}

macro_rules! animal_behaviour {
    ($animal:ident, $behaviour:expr) => {
        fn $animal() {
            info!("The {} is {}", stringify!($animal), $behaviour);
        }
    };
}

pub async fn run_animal_behavior_macro() {
    animal_behaviour_expr!("dog", "barking").await;
    animal_behaviour!(cat, "meowing");
    cat();
}
