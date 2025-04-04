use std::net::TcpListener;

use rust_web_server::multithreading::threadpool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            rust_web_server::handle_connection(stream);
        });
    }
}
