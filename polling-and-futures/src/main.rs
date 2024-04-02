use std::future::Future;
use std::ptr::null;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

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
}
