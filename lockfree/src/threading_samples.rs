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

pub async fn run_threaded_with_critical_section() {
    let val: Arc<AtomicCell<u32>> = Arc::new(AtomicCell::new(0));
    thread::spawn(move || {
        for _ in 0..1000000 {
            val.fetch_add(1);
        }
    });
}
