use std::thread;

async fn run_threaded_ops() {
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
