use crossbeam::epoch::{self as epoch, Atomic, Owned};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

struct LockFreeStack<T> {
    head: Atomic<Node<T>>,
}

struct Node<T> {
    data: T,
    next: Atomic<Node<T>>,
}

impl<T> LockFreeStack<T> {
    fn new() -> Self {
        Self {
            head: Atomic::null(),
        }
    }

    fn push(&self, data: T) {
        let guard = epoch::pin();
        let mut node = Owned::new(Node {
            data,
            next: Atomic::null(),
        });

        loop {
            let head = self.head.load(Ordering::Acquire, &guard);
            node.next.store(head, Ordering::Relaxed);
            match self
                .head
                .compare_and_set(head, node, Ordering::Release, &guard)
            {
                Ok(_) => break,
                Err(e) => node = e.new,
            }
        }
    }

    fn pop(&self) -> Option<T> {
        let guard = epoch::pin();
        loop {
            let head = self.head.load(Ordering::Acquire, &guard);
            match unsafe { head.as_ref() } {
                Some(h) => {
                    let next = h.next.load(Ordering::Relaxed, &guard);
                    match self
                        .head
                        .compare_and_set(head, next, Ordering::Release, &guard)
                    {
                        Ok(_) => {
                            unsafe {
                                guard.defer_destroy(head);
                            }
                            return Some(unsafe { std::ptr::read(&h.data) });
                        }
                        Err(_) => continue,
                    }
                }
                None => return None,
            }
        }
    }
}

struct MutexStack<T> {
    head: Mutex<Option<Box<Node<T>>>>,
}

impl<T> MutexStack<T> {
    fn new() -> Self {
        Self {
            head: Mutex::new(None),
        }
    }

    fn push(&self, data: T) {
        let mut head = self.head.lock().unwrap();
        let new_node = Box::new(Node {
            data,
            next: Atomic::null(),
        });
        *head = Some(new_node);
    }

    fn pop(&self) -> Option<T> {
        let mut head = self.head.lock().unwrap();
        head.take().map(|node| node.data)
    }
}

pub async fn run_non_locking_stack_ops() {
    const NUM_THREADS: usize = 4;
    const NUM_OPERATIONS: usize = 1000000;

    // Lock-free stack benchmark
    let lock_free_stack = Arc::new(LockFreeStack::new());
    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..NUM_THREADS {
        let stack = Arc::clone(&lock_free_stack);
        handles.push(thread::spawn(move || {
            for i in 0..NUM_OPERATIONS {
                stack.push(i);
                stack.pop();
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Lock-Free Stack Time: {:?}", duration);

    // Mutex-based stack benchmark
    let mutex_stack = Arc::new(MutexStack::new());
    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..NUM_THREADS {
        let stack = Arc::clone(&mutex_stack);
        handles.push(thread::spawn(move || {
            for i in 0..NUM_OPERATIONS {
                stack.push(i);
                stack.pop();
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Mutex Stack Time: {:?}", duration);
}
