fn main() {
    eprintln!("Server successfully starting on port: 8080");
    poker_hand_evaluator::web_server("8080".to_string());
}
