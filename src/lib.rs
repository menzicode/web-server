use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

pub struct PoolCreationError;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool {
            workers
        }
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError);
        }

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        Ok(ThreadPool {
            workers
        })
    }


    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, {

        }
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        Worker{id, thread: thread::spawn(|| {})}
    }
}
