use std::sync::mpsc::{self, Receiver, RecvError, Sender};
pub struct Worker {
    sender: Sender<Box<dyn FnOnce() + 'static + Send>>,
    status: Receiver<Result<bool, ()>>,
}
impl Worker {
    pub fn new() -> Worker {
        let (fn_tx, fn_rx) = mpsc::channel();
        let (status_tx, status_rx) = mpsc::channel();
        std::thread::spawn(move || {
            status_tx.send(Ok(true)).unwrap();
            loop {
                status_tx.send(Ok(true)).unwrap();
                let rec_fn: Result<Box<dyn FnOnce() + 'static + Send>, RecvError> = fn_rx.recv();
                if let Err(_) = rec_fn {
                    status_tx.send(Err(())).expect("Irrelevant");
                    panic!("The communication channel has hung up. Create a new worker");
                }
                status_tx.send(Ok(false)).unwrap();
                rec_fn.unwrap()();
                status_tx.send(Ok(true)).unwrap();
            }
        });
        let _ = status_rx.recv();
        Worker {
            sender: fn_tx,
            status: status_rx,
        }
    }
    //If true assume is up
    pub fn is_free(&self) -> bool {
        let values = self.status.try_iter();
        println!("{:?}", values);
        values
            .last()
            .expect("A value is sent when creating the worker")
            .expect("Temp")
    }
    pub fn is_up(&self) -> Result<bool, ()> {
        self.status.try_iter().last().expect("Temp")
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
#[cfg(test)]
mod tests;
