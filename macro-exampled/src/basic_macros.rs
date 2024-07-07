macro_rules! animal_behaviour {
    ($animal:expr, $behaviour:expr) => {
        Box::pin(async {
            info!("The {} is {}", $animal, $behaviour);
        })
    };
}
