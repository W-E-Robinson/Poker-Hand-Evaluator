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
            // NOTE: somehow pass up and print recoverable error from handle_connection?
        }
    }
}

fn handle_connection(mut stream: TcpStream) { // NOTE: common io Error here can be returned for
                                              // propagation?
    let mut buf_reader = BufReader::new(&mut stream);

    let mut headers: Vec<String> = Vec::new();
    let mut content_length = 0;

    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line).unwrap_or_else(|error| {
            eprintln!("Failed to read line: {:?}", error);
            return 1; // NOTE: what here? + consider all error handling, can propogate a common
                      // error up to handle_connection for all these for non recoverable?
        });
        line = line.trim().to_string(); // NOTE: is this in place?

        if line.is_empty() {
            break; // NOTE: how resistant is this? always only one line of space?
        }

        if let Some(value) = line.strip_prefix("Content-Length: ") {
            content_length = value.parse::<usize>().unwrap_or(0);
        }

        headers.push(line);
    }
    eprintln!("Headers: {:?}", headers);

    let mut body = vec![0; content_length]; // NOTE: how resistant is this? vec or array?
    buf_reader.read_exact(&mut body).unwrap_or_else(|error| { // NOTE: how work with the bytes?
        eprintln!("Failed to read body: {:?}", error);
        // NOTE: what if GET and no body?
        return;
    });

    let body_string = String::from_utf8_lossy(&body); // NOTE: what now?
    if body_string.len() != 0 {
        eprintln!("Body: {}", body_string);
    }

    let (status_line, filename) = if headers[0] == "GET /variants HTTP/1.1" {
        eprintln!("Call to: GET /variants");
        ("HTTP/1.1 200 OK", "./src/assets/variants.json")
    } else if headers[0].starts_with("POST /evaluate/") {
        eprintln!("Call to: POST /evaluate");
        let output = evaluate_hands();
        eprint!("output: {}", output);
        ("HTTP/1.1 200 OK", "./src/assets/variants.json")
    } else {
        eprintln!("Error: Endpoint path not found");
        ("HTTP/1.1 404 NOT FOUND", "./src/assets/404_not_found.json")
    };

    let contents = match fs::read_to_string(filename) {
        Ok(line) => line,
        Err(error) => {
            eprintln!("Failed to read file {}: {:?}", filename, error);
            return;
        }
    };

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream
        .write_all(response.as_bytes())
        .unwrap_or_else(|error| {
            eprintln!("Failed to write reponse: {:?}", error);
            return;
        });
    stream.flush().unwrap_or_else(|error| {
        eprintln!("Failed to flush stream: {:?}", error);
        return;
    });
}

fn evaluate_hands() -> String {
    return String::from("evaluate_hands");
}
