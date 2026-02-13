mod display;
mod evaluation;
mod validation;

use crate::evaluation::evaluation;
use crate::validation::validate;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
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

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Card {
    rank: Rank,
    suit: Suit,
}

#[derive(Debug, PartialEq)]
struct Player {
    name: String,
    cards: Vec<Card>,
}

#[derive(Debug, PartialEq)]
enum Variant {
    FiveCardDraw,
    TexasHoldem,
}

#[derive(Debug, PartialEq)]
pub struct Hand {
    variant: Variant,
    players: Vec<Player>,
    board: Option<Vec<Card>>,
}

#[derive(Debug, PartialEq)]
struct PlayerEval {
    name: String,
    cards: Vec<Card>,
    display: String,
}

#[derive(Debug, PartialEq)]
pub struct Evaluation {
    variant: Variant,
    players: Vec<PlayerEval>,
    board: Option<Vec<Card>>,
    winners: Vec<String>,
    display: String,
}

pub fn evaluate(hand: Hand) -> Result<Evaluation, String> {
    validate(&hand)?;
    Ok(evaluation(hand))
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
    fn to_card_string(&self) -> String {
        format!(
            "{} of {}s",
            self.clone().rank.to_string(),
            self.clone().suit.to_string()
        )
    }

    fn to_rank_string(&self, plural: bool) -> String {
        let plural_addition;

        if self.clone().rank == Rank::Six {
            plural_addition = "es";
        } else {
            plural_addition = "s"
        }

        format!(
            "{}{}",
            self.clone().rank.to_string(),
            if plural { plural_addition } else { "" }
        )
    }

    fn matrix_value(&self) -> usize {
        match self {
            Card {
                rank: Rank::Two,
                suit: Suit::Club,
            } => 0,
            Card {
                rank: Rank::Three,
                suit: Suit::Club,
            } => 1,
            Card {
                rank: Rank::Four,
                suit: Suit::Club,
            } => 2,
            Card {
                rank: Rank::Five,
                suit: Suit::Club,
            } => 3,
            Card {
                rank: Rank::Six,
                suit: Suit::Club,
            } => 4,
            Card {
                rank: Rank::Seven,
                suit: Suit::Club,
            } => 5,
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            } => 6,
            Card {
                rank: Rank::Nine,
                suit: Suit::Club,
            } => 7,
            Card {
                rank: Rank::Ten,
                suit: Suit::Club,
            } => 8,
            Card {
                rank: Rank::Jack,
                suit: Suit::Club,
            } => 9,
            Card {
                rank: Rank::Queen,
                suit: Suit::Club,
            } => 10,
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            } => 11,
            Card {
                rank: Rank::Ace,
                suit: Suit::Club,
            } => 12,
            Card {
                rank: Rank::Two,
                suit: Suit::Diamond,
            } => 13,
            Card {
                rank: Rank::Three,
                suit: Suit::Diamond,
            } => 14,
            Card {
                rank: Rank::Four,
                suit: Suit::Diamond,
            } => 15,
            Card {
                rank: Rank::Five,
                suit: Suit::Diamond,
            } => 16,
            Card {
                rank: Rank::Six,
                suit: Suit::Diamond,
            } => 17,
            Card {
                rank: Rank::Seven,
                suit: Suit::Diamond,
            } => 18,
            Card {
                rank: Rank::Eight,
                suit: Suit::Diamond,
            } => 19,
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            } => 20,
            Card {
                rank: Rank::Ten,
                suit: Suit::Diamond,
            } => 21,
            Card {
                rank: Rank::Jack,
                suit: Suit::Diamond,
            } => 22,
            Card {
                rank: Rank::Queen,
                suit: Suit::Diamond,
            } => 23,
            Card {
                rank: Rank::King,
                suit: Suit::Diamond,
            } => 24,
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            } => 25,
            Card {
                rank: Rank::Two,
                suit: Suit::Heart,
            } => 26,
            Card {
                rank: Rank::Three,
                suit: Suit::Heart,
            } => 27,
            Card {
                rank: Rank::Four,
                suit: Suit::Heart,
            } => 28,
            Card {
                rank: Rank::Five,
                suit: Suit::Heart,
            } => 29,
            Card {
                rank: Rank::Six,
                suit: Suit::Heart,
            } => 30,
            Card {
                rank: Rank::Seven,
                suit: Suit::Heart,
            } => 31,
            Card {
                rank: Rank::Eight,
                suit: Suit::Heart,
            } => 32,
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
            } => 33,
            Card {
                rank: Rank::Ten,
                suit: Suit::Heart,
            } => 34,
            Card {
                rank: Rank::Jack,
                suit: Suit::Heart,
            } => 35,
            Card {
                rank: Rank::Queen,
                suit: Suit::Heart,
            } => 36,
            Card {
                rank: Rank::King,
                suit: Suit::Heart,
            } => 37,
            Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            } => 38,
            Card {
                rank: Rank::Two,
                suit: Suit::Spade,
            } => 39,
            Card {
                rank: Rank::Three,
                suit: Suit::Spade,
            } => 40,
            Card {
                rank: Rank::Four,
                suit: Suit::Spade,
            } => 41,
            Card {
                rank: Rank::Five,
                suit: Suit::Spade,
            } => 42,
            Card {
                rank: Rank::Six,
                suit: Suit::Spade,
            } => 43,
            Card {
                rank: Rank::Seven,
                suit: Suit::Spade,
            } => 44,
            Card {
                rank: Rank::Eight,
                suit: Suit::Spade,
            } => 45,
            Card {
                rank: Rank::Nine,
                suit: Suit::Spade,
            } => 46,
            Card {
                rank: Rank::Ten,
                suit: Suit::Spade,
            } => 47,
            Card {
                rank: Rank::Jack,
                suit: Suit::Spade,
            } => 48,
            Card {
                rank: Rank::Queen,
                suit: Suit::Spade,
            } => 49,
            Card {
                rank: Rank::King,
                suit: Suit::Spade,
            } => 50,
            Card {
                rank: Rank::Ace,
                suit: Suit::Spade,
            } => 51,
        }
    }
}
