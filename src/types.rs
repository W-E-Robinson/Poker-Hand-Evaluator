use std::fmt;

use crate::constants::NUMBER_RANKS;

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum Rank {
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
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_string = match self {
            Rank::Two => "Two",
            Rank::Three => "Three",
            Rank::Four => "Four",
            Rank::Five => "Five",
            Rank::Six => "Six",
            Rank::Seven => "Seven",
            Rank::Eight => "Eight",
            Rank::Nine => "Nine",
            Rank::Ten => "Ten",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
            Rank::Ace => "Ace",
        };

        write!(f, "{}", display_string)
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_string = match self {
            Suit::Heart => "Heart",
            Suit::Diamond => "Diamond",
            Suit::Club => "Club",
            Suit::Spade => "Spade",
        };

        write!(f, "{}", display_string)
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}s", self.rank.to_string(), self.suit.to_string())
    }
}
impl Card {
    pub fn matrix_value(&self) -> usize {
        let rank_value = match self.rank {
            Rank::Two => 0,
            Rank::Three => 1,
            Rank::Four => 2,
            Rank::Five => 3,
            Rank::Six => 4,
            Rank::Seven => 5,
            Rank::Eight => 6,
            Rank::Nine => 7,
            Rank::Ten => 8,
            Rank::Jack => 9,
            Rank::Queen => 10,
            Rank::King => 11,
            Rank::Ace => 12,
        };

        let suit_multiplier = match self.suit {
            Suit::Club => 0,
            Suit::Diamond => 1,
            Suit::Heart => 2,
            Suit::Spade => 3,
        };

        rank_value + (suit_multiplier * NUMBER_RANKS)
    }
}

pub struct Player {
    pub id: String,
    pub cards: Vec<Card>,
}

pub enum Variant {
    FiveCardDraw,
}
impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_string = match self {
            Variant::FiveCardDraw => "Five-card draw",
        };

        write!(f, "{}", display_string)
    }
}
impl Variant {
    pub fn cards_per_player(&self) -> usize {
        match self {
            Variant::FiveCardDraw => 5,
        }
    }
    pub fn num_board_cards(&self) -> Option<usize> {
        match self {
            Variant::FiveCardDraw => None,
        }
    }
}

pub struct Hand {
    pub id: String,
    pub variant: Variant,
    pub players: Vec<Player>,
    pub board: Option<Vec<Card>>,
    pub burn_cards: Option<Vec<Card>>,
    pub discarded_cards: Option<Vec<Card>>,
    pub remaining_deck: Vec<Card>,
}

#[derive(Debug)]
pub struct PlayerEval {
    pub id: String,
    pub hand: String,
}

