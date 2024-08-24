use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap(); // NOTE: irrecoverable

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap(); // NOTE: recoverable

    let (status_line, filename) = if request_line == "GET /variants HTTP/1.1" {
        ("HTTP/1.1 200 OK", "./src/assets/variants.json")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./src/assets/404_not_found.json")
    };

    let contents = fs::read_to_string(filename).unwrap(); // NOTE: recoverable
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap(); // NOTE: recoverable
}
