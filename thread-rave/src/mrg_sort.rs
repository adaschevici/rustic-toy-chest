use rayon::prelude::*;
use std::time::Instant;
use tracing::info;

fn parallel_merge_sort<T: Ord + Send + Clone + Sync>(mut arr: Vec<T>) -> Vec<T> {
    let len = arr.len();
    if len <= 1 {
        return arr;
    }
    let mid = len / 2;
    let (left, right) = arr.split_at(mid);

    // Perform the sorting in parallel on the left and right halves
    let (left_sorted, right_sorted) = rayon::join(
        || parallel_merge_sort(left.to_vec()),
        || parallel_merge_sort(right.to_vec()),
    );

    // Merge the sorted halves
    merge(left_sorted, right_sorted)
}

fn merge<T: Ord + Clone>(left: Vec<T>, right: Vec<T>) -> Vec<T> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut left_iter = left.into_iter();
    let mut right_iter = right.into_iter();
    let mut left_next = left_iter.next();
    let mut right_next = right_iter.next();

    loop {
        match (left_next.clone(), right_next.clone()) {
            (Some(l), Some(r)) => {
                if l <= r {
                    merged.push(l);
                    left_next = left_iter.next();
                } else {
                    merged.push(r);
                    right_next = right_iter.next();
                }
            }
            (Some(l), None) => {
                merged.push(l);
                merged.extend(left_iter);
                break;
            }
            (None, Some(r)) => {
                merged.push(r);
                merged.extend(right_iter);
                break;
            }
            (None, None) => break,
        }
    }
    merged
}

async fn run_mrg_sort() {
    let data = (0..1_000_000)
        .map(|_| rand::random::<i32>())
        .collect::<Vec<_>>();
    let start = Instant::now();
    let result = parallel_merge_sort(data);
    let duration = start.elapsed();
    info!("Time taken: {:?}", duration);
    info!("Result (first 10): {:?}", &result[..10]);
}
