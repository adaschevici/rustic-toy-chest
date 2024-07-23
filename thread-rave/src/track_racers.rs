use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

async fn thread_task(id: usize, max_count: u32, pb: Arc<ProgressBar>) -> usize {
    let mut rng = rand::thread_rng();
    let delay = rng.gen_range(1..=50);

    for _ in 0..max_count {
        pb.inc(1);
        thread::sleep(Duration::from_millis(delay as u64));
    }
    pb.finish_with_message("done");

    id
}
