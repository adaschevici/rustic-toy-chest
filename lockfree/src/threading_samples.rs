use crossbeam::atomic::AtomicCell;
use crossbeam::queue::ArrayQueue;
use crossbeam::sync::WaitGroup;
use rand::{self, Rng};
use std::sync::Arc;
use std::thread;
use tokio::sync::mpsc::{
    unbounded_channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
};
use tokio::sync::Mutex;
use tokio::task;
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

async fn run_producer_chan(s: Sender<u32>, num: u32) -> task::JoinHandle<()> {
    task::spawn(async move {
        info!("Hello from producer thread - pushing...!");
        for i in 0..1000 {
            s.send(i).expect("Unable to send");
        }
    })
}

async fn run_consumer_chan(r: Arc<Mutex<Receiver<u32>>>, num: u32) -> task::JoinHandle<()> {
    task::spawn(async move {
        let mut i = 0;
        println!("Hello from consumer thread - popping...!");
        loop {
            let message = {
                let mut receiver = r.lock().await;
                receiver.recv().await
            };
            match message {
                Some(_) => {
                    i += 1;
                }
                None => {
                    println!("Consumer received {} messages", i);
                    break;
                }
            }
        }
    })
}

pub async fn run_pub_sub_chan() {
    info!("Running pub sub with channels");
    let (s, r): (Sender<u32>, Receiver<u32>) = unbounded_channel();
    let r = Arc::new(Mutex::new(r));

    let mut producer_handles = Vec::new();
    for i in 1..5 {
        let handle = run_producer_chan(s.clone(), i).await;
        producer_handles.push(handle);
    }

    drop(s);

    let mut consumer_handles = Vec::new();
    for i in 1..5 {
        let rx = Arc::clone(&r);
        let handle = run_consumer_chan(rx, i).await;
        consumer_handles.push(handle);
    }

    for handle in producer_handles {
        handle.await.unwrap();
    }

    for handle in consumer_handles {
        handle.await.unwrap();
    }

    info!("Pub sub with channels complete");
}

async fn do_work(thread_num: i32) {
    let num = rand::thread_rng().gen_range(100..500);
    thread::sleep(std::time::Duration::from_millis(num));
    let mut sum = 0;
    for i in 0..10 {
        sum += sum + num * i;
    }
    println!(
        "thread {} calculated sum: {}, num: {}",
        thread_num, sum, num
    );
    thread::sleep(std::time::Duration::from_millis(num));
}

pub async fn run_waitgroup() {
    let wg = WaitGroup::new();
    for i in 0..50 {
        let wg_clone = wg.clone();
        tokio::spawn(async move {
            do_work(i).await;
            drop(wg_clone);
        });
    }
    wg.wait();
}
