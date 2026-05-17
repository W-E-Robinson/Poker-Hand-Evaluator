use crate::types::Hand;

const NUMBER_CARDS_DECK: usize = 52;

pub fn validate_number_total_cards(hand: &Hand) -> Result<(), String> {
    let mut total_num_cards = 0;

    for player in hand.players.iter() {
        total_num_cards += player.cards.len();
    }

    if let Some(board) = &hand.board {
        total_num_cards += board.len();
    }

    if let Some(burn_cards) = &hand.burn_cards {
        total_num_cards += burn_cards.len();
    }

    if let Some(discarded_cards) = &hand.discarded_cards {
        total_num_cards += discarded_cards.len();
    }

    total_num_cards += hand.remaining_deck.len();

    if total_num_cards != NUMBER_CARDS_DECK {
        return Err(format!(
            "Exactly {} cards must be provided in total.",
            NUMBER_CARDS_DECK
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::types::{Card, Player, Rank, Suit, Variant};

    use super::*;

    #[test]
    fn test_invalid_number_cards_five_card_draw() {
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
            board: None,
            burn_cards: None,
            discarded_cards: Some(vec![Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            }]),
            remaining_deck: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Club,
            }],
        };

        let result = validate_number_total_cards(&hand).unwrap_err();
        assert_eq!(result, "Exactly 52 cards must be provided in total.",);
    }

    #[test]
    fn test_valid_number_cards_five_card_draw() {
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
            board: None,
            burn_cards: None,
            discarded_cards: Some(vec![
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Diamond,
                },
            ]),
            remaining_deck: vec![
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Club,
                },
            ],
        };

        let result = validate_number_total_cards(&hand);
        assert!(result.is_ok());
    }
}
