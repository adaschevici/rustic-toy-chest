use rayon::prelude::*;
use tracing::info;

async fn parallel_map_filter(arr: &[i32]) -> Vec<i32> {
    arr.par_iter()
        .map(|&x| x * 2)
        .filter(|&x| x % 2 == 0)
        .collect()
}

async fn parallel_sort(arr: &mut [i32]) {
    arr.par_sort();
}

pub async fn run_parsum() {
    let sum: i32 = (0..100).into_par_iter().sum();
    info!("Sum: {}", sum);
}

pub async fn run_parsum_map_filter() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = parallel_map_filter(&arr).await;
    info!("Result: {:?}", result);
}

pub async fn run_parsort() {
    let mut data = vec![10, 5, 8, 1, 7, 6, 3, 2, 4, 9];
    parallel_sort(&mut data).await;
    info!("The sorted array is: {:?}", data);
}
