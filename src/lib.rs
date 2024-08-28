use regex::Regex;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn web_server(port: String) {
    let address = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(&address).unwrap_or_else(|error| {
        panic!("Failed to bind to address {}: {:?}", address, error);
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(error) => eprintln!("Error accepting connection: {:?}", error),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // Read the request line
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(error)) => {
            eprintln!("Failed to read line: {:?}", error);
            return;
        }
        None => {
            eprintln!("Connetion closed unexpectedly");
            return;
        }
    };

    // Read headers
    let mut headers = Vec::new();
    loop {
        let line = match buf_reader.lines().next() {
            Some(Ok(line)) => line,
            Some(Err(error)) => {
                eprintln!("Failed to read line: {:?}", error);
                return;
            }
            None => {
                eprintln!("Connection closed unexpectedly");
                return;
            }
        };

        if line.is_empty() {
            break;
        }
        headers.push(line);
    }

    let evaluate_re = Regex::new(r"^POST /evaluate/[a-z-]+ HTTP/1.1$").unwrap_or_else(|error| {
        panic!("Unable to create evaluate path Regex: {:?}", error);
    });

    let (status_line, filename) = if request_line == "GET /variants HTTP/1.1" {
        eprintln!("Call to: GET /variants");
        ("HTTP/1.1 200 OK", "./src/assets/variants.json")
    } else if evaluate_re.is_match(&request_line) {
        eprintln!("Call to: POST /evaluate");
        ("HTTP/1.1 200 OK", "./src/assets/variants.json")
    } else {
        eprintln!("Error: Endpoint path not found");
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

    stream
        .write_all(response.as_bytes())
        .unwrap_or_else(|error| {
            eprintln!("Failed to write reponse: {}", error);
            return;
        });
}

// use std::io::{BufRead, BufReader, Read};
// use std::net::TcpStream;
//
// fn handle_request(stream: TcpStream) {
//     let mut buf_reader = BufReader::new(stream);
//
//     // Read the request line (e.g., "POST /path HTTP/1.1")
//     let request_line = match buf_reader.lines().next() {
//         Some(Ok(line)) => line,
//         Some(Err(error)) => {
//             eprintln!("Failed to read line: {}", error);
//             return;
//         }
//         None => {
//             eprintln!("Connection closed unexpectedly");
//             return;
//         }
//     };
//
//     eprintln!("Request line: {}", request_line);
//
//     // Read headers
//     let mut headers = Vec::new();
//     loop {
//         let line = match buf_reader.lines().next() {
//             Some(Ok(line)) => line,
//             Some(Err(error)) => {
//                 eprintln!("Failed to read line: {}", error);
//                 return;
//             }
//             None => {
//                 eprintln!("Connection closed unexpectedly");
//                 return;
//             }
//         };
//
//         if line.is_empty() {
//             // Empty line signifies the end of the headers
//             break;
//         }
//         headers.push(line);
//     }
//
//     // Extract Content-Length if present
//     let mut content_length = 0;
//     for header in &headers {
//         if let Some(length) = header.strip_prefix("Content-Length:") {
//             content_length = length.trim().parse::<usize>().unwrap_or(0);
//             break;
//         }
//     }
//
//     // Read the body based on the Content-Length
//     let mut body = vec![0; content_length];
//     if let Err(error) = buf_reader.read_exact(&mut body) {
//         eprintln!("Failed to read request body: {}", error);
//         return;
//     }
//
//     // Convert body to a string (if it's text-based, e.g., JSON or form data)
//     let body_str = String::from_utf8_lossy(&body);
//     eprintln!("Body: {}", body_str);
// }
//
// fn main() {
//     // Assuming you have a TcpStream, pass it to handle_request()
//     // For example:
//     // let stream = ...;
//     // handle_request(stream);
// }
