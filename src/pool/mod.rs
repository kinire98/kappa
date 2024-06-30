use std::collections::VecDeque;

use worker::Worker;

mod worker;
#[allow(dead_code)]
pub struct Kappa {
    max_threads: usize,
    workers: Vec<Worker>,
    queue: VecDeque<Box<dyn FnOnce() + 'static + Send>>,
}

impl Kappa {
    pub fn new(max_threads: usize) -> Kappa {
        Kappa {
            max_threads,
            workers: vec![],
            queue: VecDeque::new(),
        }
    }
    pub fn execute<F>(process: F)
    where
        F: FnOnce() + 'static + Send,
    {
    }
}

#[cfg(test)]
mod tests;
