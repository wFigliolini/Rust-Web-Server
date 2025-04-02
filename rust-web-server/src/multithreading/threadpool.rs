use ::std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    dispatcher: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size as usize);

        let (dispatcher, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            dispatcher: Some(dispatcher),
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.dispatcher.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.dispatcher.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("Worker attempted to acquire Mutex from Panicked thread, shutting down.")
                .recv();

            match message {
                Ok(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Err(_) => {
                    println!("Worker {} is shutting down.", id);
                    break;
                }
            }
        });
        Worker { id, thread }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threadpool_new_panics_with_zero_size() {
        assert!(std::panic::catch_unwind(|| {
            ThreadPool::new(0);
        })
        .is_err())
    }

    #[test]
    fn test_worker_new_and_execute() {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let worker = Worker::new(0, Arc::clone(&receiver));
        sender.send(Box::new(|| {})).unwrap();
        drop(sender);
        worker.thread.join().unwrap();
    }

    #[test]
    fn test_threadpool_new() {
        let _pool = ThreadPool::new(4);
    }

    #[test]
    fn test_threadpool_execute() {
        let pool = ThreadPool::new(4);
        pool.execute(|| {});
    }

    #[test]
    fn test_threadpool_drop() {
        let pool = ThreadPool::new(4);
        drop(pool);
    }
}
