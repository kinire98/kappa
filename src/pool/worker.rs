use std::sync::mpsc::{self, Receiver, Sender};

pub struct Worker {
    sender: Sender<Box<dyn FnOnce() + 'static + Send>>,
    status: Receiver<bool>,
}
impl Worker {
    pub fn new() -> Worker {
        let (fn_tx, fn_rx) = mpsc::channel();
        let (status_tx, status_rx) = mpsc::channel();
        std::thread::spawn(move || {
            status_tx.send(true).unwrap();
            loop {
                let rec_fn: Box<dyn FnOnce() + 'static + Send> = fn_rx.recv().unwrap();
                status_tx.send(false).unwrap();
                rec_fn();
                status_tx.send(true).unwrap();
            }
        });
        Worker {
            sender: fn_tx,
            status: status_rx,
        }
    }
    pub fn is_free(&self) -> bool {
        self.status
            .try_iter()
            .last()
            .expect("A value is sent when creating the worker")
    }
    pub fn execute<F>(&self, process: F)
    where
        F: FnOnce() + 'static + Send,
    {
        self.sender
            .send(Box::new(process))
            .expect("Return the error, in the future");
    }
}
