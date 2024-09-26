use std::{
    fs,
};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;

fn normalize_string(s: &str) -> String {
    s.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("")
}

#[test]
fn test_handle_evaluate_five_card_draw() {
    thread::spawn(|| {
        poker_hand_evaluator::web_server("8083".to_string());
    });

    std::thread::sleep(std::time::Duration::from_millis(100));

    let body_filename = "./fixtures/five_card_draw/sample1/body.json";
    let body = match fs::read_to_string(body_filename) {
        Ok(line) => line,
        Err(error) => {
            eprintln!("Failed to read file {}: {}", body_filename, error);
            return;
        }
    };

    let mut stream = TcpStream::connect("127.0.0.1:8083").expect("Failed to connect to server");

    let request = b"POST /variants/five-card-draw HTTP/1.1\r\n\r\n"; // NOTE: how add body?
    stream
        .write_all(request)
        .expect("Failed to write to stream");

    let mut buffer = Vec::new();
    stream
        .read_to_end(&mut buffer)
        .expect("Failed to read from stream");
    let response = String::from_utf8_lossy(&buffer);

    assert!(response.starts_with("HTTP/1.1 200 OK"));

    let response_filename = "./fixtures/five_card_draw/sample1/body.json";
    let response_body = match fs::read_to_string(response_filename) {
        Ok(line) => line,
        Err(error) => {
            eprintln!("Failed to read file {}: {}", response_filename, error);
            return;
        }
    };

    let normalized_response = normalize_string(&response);
    let normalized_expected_response = normalize_string(&response_body);

    assert!(normalized_response.contains(&normalized_expected_response));
}
