use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // code
        }

        ThreadPool { threads }
    }

    pub fn execute<F>(f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
