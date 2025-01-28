pub fn get_message() -> String {
    String::from("lib binary")
}

fn main() {
    println!("hello world");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_message() {
        assert_eq!(get_message(), String::from("lib binary"));
    }
}

// struct RequestPlayer {
//     display: String,
//     cards: Vec<String>, // NOTE: validate
// }
//
// struct EvaluateRequest {
//     players: Vec<RequestPlayer>,
// }
//
// struct ResponsePlayer {
//     display: String,
//     cards: Vec<String>, // NOTE: enum for Cards
//     hand: String,
//     winner: bool,
// }
//
// struct EvaluateResponse {h
//     players: Vec<ResponsePlayer>,
// }
//
// fn evaluate_hands(evalReq: EvaluateRequest) -> EvaluateResponse {
//     return String::from("evaluate_hands");
// }

// fn main() {
//     eprintln!("Server successfully starting on port: 8080");
//     poker_hand_evaluator::web_server("8080".to_string()); // NOTE: remove the need to put in port
// }
// why have thread running when can just run Dockerfile in compose and ping against?
