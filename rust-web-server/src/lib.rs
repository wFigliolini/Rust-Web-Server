use std::{
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request : Vec<_> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:?}", http_request);

    let request_line = &http_request[0];

    let (status_line, filename) = parse_http_request(request_line);
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, contents
    );

    stream.write(response.as_bytes()).unwrap();
}

fn parse_http_request(request: &String) -> (&str, &str) {
    match request.split_whitespace().collect::<Vec<&str>>().as_slice() {
        ["GET", "/", _] => ("HTTP/1.1 200 OK", "hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool{
    workers: Vec<Worker>,
    dispatcher: Option<mpsc::Sender<Job>>,
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
        ThreadPool { workers, dispatcher: Some(dispatcher) }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.dispatcher.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool{
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
            let message = receiver.lock().expect("Worker attempted to acquire Mutex from Panicked thread, shutting down.").recv();

            match message {
                Ok(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                },
                Err(_) => {
                    println!("Worker {} is shutting down.", id);
                    break;
                }
            }
        });
        Worker { id, thread}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_http_request_ok() {
        let request = "GET / HTTP/1.1".to_string();
        let (status_line, filename) = parse_http_request(&request);
        assert_eq!(status_line, "HTTP/1.1 200 OK");
        assert!(!filename.is_empty());
    }

    #[test]
    fn test_parse_http_request_not_found() {
        let request = "GET /does_not_exist HTTP/1.1".to_string();
        let (status_line, filename) = parse_http_request(&request);
        assert_eq!(status_line, "HTTP/1.1 404 NOT FOUND");
        assert_eq!(filename, "404.html");
    }
}