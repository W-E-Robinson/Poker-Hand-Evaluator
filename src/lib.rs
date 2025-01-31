mod evaluation;
mod validation;

fn validate(request: &EvaluationRequest) -> Result<(), String> {
    if let Err(e) = validation::general_validate(&request.players) {
        return Err(String::from(format!(
            "do error handling better", // NOTE: consider error handling at levels and how work
                                        // together
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

// NOTE: make public?
#[derive(Debug)]
enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

// NOTE: make public?
#[derive(Debug)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

// NOTE: make public?
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn from_str(string: &str) -> Result<Self, String> {
        if string.len() != 2 || !string.is_ascii() {
            return Err(format!("invalid card string: {}", string));
        }

        let rank_char = &string[..1];
        let suit_char = &string[1..2];

        let rank = match rank_char {
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "T" => Ok(Rank::Ten),
            "J" => Ok(Rank::Jack),
            "Q" => Ok(Rank::Queen),
            "K" => Ok(Rank::King),
            "A" => Ok(Rank::Ace),
            _ => Err(format!("invalid rank: {}", rank_char)),
        }?;

        let suit = match suit_char {
            "h" => Ok(Suit::Heart),
            "d" => Ok(Suit::Diamond),
            "c" => Ok(Suit::Club),
            "s" => Ok(Suit::Spade),
            _ => Err(format!("invalid suit: {}", suit_char)),
        }?;

        Ok(Card { rank, suit })
    }

    fn to_str(card: Card) -> Result<String, String> {
        let suit_char = match card.suit {
            Suit::Heart => Ok("h"),
            Suit::Diamond => Ok("d"),
            Suit::Club => Ok("c"),
            Suit::Spade => Ok("s"),
            _ => Err(format!("invalid card suit: {:?}", card.suit)),
        }?;

        let rank_char = match card.rank {
            Rank::Two => Ok("2"),
            Rank::Three => Ok("3"),
            Rank::Four => Ok("4"),
            Rank::Five => Ok("5"),
            Rank::Six => Ok("6"),
            Rank::Seven => Ok("7"),
            Rank::Eight => Ok("8"),
            Rank::Nine => Ok("9"),
            Rank::Ten => Ok("T"),
            Rank::Jack => Ok("J"),
            Rank::Queen => Ok("Q"),
            Rank::King => Ok("K"),
            Rank::Ace => Ok("A"),
            _ => Err(format!("invalid card rank: {:?}", card.rank)),
        }?;

        Ok(String::from(format!("{}{}", rank_char, suit_char)))
    }
}
