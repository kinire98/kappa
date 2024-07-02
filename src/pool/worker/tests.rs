use std::{thread, time::Duration};

use crate::pool::worker::Worker;
#[test]
fn worker_not_blocked() {
    let worker = Worker::new();
    assert!(worker.is_free());
}

#[test]
fn worker_blocked() {
    let worker = Worker::new();
    worker.execute(|| {
        thread::sleep(Duration::from_secs(1));
    });
    thread::sleep(Duration::from_millis(15));
    assert!(!worker.is_free());
}
