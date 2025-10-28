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
    winner: bool,
    display: String,
}

#[derive(Debug, PartialEq)]
pub struct Evaluation {
    variant: Variant,
    players: Vec<PlayerEval>,
    board: Option<Vec<Card>>,
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

    fn binary_value(&self) -> usize {
        match self.rank {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 4,
            Rank::Four => 8,
            Rank::Five => 16,
            Rank::Six => 32,
            Rank::Seven => 64,
            Rank::Eight => 128,
            Rank::Nine => 256,
            Rank::Ten => 512,
            Rank::Jack => 1024,
            Rank::Queen => 2048,
            Rank::King => 4096,
            Rank::Ace => 8192,
        }
    }
}