#[derive(Debug)]
pub struct Evaluation {
    pub id: String,
    pub players: Vec<PlayerEval>,
    pub winners: Vec<String>,
    pub winning_hand: String,
}

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    Validation,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    pub id: String,
    pub error_type: ErrorType,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_impl_to_string_two_heart() {
        let card = Card {
            rank: Rank::Two,
            suit: Suit::Heart,
        };
        assert_eq!(card.to_string(), "Two of Hearts");
    }

    #[test]
    fn test_card_impl_to_string_three_heart() {
        let card = Card {
            rank: Rank::Three,
            suit: Suit::Heart,
        };
        assert_eq!(card.to_string(), "Three of Hearts");
    }

    #[test]
    fn test_card_impl_to_string_four_heart() {
        let card = Card {
            rank: Rank::Four,
            suit: Suit::Heart,
        };
        assert_eq!(card.to_string(), "Four of Hearts");
    }

    #[test]
    fn test_card_impl_to_string_five_heart() {
        let card = Card {
            rank: Rank::Five,
            suit: Suit::Heart,
        };
        assert_eq!(card.to_string(), "Five of Hearts");
    }

    #[test]
    fn test_card_impl_to_string_six_heart() {
        let card = Card {
            rank: Rank::Six,
            suit: Suit::Heart,
        };
        assert_eq!(card.to_string(), "Six of Hearts");
    }

    #[test]
    fn test_card_impl_to_string_seven_heart() {
        let card = Card {
            rank: Rank::Seven,
            suit: Suit::Heart,
        };
        assert_eq!(card.to_string(), "Seven of Hearts");
    }

    #[test]
    fn test_card_impl_to_string_eight_heart() {
        let card = Card {
            rank: Rank::Eight,
            suit: Suit::Heart,
        };
        assert_eq!(card.to_string(), "Eight of Hearts");
    }

    #[test]
    fn test_card_impl_to_string_nine_heart() {
        let card = Card {
            rank: Rank::Nine,
            suit: Suit::Heart,
        };
        assert_eq!(card.to_string(), "Nine of Hearts");
    }

    #[test]
    fn test_card_impl_to_string_ten_heart() {
        let card = Card {
            rank: Rank::Ten,
            suit: Suit::Heart,
        };
        assert_eq!(card.to_string(), "Ten of Hearts");
    }

    #[test]
    fn test_card_impl_to_string_jack_heart() {
        let card = Card {
            rank: Rank::Jack,
            suit: Suit::Heart,
        };
        assert_eq!(card.to_string(), "Jack of Hearts");
    }

    #[test]
    fn test_card_impl_to_string_queen_heart() {
        let card = Card {
            rank: Rank::Queen,
            suit: Suit::Heart,
        };
        assert_eq!(card.to_string(), "Queen of Hearts");
    }

    #[test]
    fn test_card_impl_to_string_king_heart() {
        let card = Card {
            rank: Rank::King,
            suit: Suit::Heart,
        };
        assert_eq!(card.to_string(), "King of Hearts");
    }

    #[test]
    fn test_card_impl_to_string_ace_heart() {
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Heart,
        };
        assert_eq!(card.to_string(), "Ace of Hearts");
    }

    #[test]
    fn test_card_impl_to_string_two_spade() {
        let card = Card {
            rank: Rank::Two,
            suit: Suit::Spade,
        };
        assert_eq!(card.to_string(), "Two of Spades");
    }

    #[test]
    fn test_card_impl_to_string_three_spade() {
        let card = Card {
            rank: Rank::Three,
            suit: Suit::Spade,
        };
        assert_eq!(card.to_string(), "Three of Spades");
    }

    #[test]
    fn test_card_impl_to_string_four_spade() {
        let card = Card {
            rank: Rank::Four,
            suit: Suit::Spade,
        };
        assert_eq!(card.to_string(), "Four of Spades");
    }

    #[test]
    fn test_card_impl_to_string_five_spade() {
        let card = Card {
            rank: Rank::Five,
            suit: Suit::Spade,
        };
        assert_eq!(card.to_string(), "Five of Spades");
    }

    #[test]
    fn test_card_impl_to_string_six_spade() {
        let card = Card {
            rank: Rank::Six,
            suit: Suit::Spade,
        };
        assert_eq!(card.to_string(), "Six of Spades");
    }

    #[test]
    fn test_card_impl_to_string_seven_spade() {
        let card = Card {
            rank: Rank::Seven,
            suit: Suit::Spade,
        };
        assert_eq!(card.to_string(), "Seven of Spades");
    }

    #[test]
    fn test_card_impl_to_string_eight_spade() {
        let card = Card {
            rank: Rank::Eight,
            suit: Suit::Spade,
        };
        assert_eq!(card.to_string(), "Eight of Spades");
    }

    #[test]
    fn test_card_impl_to_string_nine_spade() {
        let card = Card {
            rank: Rank::Nine,
            suit: Suit::Spade,
        };
        assert_eq!(card.to_string(), "Nine of Spades");
    }

    #[test]
    fn test_card_impl_to_string_ten_spade() {
        let card = Card {
            rank: Rank::Ten,
            suit: Suit::Spade,
        };
        assert_eq!(card.to_string(), "Ten of Spades");
    }

    #[test]
    fn test_card_impl_to_string_jack_spade() {
        let card = Card {
            rank: Rank::Jack,
            suit: Suit::Spade,
        };
        assert_eq!(card.to_string(), "Jack of Spades");
    }

    #[test]
    fn test_card_impl_to_string_queen_spade() {
        let card = Card {
            rank: Rank::Queen,
            suit: Suit::Spade,
        };
        assert_eq!(card.to_string(), "Queen of Spades");
    }

    #[test]
    fn test_card_impl_to_string_king_spade() {
        let card = Card {
            rank: Rank::King,
            suit: Suit::Spade,
        };
        assert_eq!(card.to_string(), "King of Spades");
    }

    #[test]
    fn test_card_impl_to_string_ace_spade() {
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        };
        assert_eq!(card.to_string(), "Ace of Spades");
    }

    #[test]
    fn test_card_impl_to_string_two_club() {
        let card = Card {
            rank: Rank::Two,
            suit: Suit::Club,
        };
        assert_eq!(card.to_string(), "Two of Clubs");
    }

    #[test]
    fn test_card_impl_to_string_three_club() {
        let card = Card {
            rank: Rank::Three,
            suit: Suit::Club,
        };
        assert_eq!(card.to_string(), "Three of Clubs");
    }

    #[test]
    fn test_card_impl_to_string_four_club() {
        let card = Card {
            rank: Rank::Four,
            suit: Suit::Club,
        };
        assert_eq!(card.to_string(), "Four of Clubs");
    }

    #[test]
    fn test_card_impl_to_string_five_club() {
        let card = Card {
            rank: Rank::Five,
            suit: Suit::Club,
        };
        assert_eq!(card.to_string(), "Five of Clubs");
    }

    #[test]
    fn test_card_impl_to_string_six_club() {
        let card = Card {
            rank: Rank::Six,
            suit: Suit::Club,
        };
        assert_eq!(card.to_string(), "Six of Clubs");
    }

    #[test]
    fn test_card_impl_to_string_seven_club() {
        let card = Card {
            rank: Rank::Seven,
            suit: Suit::Club,
        };
        assert_eq!(card.to_string(), "Seven of Clubs");
    }

    #[test]
    fn test_card_impl_to_string_eight_club() {
        let card = Card {
            rank: Rank::Eight,
            suit: Suit::Club,
        };
        assert_eq!(card.to_string(), "Eight of Clubs");
    }

    #[test]
    fn test_card_impl_to_string_nine_club() {
        let card = Card {
            rank: Rank::Nine,
            suit: Suit::Club,
        };
        assert_eq!(card.to_string(), "Nine of Clubs");
    }

    #[test]
    fn test_card_impl_to_string_ten_club() {
        let card = Card {
            rank: Rank::Ten,
            suit: Suit::Club,
        };
        assert_eq!(card.to_string(), "Ten of Clubs");
    }

    #[test]
    fn test_card_impl_to_string_jack_club() {
        let card = Card {
            rank: Rank::Jack,
            suit: Suit::Club,
        };
        assert_eq!(card.to_string(), "Jack of Clubs");
    }

    #[test]
    fn test_card_impl_to_string_queen_club() {
        let card = Card {
            rank: Rank::Queen,
            suit: Suit::Club,
        };
        assert_eq!(card.to_string(), "Queen of Clubs");
    }

    #[test]
    fn test_card_impl_to_string_king_club() {
        let card = Card {
            rank: Rank::King,
            suit: Suit::Club,
        };
        assert_eq!(card.to_string(), "King of Clubs");
    }

    #[test]
    fn test_card_impl_to_string_ace_club() {
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Club,
        };
        assert_eq!(card.to_string(), "Ace of Clubs");
    }

    #[test]
    fn test_card_impl_to_string_two_diamond() {
        let card = Card {
            rank: Rank::Two,
            suit: Suit::Diamond,
        };
        assert_eq!(card.to_string(), "Two of Diamonds");
    }

    #[test]
    fn test_card_impl_to_string_three_diamond() {
        let card = Card {
            rank: Rank::Three,
            suit: Suit::Diamond,
        };
        assert_eq!(card.to_string(), "Three of Diamonds");
    }

    #[test]
    fn test_card_impl_to_string_four_diamond() {
        let card = Card {
            rank: Rank::Four,
            suit: Suit::Diamond,
        };
        assert_eq!(card.to_string(), "Four of Diamonds");
    }

    #[test]
    fn test_card_impl_to_string_five_diamond() {
        let card = Card {
            rank: Rank::Five,
            suit: Suit::Diamond,
        };
        assert_eq!(card.to_string(), "Five of Diamonds");
    }

    #[test]
    fn test_card_impl_to_string_six_diamond() {
        let card = Card {
            rank: Rank::Six,
            suit: Suit::Diamond,
        };
        assert_eq!(card.to_string(), "Six of Diamonds");
    }

    #[test]
    fn test_card_impl_to_string_seven_diamond() {
        let card = Card {
            rank: Rank::Seven,
            suit: Suit::Diamond,
        };
        assert_eq!(card.to_string(), "Seven of Diamonds");
    }

    #[test]
    fn test_card_impl_to_string_eight_diamond() {
        let card = Card {
            rank: Rank::Eight,
            suit: Suit::Diamond,
        };
        assert_eq!(card.to_string(), "Eight of Diamonds");
    }

    #[test]
    fn test_card_impl_to_string_nine_diamond() {
        let card = Card {
            rank: Rank::Nine,
            suit: Suit::Diamond,
        };
        assert_eq!(card.to_string(), "Nine of Diamonds");
    }

    #[test]
    fn test_card_impl_to_string_ten_diamond() {
        let card = Card {
            rank: Rank::Ten,
            suit: Suit::Diamond,
        };
        assert_eq!(card.to_string(), "Ten of Diamonds");
    }

    #[test]
    fn test_card_impl_to_string_jack_diamond() {
        let card = Card {
            rank: Rank::Jack,
            suit: Suit::Diamond,
        };
        assert_eq!(card.to_string(), "Jack of Diamonds");
    }

    #[test]
    fn test_card_impl_to_string_queen_diamond() {
        let card = Card {
            rank: Rank::Queen,
            suit: Suit::Diamond,
        };
        assert_eq!(card.to_string(), "Queen of Diamonds");
    }

    #[test]
    fn test_card_impl_to_string_king_diamond() {
        let card = Card {
            rank: Rank::King,
            suit: Suit::Diamond,
        };
        assert_eq!(card.to_string(), "King of Diamonds");
    }

    #[test]
    fn test_card_impl_to_string_ace_diamond() {
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Diamond,
        };
        assert_eq!(card.to_string(), "Ace of Diamonds");
    }

    #[test]
    fn test_card_impl_matric_value_two_club() {
        let card = Card {
            rank: Rank::Two,
            suit: Suit::Club,
        };
        assert_eq!(card.matrix_value(), 0);
    }

    #[test]
    fn test_card_impl_matric_value_three_club() {
        let card = Card {
            rank: Rank::Three,
            suit: Suit::Club,
        };
        assert_eq!(card.matrix_value(), 1);
    }

    #[test]
    fn test_card_impl_matric_value_four_club() {
        let card = Card {
            rank: Rank::Four,
            suit: Suit::Club,
        };
        assert_eq!(card.matrix_value(), 2);
    }

    #[test]
    fn test_card_impl_matric_value_five_club() {
        let card = Card {
            rank: Rank::Five,
            suit: Suit::Club,
        };
        assert_eq!(card.matrix_value(), 3);
    }

    #[test]
    fn test_card_impl_matric_value_six_club() {
        let card = Card {
            rank: Rank::Six,
            suit: Suit::Club,
        };
        assert_eq!(card.matrix_value(), 4);
    }

    #[test]
    fn test_card_impl_matric_value_seven_club() {
        let card = Card {
            rank: Rank::Seven,
            suit: Suit::Club,
        };
        assert_eq!(card.matrix_value(), 5);
    }

    #[test]
    fn test_card_impl_matric_value_eight_club() {
        let card = Card {
            rank: Rank::Eight,
            suit: Suit::Club,
        };
        assert_eq!(card.matrix_value(), 6);
    }

    #[test]
    fn test_card_impl_matric_value_nine_club() {
        let card = Card {
            rank: Rank::Nine,
            suit: Suit::Club,
        };
        assert_eq!(card.matrix_value(), 7);
    }

    #[test]
    fn test_card_impl_matric_value_ten_club() {
        let card = Card {
            rank: Rank::Ten,
            suit: Suit::Club,
        };
        assert_eq!(card.matrix_value(), 8);
    }

    #[test]
    fn test_card_impl_matric_value_jack_club() {
        let card = Card {
            rank: Rank::Jack,
            suit: Suit::Club,
        };
        assert_eq!(card.matrix_value(), 9);
    }

    #[test]
    fn test_card_impl_matric_value_queen_club() {
        let card = Card {
            rank: Rank::Queen,
            suit: Suit::Club,
        };
        assert_eq!(card.matrix_value(), 10);
    }

    #[test]
    fn test_card_impl_matric_value_king_club() {
        let card = Card {
            rank: Rank::King,
            suit: Suit::Club,
        };
        assert_eq!(card.matrix_value(), 11);
    }

    #[test]
    fn test_card_impl_matric_value_ace_club() {
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Club,
        };
        assert_eq!(card.matrix_value(), 12);
    }

    #[test]
    fn test_card_impl_matric_value_two_diamond() {
        let card = Card {
            rank: Rank::Two,
            suit: Suit::Diamond,
        };
        assert_eq!(card.matrix_value(), 13);
    }

    #[test]
    fn test_card_impl_matric_value_three_diamond() {
        let card = Card {
            rank: Rank::Three,
            suit: Suit::Diamond,
        };
        assert_eq!(card.matrix_value(), 14);
    }

    #[test]
    fn test_card_impl_matric_value_four_diamond() {
        let card = Card {
            rank: Rank::Four,
            suit: Suit::Diamond,
        };
        assert_eq!(card.matrix_value(), 15);
    }

    #[test]
    fn test_card_impl_matric_value_five_diamond() {
        let card = Card {
            rank: Rank::Five,
            suit: Suit::Diamond,
        };
        assert_eq!(card.matrix_value(), 16);
    }

    #[test]
    fn test_card_impl_matric_value_six_diamond() {
        let card = Card {
            rank: Rank::Six,
            suit: Suit::Diamond,
        };
        assert_eq!(card.matrix_value(), 17);
    }

    #[test]
    fn test_card_impl_matric_value_seven_diamond() {
        let card = Card {
            rank: Rank::Seven,
            suit: Suit::Diamond,
        };
        assert_eq!(card.matrix_value(), 18);
    }

    #[test]
    fn test_card_impl_matric_value_eight_diamond() {
        let card = Card {
            rank: Rank::Eight,
            suit: Suit::Diamond,
        };
        assert_eq!(card.matrix_value(), 19);
    }

    #[test]
    fn test_card_impl_matric_value_nine_diamond() {
        let card = Card {
            rank: Rank::Nine,
            suit: Suit::Diamond,
        };
        assert_eq!(card.matrix_value(), 20);
    }

    #[test]
    fn test_card_impl_matric_value_ten_diamond() {
        let card = Card {
            rank: Rank::Ten,
            suit: Suit::Diamond,
        };
        assert_eq!(card.matrix_value(), 21);
    }

    #[test]
    fn test_card_impl_matric_value_jack_diamond() {
        let card = Card {
            rank: Rank::Jack,
            suit: Suit::Diamond,
        };
        assert_eq!(card.matrix_value(), 22);
    }

    #[test]
    fn test_card_impl_matric_value_queen_diamond() {
        let card = Card {
            rank: Rank::Queen,
            suit: Suit::Diamond,
        };
        assert_eq!(card.matrix_value(), 23);
    }

    #[test]
    fn test_card_impl_matric_value_king_diamond() {
        let card = Card {
            rank: Rank::King,
            suit: Suit::Diamond,
        };
        assert_eq!(card.matrix_value(), 24);
    }

    #[test]
    fn test_card_impl_matric_value_ace_diamond() {
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Diamond,
        };
        assert_eq!(card.matrix_value(), 25);
    }

    #[test]
    fn test_card_impl_matric_value_two_heart() {
        let card = Card {
            rank: Rank::Two,
            suit: Suit::Heart,
        };
        assert_eq!(card.matrix_value(), 26);
    }

    #[test]
    fn test_card_impl_matric_value_three_heart() {
        let card = Card {
            rank: Rank::Three,
            suit: Suit::Heart,
        };
        assert_eq!(card.matrix_value(), 27);
    }

    #[test]
    fn test_card_impl_matric_value_four_heart() {
        let card = Card {
            rank: Rank::Four,
            suit: Suit::Heart,
        };
        assert_eq!(card.matrix_value(), 28);
    }

    #[test]
    fn test_card_impl_matric_value_five_heart() {
        let card = Card {
            rank: Rank::Five,
            suit: Suit::Heart,
        };
        assert_eq!(card.matrix_value(), 29);
    }

    #[test]
    fn test_card_impl_matric_value_six_heart() {
        let card = Card {
            rank: Rank::Six,
            suit: Suit::Heart,
        };
        assert_eq!(card.matrix_value(), 30);
    }

    #[test]
    fn test_card_impl_matric_value_seven_heart() {
        let card = Card {
            rank: Rank::Seven,
            suit: Suit::Heart,
        };
        assert_eq!(card.matrix_value(), 31);
    }

    #[test]
    fn test_card_impl_matric_value_eight_heart() {
        let card = Card {
            rank: Rank::Eight,
            suit: Suit::Heart,
        };
        assert_eq!(card.matrix_value(), 32);
    }

    #[test]
    fn test_card_impl_matric_value_nine_heart() {
        let card = Card {
            rank: Rank::Nine,
            suit: Suit::Heart,
        };
        assert_eq!(card.matrix_value(), 33);
    }

    #[test]
    fn test_card_impl_matric_value_ten_heart() {
        let card = Card {
            rank: Rank::Ten,
            suit: Suit::Heart,
        };
        assert_eq!(card.matrix_value(), 34);
    }

    #[test]
    fn test_card_impl_matric_value_jack_heart() {
        let card = Card {
            rank: Rank::Jack,
            suit: Suit::Heart,
        };
        assert_eq!(card.matrix_value(), 35);
    }

    #[test]
    fn test_card_impl_matric_value_queen_heart() {
        let card = Card {
            rank: Rank::Queen,
            suit: Suit::Heart,
        };
        assert_eq!(card.matrix_value(), 36);
    }

    #[test]
    fn test_card_impl_matric_value_king_heart() {
        let card = Card {
            rank: Rank::King,
            suit: Suit::Heart,
        };
        assert_eq!(card.matrix_value(), 37);
    }

    #[test]
    fn test_card_impl_matric_value_ace_heart() {
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Heart,
        };
        assert_eq!(card.matrix_value(), 38);
    }

    #[test]
    fn test_card_impl_matric_value_two_spade() {
        let card = Card {
            rank: Rank::Two,
            suit: Suit::Spade,
        };
        assert_eq!(card.matrix_value(), 39);
    }

    #[test]
    fn test_card_impl_matric_value_three_spade() {
        let card = Card {
            rank: Rank::Three,
            suit: Suit::Spade,
        };
        assert_eq!(card.matrix_value(), 40);
    }

    #[test]
    fn test_card_impl_matric_value_four_spade() {
        let card = Card {
            rank: Rank::Four,
            suit: Suit::Spade,
        };
        assert_eq!(card.matrix_value(), 41);
    }

    #[test]
    fn test_card_impl_matric_value_five_spade() {
        let card = Card {
            rank: Rank::Five,
            suit: Suit::Spade,
        };
        assert_eq!(card.matrix_value(), 42);
    }

    #[test]
    fn test_card_impl_matric_value_six_spade() {
        let card = Card {
            rank: Rank::Six,
            suit: Suit::Spade,
        };
        assert_eq!(card.matrix_value(), 43);
    }

    #[test]
    fn test_card_impl_matric_value_seven_spade() {
        let card = Card {
            rank: Rank::Seven,
            suit: Suit::Spade,
        };
        assert_eq!(card.matrix_value(), 44);
    }

    #[test]
    fn test_card_impl_matric_value_eight_spade() {
        let card = Card {
            rank: Rank::Eight,
            suit: Suit::Spade,
        };
        assert_eq!(card.matrix_value(), 45);
    }

    #[test]
    fn test_card_impl_matric_value_nine_spade() {
        let card = Card {
            rank: Rank::Nine,
            suit: Suit::Spade,
        };
        assert_eq!(card.matrix_value(), 46);
    }

    #[test]
    fn test_card_impl_matric_value_ten_spade() {
        let card = Card {
            rank: Rank::Ten,
            suit: Suit::Spade,
        };
        assert_eq!(card.matrix_value(), 47);
    }

    #[test]
    fn test_card_impl_matric_value_jack_spade() {
        let card = Card {
            rank: Rank::Jack,
            suit: Suit::Spade,
        };
        assert_eq!(card.matrix_value(), 48);
    }

    #[test]
    fn test_card_impl_matric_value_queen_spade() {
        let card = Card {
            rank: Rank::Queen,
            suit: Suit::Spade,
        };
        assert_eq!(card.matrix_value(), 49);
    }

    #[test]
    fn test_card_impl_matric_value_king_spade() {
        let card = Card {
            rank: Rank::King,
            suit: Suit::Spade,
        };
        assert_eq!(card.matrix_value(), 50);
    }

    #[test]
    fn test_card_impl_matric_value_ace_spade() {
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        };
        assert_eq!(card.matrix_value(), 51);
    }

    #[test]
    fn test_variant_impl_to_string_five_card_draw() {
        let variant = Variant::FiveCardDraw;
        assert_eq!(variant.to_string(), "Five-card draw");
    }

    #[test]
    fn test_variant_impl_cards_per_player_five_card_draw() {
        let variant = Variant::FiveCardDraw;
        assert_eq!(variant.cards_per_player(), 5);
    }

    #[test]
    fn test_variant_impl_num_board_cards_five_card_draw() {
        let variant = Variant::FiveCardDraw;
        assert_eq!(variant.num_board_cards(), None);
    }
}
