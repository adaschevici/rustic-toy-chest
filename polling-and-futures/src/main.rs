use async_std::task;
use std::future::Future;
use std::ptr::null;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::time::Duration;

fn wake(_data: *const ()) {}
fn noop(_data: *const ()) {}

static VTABLE: RawWakerVTable =
    RawWakerVTable::new(|data| RawWaker::new(data, &VTABLE), wake, wake, noop);

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
}
