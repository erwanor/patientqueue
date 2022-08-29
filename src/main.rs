use futures::{future::poll_fn, StreamExt};
use patientqueue::PatientQueue;
use tokio;
use tokio_util::time::DelayQueue;

async fn dq_processor(mut dq: DelayQueue<usize>) {
    loop {
        let entry = poll_fn(|cx| dq.poll_expired(cx)).await;
        println!("found entry: {}", entry.unwrap().into_inner());
    }
}

async fn q_processor(mut q: PatientQueue<usize>) {
    loop {
        let entry = poll_fn(|cx| q.poll_expired(cx)).await;
        println!("found entry: {}", entry.into_inner());
    }
}

#[tokio::main]
async fn main() {
    let mut q: PatientQueue<usize> = PatientQueue::default();
    let mut dq: DelayQueue<usize> = DelayQueue::new();
    dq.insert(1000usize, std::time::Duration::ZERO);
    // q.insert(1000usize, std::time::Duration::ZERO);
    tokio::spawn(dq_processor(dq)).await;
    // tokio::spawn(q_processor(q)).await;
}
