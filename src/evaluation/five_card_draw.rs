use crate::TransformedPlayerRequest;

struct PlayerResponse {
    display: String,
    cards: Vec<String>,
    hand: String,
    winner: bool,
}

struct SuccessEvaluationResponse {
    variant: String,
    players: Vec<PlayerResponse>,
}

struct FailureEvaluationResponse {
    message: String,
}

enum EvaluationResponse {
    Success(SuccessEvaluationResponse),
    Failure(FailureEvaluationResponse),
}
pub fn evaluate(players: Vec<TransformedPlayerRequest>) -> Result<(), String> {
    // let res = SuccessEvaluationResponse { // NOTE: or failure
    //     variant: String::from("five-card-draw"),
    //     players: vec![],
    // };

    return Err(String::from("placeholder error"));
}
