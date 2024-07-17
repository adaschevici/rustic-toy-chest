use crossbeam_epoch::{self as epoch, Atomic, Owned, Shared};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

struct Node<T> {
    data: T,
    next: Atomic<Node<T>>,
}
impl<T> Node<T> {
    fn get_data(&self) -> &T {
        &self.data
    }
}
struct LockFreeQueue<T> {
    head: Atomic<Node<T>>,
    tail: Atomic<Node<T>>,
}

impl<T> LockFreeQueue<T> {
    fn new() -> Self {
        let node = Owned::new(Node {
            data: unsafe { std::mem::zeroed() }, // Placeholder for the head node
            next: Atomic::null(),
        });
        let guard = epoch::pin();
        let node = node.into_shared(&guard);
        Self {
            head: Atomic::from(node),
            tail: Atomic::from(node),
        }
    }

    fn enqueue(&self, data: T) {
        let node = Owned::new(Node {
            data,
            next: Atomic::null(),
        });
        let guard = epoch::pin();
        let node = node.into_shared(&guard);
        loop {
            let tail = self.tail.load(Ordering::Acquire, &guard);
            let next = unsafe { tail.deref() }.next.load(Ordering::Acquire, &guard);
            if next.is_null() {
                if unsafe { tail.deref() }
                    .next
                    .compare_exchange(next, node, Ordering::Release, Ordering::Acquire, &guard)
                    .is_ok()
                {
                    self.tail
                        .compare_exchange(tail, node, Ordering::Release, Ordering::Acquire, &guard)
                        .unwrap();
                    break;
                }
            } else {
                self.tail
                    .compare_exchange(tail, next, Ordering::Release, Ordering::Acquire, &guard)
                    .unwrap();
            }
        }
    }

    fn dequeue(&self) -> Option<T> {
        let guard = epoch::pin();
        loop {
            let head = self.head.load(Ordering::Acquire, &guard);
            let tail = self.tail.load(Ordering::Acquire, &guard);
            let next = unsafe { head.deref() }.next.load(Ordering::Acquire, &guard);
            if head == tail {
                if next.is_null() {
                    return None;
                }
                self.tail
                    .compare_exchange(tail, next, Ordering::Release, Ordering::Acquire, &guard)
                    .unwrap();
            } else {
                if let Some(next) = unsafe { next.as_ref() } {
                    let next = next.next.load(Ordering::Acquire, &guard);
                    if self
                        .head
                        .compare_exchange(head, next, Ordering::Release, Ordering::Acquire, &guard)
                        .is_ok()
                    {
                        unsafe {
                            guard.defer_destroy(head);
                        }
                        // return Some(unsafe { std::ptr::read(next.deref()) });
                        let return_data = unsafe { next.deref().get_data() };
                        return Some(unsafe { std::ptr::read(return_data) });
                    }
                }
            }
        }
    }
}

struct MutexQueue<T> {
    queue: Mutex<Vec<T>>,
}

impl<T> MutexQueue<T> {
    fn new() -> Self {
        Self {
            queue: Mutex::new(Vec::new()),
        }
    }

    fn enqueue(&self, data: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(data);
    }

    fn dequeue(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        if !queue.is_empty() {
            Some(queue.remove(0))
        } else {
            None
        }
    }
}

pub async fn run_non_locking_queue_ops() {
    const NUM_THREADS: usize = 4;
    const NUM_OPERATIONS: usize = 1000000;

    // Lock-free queue benchmark
    let lock_free_queue = Arc::new(LockFreeQueue::new());
    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..NUM_THREADS {
        let queue = Arc::clone(&lock_free_queue);
        handles.push(thread::spawn(move || {
            for i in 0..NUM_OPERATIONS {
                queue.enqueue(i);
                queue.dequeue();
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Lock-Free Queue Time: {:?}", duration);

    // Mutex-based queue benchmark
    let mutex_queue = Arc::new(MutexQueue::new());
    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..NUM_THREADS {
        let queue = Arc::clone(&mutex_queue);
        handles.push(thread::spawn(move || {
            for i in 0..NUM_OPERATIONS {
                queue.enqueue(i);
                queue.dequeue();
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Mutex Queue Time: {:?}", duration);
}
