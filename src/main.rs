fn main() {
    eprintln!("Server successfully starting on port: 8080");
    poker_hand_evaluator::web_server("8080".to_string()); // NOTE: remove the need to put in port
}
// why have thread running when can just run Dockerfile in compose and ping against?

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
