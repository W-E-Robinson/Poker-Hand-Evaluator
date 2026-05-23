use crate::types::{Hand, Variant};

pub fn validate_number_board_cards(hand: &Hand) -> Result<(), String> {
    match hand.variant {
        Variant::FiveCardDraw => {
            if hand.board.is_some() {
                return Err(format!(
                    "There should be no board for {}.",
                    hand.variant.to_string(),
                ));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::types::{Card, Player, Rank, Suit};

    use super::*;

    #[test]
    fn test_board_given_five_card_draw() {
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
            board: Some(vec![Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            }]),
            burn_cards: None,
            discarded_cards: Some(vec![Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            }]),
            remaining_deck: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            }],
        };

        let result = validate_number_board_cards(&hand).unwrap_err();
        assert_eq!(result, "There should be no board for Five-card draw.",);
    }

    #[test]
    fn test_none_board_five_card_draw() {
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

        let result = validate_number_board_cards(&hand);
        assert!(result.is_ok());
    }
}
