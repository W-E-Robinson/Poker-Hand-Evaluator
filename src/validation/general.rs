use std::collections::HashSet;

use crate::types::{Card, Hand, Variant};

fn validate_no_repeated_cards(hand: &Hand) -> Result<(), String> {
    let mut given_cards: HashSet<Card> = HashSet::new();
    let mut repeated_cards: HashSet<Card> = HashSet::new();

    for player in hand.players.iter() {
        for card in player.cards.iter() {
            if given_cards.get(card) == None {
                given_cards.insert(card.clone());
            } else {
                repeated_cards.insert(card.clone());
            }
        }
    }

    if let Some(board) = &hand.board {
        for card in board {
            if !given_cards.insert(card.clone()) {
                repeated_cards.insert(card.clone());
            }
        }
    }

    if repeated_cards.is_empty() {
        Ok(())
    } else {
        let mut cards: Vec<String> = repeated_cards.iter().map(|c| c.to_string()).collect();
        cards.sort();

        Err(format!("Repeated cards: {}", cards.join(", ")))
    }
}

fn validate_board(hand: &Hand) -> Result<(), String> {
    match hand.variant {
        Variant::FiveCardDraw => {
            if hand.board.is_some() {
                return Err(format!(
                    "There should be no board for {}",
                    hand.variant.to_string(),
                ));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::types::{Player, Rank, Suit, Variant};

    use super::*;

    #[test]
    fn test_validation_error_on_repeated_cards_five_card_draw() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            players: vec![
                Player {
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
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
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
                },
            ],
            board: None,
        };

        let result = validate_no_repeated_cards(&hand).unwrap_err();
        assert_eq!(
            result,
            "Repeated cards: Jack of Hearts, Queen of Hearts, Ten of Hearts"
        );
    }

    #[test]
    fn test_valid_when_no_repeated_cards_five_card_draw() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            players: vec![
                Player {
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
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
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
                    ],
                },
            ],
            board: None,
        };

        let result = validate_no_repeated_cards(&hand);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_when_no_board_given_five_card_draw() {
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
        };

        let result = validate_board(&hand);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_when_board_given_five_card_draw() {
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
            board: Some(vec![
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
            ]),
        };

        let result = validate_board(&hand).unwrap_err();
        assert_eq!(result, "There should be no board for Five-card draw");
    }
}
