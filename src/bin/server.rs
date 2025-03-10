mod validation;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

// use poker_hand_evaluator::get_message;

fn main() {
    web_server(String::from("8080"));
}

fn web_server(port: String) {
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

        println!("line: {:?}", line);

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

    // let (status_line, _filename) = if headers[0] == "GET /variants HTTP/1.1" {
    //     eprintln!("Call to: GET /variants");
    //     ("HTTP/1.1 200 OK", "./src/assets/variants.json")
    // } else if headers[0].starts_with("POST /evaluate/") {
    //     eprintln!("Call to: POST /evaluate");
    //     ("HTTP/1.1 200 OK", "./src/assets/variants.json")
    // } else {
    //     eprintln!("Error: Endpoint path not found");
    //     ("HTTP/1.1 404 NOT FOUND", "./src/assets/404_not_found.json")
    // };

    let contents = String::from("lib binary");
    let length = contents.len();
    let status_line = "HTTP/1.1 404 NOT FOUND";

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    eprintln!("response: {:?}", response);

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
fn validate(request: &EvaluationRequest) -> Result<(), String> {
    if let Err(e) = validation::general_validate(&request.players) {
        return Err(String::from(format!(
            "do error handling better", // NOTE: consider error handling at levels and how work
                                        // together
        )));
    }
    // NOTE: should I just return the strings as Card from validation, or demand on creation?

    match request.variant.as_str() {
        "five_card_draw" => match validation::five_card_draw::validate(&request.players) {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(format!("Error in five_card_draw validation: {}", e));
            }
        },
        _ => Err(String::from(format!(
            "poker variant is not supported <{}>",
            request.variant
        ))),
    }
}
//
// NOTE: if enum Card is made public can skip this step entirely, offer both?
fn transform(players: Vec<PlayerRequest>) -> Result<Vec<TransformedPlayerRequest>, String> {
    players
        .into_iter()
        .map(|player| {
            let transformed_cards: Result<Vec<Card>, String> = player
                .cards
                .into_iter()
                .map(|card| {
                    Card::from_str(&card).map_err(|_| format!("failed to transform card: {}", card))
                })
                .collect();

            transformed_cards.map(|cards| TransformedPlayerRequest {
                display: player.display,
                cards,
            })
        })
        .collect()
}

