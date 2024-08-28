use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn web_server(port: String) {
    let address = format!("127.0.0.1:{}", port);

    let listener = match TcpListener::bind(&address) {
        Ok(tcp_listener) => tcp_listener,
        Err(error) => panic!("Problem starting the TcpLister: {:?}", error),
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(error) => eprintln!("Error accepting connection: {:?}", error),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(error)) => {
            eprintln!("Failed to read line: {}", error);
            return;
        }
        None => {
            eprintln!("Connetion closed unexpectedly");
            return;
        }
    };

    let (status_line, filename) = if request_line == "GET /variants HTTP/1.1" {
        ("HTTP/1.1 200 OK", "./src/assets/variants.json")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./src/assets/404_not_found.json")
    };

    let contents = match fs::read_to_string(filename) {
        Ok(line) => line,
        Err(error) => {
            eprintln!("Failed to read file {}: {}", filename, error);
            return;
        }
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    if let Err(error) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write reponse: {}", error);
    }
}

