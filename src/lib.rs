use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

pub struct PoolCreationError;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            let new_thread = 
                thread::spawn(|| {
                });
            threads.push(new_thread);
        }

        ThreadPool {
            threads
        }
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError);
        }

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            let new_thread = 
                thread::spawn(|| {
                });
            threads.push(new_thread);
        }

        Ok(ThreadPool { threads })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, {

        }
}

