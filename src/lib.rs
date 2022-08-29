use std::{task::{Waker, Poll, Context}, time::Duration};

use tokio_util::time::{DelayQueue, delay_queue::{Expired, Key}};

pub struct PatientQueue<T> {
    inner: DelayQueue<T>,
    waker: Option<Waker>,
}

impl<T> PatientQueue<T> {
    pub fn insert(&mut self, value: T, timeout: Duration) -> Key {
        if let Some(w) = self.waker.take() {
            w.wake()
        }

        self.inner.insert(value, timeout)
    }

    pub fn poll_expired(&mut self, cx: &mut Context<'_>) -> Poll<Expired<T>> {
        match self.inner.poll_expired(cx) {
            Poll::Ready(Some(entry)) => Poll::Ready(entry),
            Poll::Ready(None) => {
                self.waker = Some(cx.waker().clone());
                Poll::Pending
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
