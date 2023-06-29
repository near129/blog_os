use core::sync::atomic::{AtomicU64, Ordering};
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::task::AtomicWaker;

use futures_util::Future;

static CNT: AtomicU64 = AtomicU64::new(0);
static WAKER: AtomicWaker = AtomicWaker::new();

pub fn add_cnt() {
    CNT.fetch_add(1, Ordering::Relaxed);
    WAKER.wake();
}

pub struct Timer {
    _private: (),
}

impl Timer {
    pub fn new() -> Self {
        Timer { _private: () }
    }

    pub fn get_cnt(&self) -> u64 {
        CNT.load(Ordering::Relaxed)
    }

    pub fn sleep(&self, duration: u64) -> SleepFuture {
        let start_cnt = self.get_cnt();
        SleepFuture {
            start_cnt,
            duration,
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn sleep(duration: u64) {
    Timer::new().sleep(duration).await
}
pub struct SleepFuture {
    start_cnt: u64,
    duration: u64,
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        WAKER.register(cx.waker());
        if CNT.load(Ordering::Relaxed) - self.start_cnt >= self.duration {
            WAKER.take();
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
