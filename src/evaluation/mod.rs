// central logic then five card separate at final loopy level?
/* use crate::{Card, Rank, Suit};
use std::collections::HashMap;

pub mod five_card_draw;

enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

// NOTE: this is only for 5 card draw etc! Need to split vs high low for example
impl HandRank {
    fn to_display(&self, cards: Vec<Card>) -> String {
        let ordered_cards = cards; // NOTE: needed here or move when needed?

        match self {
            HandRank::HighCard => String::from(format!(
                // NOTE: String::from for format isnt
                // needed as format does anyway!
                "High card {} with {}, {}, {}, and {} kickers",
                // ordered_cards[0].to_display(false),
                // // ordered_cards[1].to_display(false),
                // ordered_cards[2].to_display(false),
                // ordered_cards[3].to_display(false),
                // ordered_cards[4].to_display(false),
                "placeholder",
                "placeholder",
                "placeholder",
                "placeholder",
                "placeholder",
            )),
            HandRank::Pair => String::from(format!(
                "Pair of {} with {}, {}, and {} kickers",
                "placeholder", /*to_display(true)*/
                "placeholder",
                "placeholder",
                "placeholder",
            )),
            HandRank::TwoPair => String::from(format!(
                "Two pair, {} over {} with {} kicker",
                "placeholder", /*to_display(true)*/
                "placeholder", /*to_display(true)*/
                "placeholder"
            )),
            HandRank::ThreeOfAKind => String::from(format!(
                "Three of a kind, {} and {}, {} kickers",
                "placeholder", /*to_display(true)*/ "placeholder", "placeholder"
            )),
            HandRank::Straight => String::from(format!(
                "{} high straight",
                /*"Wheel"*/
                /*"Broadway"*/
                "placeholder"
            )),
            HandRank::Flush => String::from(format!(
                "{} high flush, with {}, {}, {}, {}",
                "placeholder", "placeholder", "placeholder", "placeholder", "placeholder"
            )),
            HandRank::FullHouse => String::from(format!(
                "Full house, {} full of {}",
                "placeholder", /*to_display(true)*/ "placeholder" /*to_display(true)*/
            )),
            HandRank::FourOfAKind => {
                format!(
                    // NOTE: or can figure out from value of evaluation? = do evaluation first
                    "Four of a kind, {} with {} kicker",
                    "placeholder", /*to_display(true)*/ "placeholder"
                )
            }
            HandRank::StraightFlush => String::from(format!(
                "{} high straight flush",
                "placeholder" /*"Royal flush"*/
            )),
            // NOTE: error possible?
        }
    }
}

