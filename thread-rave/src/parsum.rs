use rayon::prelude::*;
use tracing::info;

pub async fn run_parsum() {
    let sum: i32 = (0..100).into_par_iter().sum();
    info!("Sum: {}", sum);
}
