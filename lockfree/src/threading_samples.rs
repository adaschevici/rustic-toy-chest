use crossbeam::atomic::AtomicCell;
use std::sync::Arc;
use std::thread;
use tracing::info;

pub async fn run_threaded_ops() {
    let mut handles = vec![];
    for i in 0..10 {
        let handle = thread::spawn(move || {
            info!("Thread number: {}", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

async fn run_thread(val: Arc<AtomicCell<u32>>, num: u32, store: bool) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        if store {
            val.fetch_add(1);
        }
        info!("Hello from thread {}! value: {}", num, val.load());
    })
}

pub async fn run_threaded_with_critical_section() {
    info!("Running threaded with critical section");
    let val: Arc<AtomicCell<u32>> = Arc::new(AtomicCell::new(42));
    let mut thread_handles_ac: Vec<thread::JoinHandle<()>> = Vec::new();
    for i in 1..10 {
        thread_handles_ac.push(run_thread(val.clone(), i, i % 2 == 0).await);
    }

    thread_handles_ac.into_iter().for_each(|handle| {
        handle.join().expect("Unable to join thread");
    });

    info!("Final value: {}", val.load());
    info!("Threaded with critical section complete");
}
