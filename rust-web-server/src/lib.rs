use std::thread;

pub struct ThreadPool{
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool{
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: u32) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size as usize);

        for _ in 0..size {
            // create some threads and store them in the vector
        }
        ThreadPool { threads }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        unimplemented!()
    }
}