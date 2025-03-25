use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request : Vec<_> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:?}", http_request);


    if http_request.len() == 0 {
        return;
    }
    if http_request[0].contains("favicon.ico") {
        return;
    }
    if http_request[0].contains("GET / HTTP/1.1") {
        get_home_page(stream);
    } else {
        get_404_page(stream);
    }
}

fn get_home_page(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, contents
    );

    stream.write(response.as_bytes()).unwrap();
}

fn get_404_page(mut stream: TcpStream){
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let contents = fs::read_to_string("404.html").unwrap();
    let length = contents.len();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, contents
    );

    stream.write(response.as_bytes()).unwrap();
}