use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{io, io::Write};
use tracing::info;

fn thread_task(id: usize, max_count: u32, pb: Arc<ProgressBar>) -> usize {
    let mut rng = rand::thread_rng();
    let delay = rng.gen_range(1..=50);

    for _ in 0..max_count {
        pb.inc(1);
        thread::sleep(Duration::from_millis(delay as u64));
    }
    pb.finish_with_message("done");

    id
}

fn run_race(
    winner: Arc<Mutex<Option<usize>>>,
    num_threads: usize,
    max_count: u32,
    mp: Arc<MultiProgress>,
) -> usize {
    (0..num_threads).into_par_iter().for_each(|id| {
        let pb = mp.add(ProgressBar::new(max_count as u64));
        pb.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "[{{elapsed_precise}}] {{bar:40.cyan/blue}} Thread {}: {{pos}}/{{len}} {{msg}}",
                    id
                ))
                .unwrap()
                .progress_chars("##-"),
        );

        let pb = Arc::new(pb);
        let result = thread_task(id, max_count, pb);

        let mut winner_guard = winner.lock().unwrap();
        if winner_guard.is_none() {
            *winner_guard = Some(result);
        }
    });

    winner.lock().unwrap().unwrap()
}

pub async fn run_race_event() {
    let num_threads = 5;
    let max_count = 100;

    info!("Welcome to the Thread Race Game!");
    info!(
        "There are {} threads racing to complete their task.",
        num_threads
    );
    info!(
        "Place your bet on which thread will win (0-{}): ",
        num_threads - 1
    );
    io::stdout().flush().unwrap();

    let mut bet = String::new();
    io::stdin()
        .read_line(&mut bet)
        .expect("Failed to read line");
    let bet: usize = bet.trim().parse().expect("Please enter a valid number");

    info!("Starting the race...");

    let winner: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(None));
    let mp = Arc::new(MultiProgress::new());

    let winner = run_race(winner, num_threads, max_count, mp);

    info!("Thread {} wins the race!", winner);
    if winner == bet {
        info!("Congratulations! Your bet was correct!");
    } else {
        info!("Sorry, better luck next time.");
    }
}
