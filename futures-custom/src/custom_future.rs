use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread;
use std::time::Duration;
use tracing::info;

struct MyWaker {
    waker: Mutex<Option<Waker>>,
}

impl Wake for MyWaker {
    fn wake(self: Arc<Self>) {
        info!("Waking up...");
        let waker = self.waker.lock().unwrap().take();
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

struct MyFuture {
    state: i32,
    waker: Arc<MyWaker>,
}

impl Future for MyFuture {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.state == 0 {
            self.state += 1;
            let mut waker = self.waker.waker.lock().unwrap();
            *waker = Some(cx.waker().clone());
            info!("Polling...");
            Poll::Pending
        } else {
            Poll::Ready(self.state)
        }
    }
}

pub async fn use_custom_future() {
    let waker = Arc::new(MyWaker {
        waker: Mutex::new(None),
    });
    let future = MyFuture {
        state: 0,
        waker: waker.clone(),
    };
    let result_future = async {
        let result = future.await;
        info!("Result: {}", result);
    };

    // Spawn a thread to wake up the future after a delay
    let waker_clone = waker.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        waker_clone.wake_by_ref();
    });

    // result_future.await;
    info!("Result: {:?}", result_future.await);
}
