pub struct ThreadPool;

impl ThreadPool{
    pub fn new(size: u32) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        unimplemented!()
    }
}