/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_card_to_display() {
        let cards = vec![
            Card {
                suit: Suit::Heart,
                rank: Rank::Two,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Three,
            },
            Card {
                suit: Suit::Spade,
                rank: Rank::King,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Jack,
            },
        ];
        assert_eq!(
            HandRank::HighCard.to_display(cards),
            "High card Ace with King, Queen, Three, and Two kickers"
        );
    }

    #[test]
    fn test_pair_to_display() {
        let cards = vec![
            Card {
                suit: Suit::Heart,
                rank: Rank::Two,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Two,
            },
            Card {
                suit: Suit::Spade,
                rank: Rank::King,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Jack,
            },
        ];
        assert_eq!(
            HandRank::Pair.to_display(cards),
            "Pair of Twos with Ace, King, and Jack kickers"
        );
    }

    #[test]
    fn test_two_pair_to_display() {
        let cards = vec![
            Card {
                suit: Suit::Heart,
                rank: Rank::Six,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Six,
            },
            Card {
                suit: Suit::Spade,
                rank: Rank::King,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::King,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Jack,
            },
        ];
        assert_eq!(
            HandRank::TwoPair.to_display(cards),
            "Two pair, Kings over Sixes with Jack kicker",
        );
    }

    #[test]
    fn test_three_of_a_kind_to_display() {
        let cards = vec![
            Card {
                suit: Suit::Heart,
                rank: Rank::Two,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Two,
            },
            Card {
                suit: Suit::Spade,
                rank: Rank::Two,
            },
            Card {
                suit: Suit::Heart,
                rank: Rank::King,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Jack,
            },
        ];
        assert_eq!(
            HandRank::ThreeOfAKind.to_display(cards),
            "Three of a kind, Twos and King, Jack kickers",
        );
    }

    #[test]
    fn test_straight_to_display() {
        let cards = vec![
            Card {
                suit: Suit::Heart,
                rank: Rank::Two,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Three,
            },
            Card {
                suit: Suit::Spade,
                rank: Rank::Four,
            },
            Card {
                suit: Suit::Heart,
                rank: Rank::Five,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Six,
            },
        ];
        assert_eq!(HandRank::Straight.to_display(cards), "Six high straight",);
    }

    #[test]
    fn test_wheel_straight_to_display() {
        let cards = vec![
            Card {
                suit: Suit::Heart,
                rank: Rank::Two,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Three,
            },
            Card {
                suit: Suit::Spade,
                rank: Rank::Four,
            },
            Card {
                suit: Suit::Heart,
                rank: Rank::Five,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Ace,
            },
        ];
        assert_eq!(HandRank::Straight.to_display(cards), "Wheel",);
    }

    #[test]
    fn test_broadway_straight_to_display() {
        let cards = vec![
            Card {
                suit: Suit::Heart,
                rank: Rank::King,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Spade,
                rank: Rank::Jack,
            },
            Card {
                suit: Suit::Heart,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Six,
            },
        ];
        assert_eq!(HandRank::Straight.to_display(cards), "Broadway",);
    }

    #[test]
    fn test_flush_to_display() {
        let cards = vec![
            Card {
                suit: Suit::Heart,
                rank: Rank::King,
            },
            Card {
                suit: Suit::Heart,
                rank: Rank::Seven,
            },
            Card {
                suit: Suit::Heart,
                rank: Rank::Jack,
            },
            Card {
                suit: Suit::Heart,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Heart,
                rank: Rank::Six,
            },
        ];
        assert_eq!(
            HandRank::Flush.to_display(cards),
            "King high flush, with Jack, Ten, Seven, Six"
        );
    }

    #[test]
    fn test_full_house_to_display() {
        let cards = vec![
            Card {
                suit: Suit::Spade,
                rank: Rank::Seven,
            },
            Card {
                suit: Suit::Heart,
                rank: Rank::Seven,
            },
            Card {
                suit: Suit::Spade,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Heart,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Ten,
            },
        ];
        assert_eq!(
            HandRank::FullHouse.to_display(cards),
            "Full house, Tens full of Sevens"
        );
    }

    #[test]
    fn test_four_of_a_kind_house_to_display() {
        let cards = vec![
            Card {
                suit: Suit::Spade,
                rank: Rank::Seven,
            },
            Card {
                suit: Suit::Diamond,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Spade,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Heart,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Ten,
            },
        ];
        assert_eq!(
            HandRank::FourOfAKind.to_display(cards),
            "Four of a kind, Tens with Seven kicker"
        );
    }

    #[test]
    fn test_straight_flush_to_display() {
        let cards = vec![
            Card {
                suit: Suit::Diamond,
                rank: Rank::Nine,
            },
            Card {
                suit: Suit::Diamond,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Diamond,
                rank: Rank::Jack,
            },
            Card {
                suit: Suit::Diamond,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Diamond,
                rank: Rank::King,
            },
        ];
        assert_eq!(
            HandRank::StraightFlush.to_display(cards),
            "King high straight flush"
        );
    }

    #[test]
    fn test_royal_flush_to_display() {
        let cards = vec![
            Card {
                suit: Suit::Diamond,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Diamond,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Diamond,
                rank: Rank::Jack,
            },
            Card {
                suit: Suit::Diamond,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Diamond,
                rank: Rank::King,
            },
        ];
        assert_eq!(HandRank::StraightFlush.to_display(cards), "Royal flush");
    }
} */ */
