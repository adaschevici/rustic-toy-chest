use rayon::prelude::*;
use tracing::info;

fn parallel_map_filter(arr: &[i32]) -> Vec<i32> {
    arr.par_iter()
        .map(|&x| x * 2)
        .filter(|&x| x % 2 == 0)
        .collect()
}

pub async fn run_parsum() {
    let sum: i32 = (0..100).into_par_iter().sum();
    info!("Sum: {}", sum);
}

pub async fn run_parsum_map_filter() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = parallel_map_filter(&arr);
    info!("Result: {:?}", result);
}
