pub struct ThreadPool;

impl ThreadPool{
    pub fn new(_size: u32) -> ThreadPool {
        ThreadPool
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        unimplemented!()
    }
}