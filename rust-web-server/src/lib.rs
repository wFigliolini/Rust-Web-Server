pub mod multithreading;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,

};

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request : Vec<_> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

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