mod evaluation;
mod validation;

fn validate(request: &EvaluationRequest) -> Result<(), String> {
    if let Err(e) = validation::general_validate(&request.players) {
        return Err(String::from(format!(
            "do error handling better",
        )));
    }

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

// pub fn evaluate(request: EvaluationRequest) -> Result<EvaluationResponse, String> {
pub fn evaluate(request: EvaluationRequest) {
    if let Err(e) = validate(&request) {
        println!("Validation failed: {}", e); // NOTE: don't print Err
    }

    let evaluation = match request.variant.as_str() {
        "five_card_draw" => evaluation::five_card_draw::evaluate(&request.players),
        _ => Err(String::from(format!(
            "Evaluation error: poker variant not is supported <{}>",
            request.variant
        ))),
    };
}

pub struct PlayerRequest {
    display: String,
    cards: Vec<String>,
}

pub struct EvaluationRequest {
    variant: String,
    players: Vec<PlayerRequest>,
}

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
