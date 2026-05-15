mod evaluation;
mod types;
mod validation;

// pub fn evaluate(request: EvaluationRequest) -> Result<EvaluationResponse, String> {
/* pub fn evaluate(request: EvaluationRequest) {
    if let Err(e) = validate(&request) {
        println!("Validation failed: {}", e); // NOTE: don't print Err, return
    }

    let transformed_players = transform(request.players).map_err(|err| err); // NOTE: returning?

    // NOTE: attach variant back on + rest of response
    // let evaluation = match request.variant.as_str() {
    //     "five_card_draw" => evaluation::five_card_draw::evaluate(transformed_players),
    //     _ => Err(String::from(format!(
    //         "Evaluation error: poker variant not is supported <{}>",
    //         request.variant // NOTE: return error
    //     ))),
    // };
} */

pub struct PlayerRequest {
    // NOTE: move to a types.rs / structures.rs
    display: String,
    cards: Vec<String>,
}

// NOTE: offer Card publically? or offer both?
pub struct TransformedPlayerRequest {
    display: String,
    cards: Vec<Card>,
}

// NOTE: where should various structs/enums live?
pub struct EvaluationRequest {
    // NOTE: have all structs here?
    variant: String,
    players: Vec<PlayerRequest>,
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

// NOTE: this needs full testing = all impls below
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

    fn to_display(self, plural: bool) -> String {
        match self.rank {
            Rank::Two => format!("Two{}", if plural { "s" } else { "" }),
            Rank::Three => format!("Three{}", if plural { "s" } else { "" }),
            Rank::Four => format!("Four{}", if plural { "s" } else { "" }),
            Rank::Five => format!("Five{}", if plural { "s" } else { "" }),
            Rank::Six => format!("Six{}", if plural { "es" } else { "" }),
            Rank::Seven => format!("Seven{}", if plural { "s" } else { "" }),
            Rank::Eight => format!("Eight{}", if plural { "s" } else { "" }),
            Rank::Nine => format!("Nine{}", if plural { "s" } else { "" }),
            Rank::Ten => format!("Ten{}", if plural { "s" } else { "" }),
            Rank::Jack => format!("Jack{}", if plural { "s" } else { "" }),
            Rank::Queen => format!("Queen{}", if plural { "s" } else { "" }),
            Rank::King => format!("King{}", if plural { "s" } else { "" }),
            Rank::Ace => format!("Ace{}", if plural { "s" } else { "" }),
            // NOTE: can there be an error? - think all impls
            // }?;
        }
    }

    // fn binary_value(&self) -> usize {
    //     match self.rank {
    //         Rank::Ace => 1,
    //         Rank::Two => 2,
    //         Rank::Three => 4,
    //         Rank::Four => 8,
    //         Rank::Five => 16,
    //         Rank::Six => 32,
    //         Rank::Seven => 64,
    //         Rank::Eight => 128,
    //         Rank::Nine => 256,
    //         Rank::Ten => 512,
    //         Rank::Jack => 1024,
    //         Rank::Queen => 2048,
    //         Rank::King => 4096,
    //         Rank::Ace => 8192,
    //     }
    // }
}
