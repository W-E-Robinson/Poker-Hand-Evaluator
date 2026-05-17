use std::fmt;

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

pub struct Hand {
    pub id: String,
    pub variant: Variant,
    pub players: Vec<Player>,
    pub board: Option<Vec<Card>>,
    pub burn_cards: Option<Vec<Card>>,
    pub discarded_cards: Option<Vec<Card>>,
    pub remaining_deck: Vec<Card>,
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
    fn test_variant_impl_to_string_five_card_draw() {
        let variant = Variant::FiveCardDraw;
        assert_eq!(variant.to_string(), "Five-card draw");
    }
}
