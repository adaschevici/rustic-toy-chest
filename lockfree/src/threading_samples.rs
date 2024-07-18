use crossbeam::atomic::AtomicCell;
use crossbeam::queue::ArrayQueue;
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

async fn run_producer(q: Arc<ArrayQueue<u32>>, num: u32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        info!("Hello from producer thread {} - pushing...!", num);
        for _ in 0..20 {
            q.push(num).expect("pushing failed");
        }
    })
}

async fn run_consumer(q: Arc<ArrayQueue<u32>>, num: u32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("Hello from producer thread {} - popping!", num);
        for _ in 0..20 {
            q.pop();
        }
    })
}

pub async fn run_pub_sub() {
    info!("Running pub sub");
    let q: Arc<ArrayQueue<u32>> = Arc::new(ArrayQueue::new(100));
    let mut producer_handles: Vec<thread::JoinHandle<()>> = Vec::new();
    let mut consumer_handles: Vec<thread::JoinHandle<()>> = Vec::new();

    for i in 1..10 {
        producer_handles.push(run_producer(q.clone(), i).await);
    }

    for i in 1..10 {
        consumer_handles.push(run_consumer(q.clone(), i).await);
    }

    for handle in producer_handles {
        handle.join().expect("Unable to join producer thread");
    }

    for handle in consumer_handles {
        handle.join().expect("Unable to join consumer thread");
    }

    info!("Pub sub complete");
}

// pub async fn run_pub_sub() {
//     info!("Running pub sub");
//     let q: Arc<ArrayQueue<u32>> = Arc::new(ArrayQueue::new(100));
//     let mut producer_handles: Vec<thread::JoinHandle<()>> = Vec::new();
//     let mut consumer_handles: Vec<thread::JoinHandle<()>> = Vec::new();
//     for i in 1..10 {
//         producer_handles.push(run_producer(q.clone(), i).await);
//     }
//
//     let mut value = q.pop();
//     for i in 1..10 {
//         consumer_handles.push(thread::spawn(move || {
//             info!("Hello from consumer thread {} - popping...!", i);
//             for _ in 0..20 {
//                 value.expect("popping failed");
//             }
//         }));
//     }
//
//     producer_handles.into_iter().for_each(|handle| {
//         handle.join().expect("Unable to join producer thread");
//     });
//
//     consumer_handles.into_iter().for_each(|handle| {
//         handle.join().expect("Unable to join consumer thread");
//     });
//
//     info!("Pub sub complete");
// }
