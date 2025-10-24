mod validation;

use crate::validation::validate;

pub fn evaluate(hand: Hand) -> Result<(), String> {
    validate(&hand)?;

    Ok(())
}

#[derive(Eq, Hash, PartialEq, Clone)]
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

#[derive(Eq, Hash, PartialEq, Clone)]
enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Card {
    rank: Rank,
    suit: Suit,
}

struct Player {
    display: String,
    cards: Vec<Card>,
}

enum Variant {
    FiveCardDraw,
    TexasHoldem,
}

pub struct Hand {
    variant: Variant,
    players: Vec<Player>,
    board: Option<Vec<Card>>,
}

impl Hand {
    pub fn board_len(&self) -> usize {
        self.board.as_ref().map_or(0, |b| b.len())
    }
}

impl Variant {
    fn to_string(&self) -> String {
        match self {
            Self::FiveCardDraw => String::from("Five-card draw"),
            Self::TexasHoldem => String::from("Texas hold 'em"),
        }
    }

    fn number_board_cards(&self) -> usize {
        match self {
            Self::FiveCardDraw => 0,
            Self::TexasHoldem => 5,
        }
    }

    fn number_player_cards(&self) -> usize {
        match self {
            Self::FiveCardDraw => 5,
            Self::TexasHoldem => 2,
        }
    }

    fn number_burn_cards(&self) -> usize {
        match self {
            Self::FiveCardDraw => 3,
            Self::TexasHoldem => 3,
        }
    }
}

impl Rank {
    fn to_string(self) -> String {
        match self {
            Rank::Two => format!("Two"),
            Rank::Three => format!("Three"),
            Rank::Four => format!("Four"),
            Rank::Five => format!("Five"),
            Rank::Six => format!("Six"),
            Rank::Seven => format!("Seven"),
            Rank::Eight => format!("Eight"),
            Rank::Nine => format!("Nine"),
            Rank::Ten => format!("Ten"),
            Rank::Jack => format!("Jack"),
            Rank::Queen => format!("Queen"),
            Rank::King => format!("King"),
            Rank::Ace => format!("Ace"),
        }
    }
}

impl Suit {
    fn to_string(self) -> String {
        match self {
            Suit::Heart => format!("Heart"),
            Suit::Diamond => format!("Diamond"),
            Suit::Club => format!("Club"),
            Suit::Spade => format!("Spade"),
        }
    }
}

impl Card {
    fn to_card_string(self) -> String {
        format!("{} of {}s", self.rank.to_string(), self.suit.to_string())
    }

    fn to_rank_string(self, plural: bool) -> String {
        format!("{}{}", self.rank.to_string(), if plural { "s" } else { "" })
    }
}

// Rank::Two => format!("Two{}", if plural { "s" } else { "" }),
// Rank::Three => format!("Three{}", if plural { "s" } else { "" }),
// Rank::Four => format!("Four{}", if plural { "s" } else { "" }),
// Rank::Five => format!("Five{}", if plural { "s" } else { "" }),
// Rank::Six => format!("Six{}", if plural { "es" } else { "" }),
// Rank::Seven => format!("Seven{}", if plural { "s" } else { "" }),
// Rank::Eight => format!("Eight{}", if plural { "s" } else { "" }),
// Rank::Nine => format!("Nine{}", if plural { "s" } else { "" }),
// Rank::Ten => format!("Ten{}", if plural { "s" } else { "" }),
// Rank::Jack => format!("Jack{}", if plural { "s" } else { "" }),
// Rank::Queen => format!("Queen{}", if plural { "s" } else { "" }),
// Rank::King => format!("King{}", if plural { "s" } else { "" }),
// Rank::Ace => format!("Ace{}", if plural { "s" } else { "" }),
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
// pub fn evaluate(request: EvaluationRequest) -> Result<EvaluationResponse, String> {
// NOTE: this needs full testing = all impls below
// let transformed_players = transform(request.players).map_err(|err| err); // NOTE: returning?
// NOTE: attach variant back on + rest of response
// let evaluation = match request.variant.as_str() {
//     "five_card_draw" => evaluation::five_card_draw::evaluate(transformed_players),
//     _ => Err(String::from(format!(
//         "Evaluation error: poker variant not is supported <{}>",
//         request.variant // NOTE: return error
//     ))),
// };
