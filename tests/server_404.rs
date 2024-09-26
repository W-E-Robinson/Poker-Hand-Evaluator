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
fn test_handle_404() {
    thread::spawn(|| {
        poker_hand_evaluator::web_server("8082".to_string());
    });

    std::thread::sleep(std::time::Duration::from_millis(100));

    let mut stream = TcpStream::connect("127.0.0.1:8082").expect("Failed to connect to server");

    let request = b"GET /none-method HTTP/1.1\r\n\r\n";
    stream
        .write_all(request)
        .expect("Failed to write to stream");

    let mut buffer = Vec::new();
    stream
        .read_to_end(&mut buffer)
        .expect("Failed to read from stream");
    let response = String::from_utf8_lossy(&buffer);

    assert!(response.starts_with("HTTP/1.1 404 NOT FOUND"));

    let expected_response = r#"
    {
        "error": "Endpoint call was not found.",
        "code": 404,
        "display": "NOT FOUND"
    }"#;

    let normalized_response = normalize_string(&response);
    let normalized_expected_response = normalize_string(expected_response);

    assert!(normalized_response.contains(&normalized_expected_response));
}
