use crate::types::{Hand, Variant};

pub fn validate_number_discarded_cards(hand: &Hand) -> Result<(), String> {
    match hand.variant {
        Variant::FiveCardDraw => {
            if hand.discarded_cards.is_none() {
                return Err(format!(
                    "There should be discarded cards for {}. There can be zero provided.",
                    hand.variant.to_string(),
                ));
            }
        }
        Variant::TexasHoldem | Variant::FourCardOmahaHi => {
            if hand.discarded_cards.is_some() {
                return Err(format!(
                    "There should be no discarded cards for {}.",
                    hand.variant.to_string(),
                ));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::types::{Card, Player, Rank, Suit, Variant};

    use super::*;

    #[test]
    fn test_no_discarded_cards_given_five_card_draw() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            players: vec![Player {
                id: String::from("player-1-id"),
                cards: vec![
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::King,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Queen,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Jack,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Ten,
                        suit: Suit::Heart,
                    },
                ],
            }],
            burn_cards: Some(vec![Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            }]),
            board: None,
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            }],
        };

        let result = validate_number_discarded_cards(&hand).unwrap_err();
        assert_eq!(
            result,
            "There should be discarded cards for Five-card draw. There can be zero provided.",
        );
    }

    #[test]
    fn test_discarded_cards_five_card_draw() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            players: vec![Player {
                id: String::from("player-1-id"),
                cards: vec![
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::King,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Queen,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Jack,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Ten,
                        suit: Suit::Heart,
                    },
                ],
            }],
            burn_cards: None,
            board: None,
            discarded_cards: Some(vec![]),
            remaining_deck: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            }],
        };

        let result = validate_number_discarded_cards(&hand);
        assert!(result.is_ok());
    }

    #[test]
    fn test_discarded_cards_given_texas_holdem() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            players: vec![Player {
                id: String::from("player-1-id"),
                cards: vec![
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::King,
                        suit: Suit::Heart,
                    },
                ],
            }],
            burn_cards: Some(vec![]),
            board: Some(vec![]),
            discarded_cards: Some(vec![Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            }]),
            remaining_deck: vec![Card {
                rank: Rank::King,
                suit: Suit::Diamond,
            }],
        };

        let result = validate_number_discarded_cards(&hand).unwrap_err();
        assert_eq!(
            result,
            "There should be no discarded cards for Texas Hold 'em.",
        );
    }

    #[test]
    fn test_none_discarded_cards_texas_holdem() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            players: vec![Player {
                id: String::from("player-1-id"),
                cards: vec![
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::King,
                        suit: Suit::Heart,
                    },
                ],
            }],
            burn_cards: Some(vec![]),
            board: Some(vec![]),
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            }],
        };

        let result = validate_number_discarded_cards(&hand);
        assert!(result.is_ok());
    }

    #[test]
    fn test_discarded_cards_given_four_card_omaha_hi() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FourCardOmahaHi,
            players: vec![Player {
                id: String::from("player-1-id"),
                cards: vec![
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::King,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Queen,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Jack,
                        suit: Suit::Heart,
                    },
                ],
            }],
            burn_cards: Some(vec![]),
            board: Some(vec![]),
            discarded_cards: Some(vec![Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            }]),
            remaining_deck: vec![Card {
                rank: Rank::King,
                suit: Suit::Diamond,
            }],
        };

        let result = validate_number_discarded_cards(&hand).unwrap_err();
        assert_eq!(
            result,
            "There should be no discarded cards for Omaha Hold 'em.",
        );
    }

    #[test]
    fn test_none_discarded_cards_four_card_omaha_hi() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FourCardOmahaHi,
            players: vec![Player {
                id: String::from("player-1-id"),
                cards: vec![
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::King,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Queen,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Jack,
                        suit: Suit::Heart,
                    },
                ],
            }],
            burn_cards: Some(vec![]),
            board: Some(vec![]),
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            }],
        };

        let result = validate_number_discarded_cards(&hand);
        assert!(result.is_ok());
    }
}
