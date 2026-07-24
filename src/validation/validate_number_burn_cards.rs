use crate::types::Hand;

pub fn validate_number_burn_cards(hand: &Hand) -> Result<(), String> {
    match hand.variant.num_burn_cards() {
        None => {
            if hand.burn_cards.is_some() {
                return Err(format!(
                    "There should be no burn cards for {}.",
                    hand.variant.to_string(),
                ));
            }
        }
        Some(expected_num_burn_cards) => {
            let actual_num_burn_cards = hand.burn_cards.as_ref().map_or(0, |cards| cards.len());

            if actual_num_burn_cards != expected_num_burn_cards {
                return Err(format!(
                    "There should be exactly {} burn cards for {}.",
                    expected_num_burn_cards,
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
    fn test_burn_cards_given_five_card_draw() {
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
            discarded_cards: Some(vec![Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            }]),
            remaining_deck: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            }],
        };

        let result = validate_number_burn_cards(&hand).unwrap_err();
        assert_eq!(result, "There should be no burn cards for Five-card draw.",);
    }

    #[test]
    fn test_none_burn_cards_five_card_draw() {
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

        let result = validate_number_burn_cards(&hand);
        assert!(result.is_ok());
    }

    #[test]
    fn test_no_burn_cards_given_texas_holdem() {
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
            burn_cards: None,
            board: Some(vec![]),
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            }],
        };

        let result = validate_number_burn_cards(&hand).unwrap_err();
        assert_eq!(
            result,
            "There should be exactly 3 burn cards for Texas Hold 'em.",
        );
    }

    #[test]
    fn test_incorrect_number_burn_cards_texas_holdem() {
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
            burn_cards: Some(vec![
                Card {
                    rank: Rank::Two,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Diamond,
                },
            ]),
            board: Some(vec![]),
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            }],
        };

        let result = validate_number_burn_cards(&hand).unwrap_err();
        assert_eq!(
            result,
            "There should be exactly 3 burn cards for Texas Hold 'em.",
        );
    }

    #[test]
    fn test_correct_number_burn_cards_texas_holdem() {
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
            burn_cards: Some(vec![
                Card {
                    rank: Rank::Two,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Diamond,
                },
            ]),
            board: Some(vec![]),
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            }],
        };

        let result = validate_number_burn_cards(&hand);
        assert!(result.is_ok());
    }
}
