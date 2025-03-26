use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool{
    workers: Vec<Worker>,
    dispatcher: mpsc::Sender<Job>,
}

impl ThreadPool{
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
        ThreadPool { workers, dispatcher }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        
        self.dispatcher.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
    receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {});
        Worker { id, thread, receiver }
    }
}

struct Job;

impl Job{
    fn new() -> Job {
        Job
    }
}