use std::collections::VecDeque;

use worker::Worker;

mod worker;
#[allow(dead_code)]
pub struct Kappa {
    max_threads: usize,
    workers: Vec<Worker>,
    queue: VecDeque<Box<dyn FnOnce() + 'static + Send>>,
}

#[allow(dead_code)]
impl Kappa {
    pub fn new(max_threads: usize) -> Kappa {
        let mut workers = vec![];
        for _ in 0..=max_threads {
            workers.push(Worker::new());
        }
        Kappa {
            max_threads,
            workers,
            queue: VecDeque::new(),
        }
    }
    pub fn execute<F>(&mut self, process: F)
    where
        F: FnOnce() + 'static + Send,
    {
        self.queue.push_back(Box::new(process));
    }
}

#[cfg(test)]
mod tests;
