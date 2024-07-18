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

async fn run_producer_chan(s: Sender<u32>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        info!("Hello from producer thread - pushing...!");
        for i in 0..1000 {
            s.send(i).expect("Unable to send");
        }
    })
}

async fn run_consumer_chan(r: Receiver<u32>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut i = 0;
        info!("Hello from consumer thread - popping...!");
        loop {
            if let Err(_) = r.recv() {
                info!("Consumer received {} messages", i);
                break;
            }
            i += 1;
        }
    })
}

pub async fn run_pub_sub_chan() {
    info!("Running pub sub with channels");
    let (s, r) = unbounded();

    for i in 1..5 {
        run_producer_chan(s.clone(), i);
    }
    drop(s);

    for i in 1..5 {
        run_consumer_chan(r.clone(), i);
    }

    info!("Pub sub with channels complete");
}
