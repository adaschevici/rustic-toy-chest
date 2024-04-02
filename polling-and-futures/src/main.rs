use async_std::task;
use std::cell::Cell;
use std::future::Future;
use std::ptr::null;
use std::rc::Rc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::time::Duration;

fn wake(_data: *const ()) {}
fn noop(_data: *const ()) {}

static VTABLE: RawWakerVTable =
    RawWakerVTable::new(|data| RawWaker::new(data, &VTABLE), wake, wake, noop);

pub struct Resource {
    value: i32,
    elapsed: Rc<Cell<u64>>,
}

impl Resource {
    pub fn result(&self) -> i32 {
        self.value
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("{:>4} drop", self.elapsed.get());
    }
}

fn main() {
    let task = async { 13 };
    let waker = RawWaker::new(null(), &VTABLE);

    let waker = unsafe { Waker::from_raw(waker) };

    let mut cx = Context::from_waker(&waker);
    let mut task = Box::pin(task);

    match task.as_mut().poll(&mut cx) {
        Poll::Ready(val) => println!("Ready: {}", val),
        Poll::Pending => println!("Pending"),
    }

    let task2 = async {
        task::sleep(Duration::from_secs(1)).await;
        13
    };

    let mut elapsed = 0;
    let mut cx = Context::from_waker(&waker);
    let mut task2 = Box::pin(task2);

    loop {
        match task2.as_mut().poll(&mut cx) {
            Poll::Ready(value) => break println!("{elapsed:>4} ready {value:?}"),
            Poll::Pending => println!("{elapsed:>4} pending"),
        }
        std::thread::sleep(Duration::from_millis(300));
        elapsed += 300;
    }

    let elapsed = Rc::new(Cell::new(0));
    let copied = elapsed.clone();

    let task3 = async {
        let resource = Resource {
            value: 13,
            elapsed: copied,
        };
        task::sleep(Duration::from_secs(1)).await;
        resource.result()
    };
    let mut cx = Context::from_waker(&waker);
    let mut task3 = Box::pin(task3);
    loop {
        match task3.as_mut().poll(&mut cx) {
            Poll::Ready(value) => break println!("{:>4} ready {value:?}", elapsed.get()),
            Poll::Pending => println!("{:>4} pending", elapsed.get()),
        }
        std::thread::sleep(Duration::from_millis(300));
        elapsed.set(elapsed.get() + 300);
    }
}
