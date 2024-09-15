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
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    eprintln!("Error handling connection: {:?}", e);
                    // NOTE: how pass an error through stream back to user?
                }
            }
            Err(error) => eprintln!("Error accepting connection: {:?}", error),
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error> {
    let mut buf_reader = BufReader::new(&mut stream);

    let mut headers: Vec<String> = Vec::new();
    let mut content_length = 0;

    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line)?;
        line = line.trim().to_string();

        if line.is_empty() {
            // Line space between headers and body
            break;
        }

        if let Some(value) = line.strip_prefix("Content-Length: ") {
            content_length = value.parse::<usize>().unwrap_or(0);
        }

        headers.push(line);
    }
    eprintln!("Headers: {:?}", headers);

    let mut body = vec![0; content_length];
    buf_reader.read_exact(&mut body)?;

    let body_string = String::from_utf8_lossy(&body);
    if body_string.len() != 0 {
        eprintln!("Body: {}", body_string);
    }

    let (status_line, filename) = if headers[0] == "GET /variants HTTP/1.1" {
        eprintln!("Call to: GET /variants");
        ("HTTP/1.1 200 OK", "./src/assets/variants.json")
    } else if headers[0].starts_with("POST /evaluate/") {
        eprintln!("Call to: POST /evaluate");
        ("HTTP/1.1 200 OK", "./src/assets/variants.json")
    } else {
        eprintln!("Error: Endpoint path not found");
        ("HTTP/1.1 404 NOT FOUND", "./src/assets/404_not_found.json")
    };

    let contents = fs::read_to_string(filename)?;

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

struct RequestPlayer {
    display: String,
    cards: Vec<String>, // NOTE: validate
}

struct EvaluateRequest {
    players: Vec<RequestPlayer>,
}

struct ResponsePlayer {
    display: String,
    cards: Vec<String>, // NOTE: enum for Cards
    hand: String,
    winner: bool,
}

struct EvaluateResponse {
    players: Vec<ResponsePlayer>,
}

fn evaluate_hands(evalReq: EvaluateRequest) -> EvaluateResponse {
    return String::from("evaluate_hands");
}
