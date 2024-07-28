use rayon::prelude::*;
use std::time::Instant;
use tracing::info;

fn parallel_matrix_multiply(a: &[Vec<i32>], b: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let n = a.len();
    let m = b[0].len();
    let p = b.len();
    assert_eq!(a[0].len(), p);
    let mut result = vec![vec![0; m]; n];
    result.par_iter_mut().enumerate().for_each(|(i, row)| {
        for j in 0..m {
            row[j] = (0..p).map(|k| a[i][k] * b[k][j]).sum();
        }
    });
    result
}

pub async fn run_matrix_mul() {
    let n = 500;
    let m = 500;
    let p = 500;
    let a: Vec<Vec<i32>> = (0..n)
        .map(|_| (0..p).map(|_| rand::random::<i32>() % 10).collect())
        .collect();
    let b: Vec<Vec<i32>> = (0..p)
        .map(|_| (0..m).map(|_| rand::random::<i32>() % 10).collect())
        .collect();
    let start = Instant::now();
    let result = parallel_matrix_multiply(&a, &b);
    let duration = start.elapsed();
    info!("Time taken: {:?}", duration);
    info!("Result (first row): {:?}", result[0]);
}
