use std::collections::HashSet;

use crate::types::{Card, Hand};

pub fn validate_no_repeated_cards(hand: &Hand) -> Result<(), String> {
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

    if let Some(burn_cards) = &hand.burn_cards {
        for card in burn_cards {
            if !given_cards.insert(card.clone()) {
                repeated_cards.insert(card.clone());
            }
        }
    }

    if let Some(discarded_cards) = &hand.discarded_cards {
        for card in discarded_cards {
            if !given_cards.insert(card.clone()) {
                repeated_cards.insert(card.clone());
            }
        }
    }

    for card in hand.remaining_deck.iter() {
        if !given_cards.insert(card.clone()) {
            repeated_cards.insert(card.clone());
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

#[cfg(test)]
mod tests {
    use crate::types::{Player, Rank, Suit, Variant};

    use super::*;

    #[test]
    fn test_error_repeated_cards_five_card_draw() {
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
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Ten,
                suit: Suit::Heart,
            }],
        };

        let result = validate_no_repeated_cards(&hand).unwrap_err();
        assert_eq!(
            result,
            "Repeated cards: Jack of Hearts, Queen of Hearts, Ten of Hearts"
        );
    }

    #[test]
    fn test_valid_no_repeated_cards_five_card_draw() {
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
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Ten,
                suit: Suit::Diamond,
            }],
        };

        let result = validate_no_repeated_cards(&hand);
        assert!(result.is_ok(),);
    }

    #[test]
    fn test_error_repeated_cards_texas_holdem() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
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
                    ],
                },
            ],
            board: Some(vec![
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Heart,
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
            ]),
            burn_cards: Some(vec![]),
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Two,
                suit: Suit::Spade,
            }],
        };

        let result = validate_no_repeated_cards(&hand).unwrap_err();
        assert_eq!(result, "Repeated cards: Ace of Hearts");
    }

    #[test]
    fn test_valid_no_repeated_cards_texas_holdem() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
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
                    ],
                },
            ],
            board: Some(vec![
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
            ]),
            burn_cards: Some(vec![
                Card {
                    rank: Rank::Two,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Spade,
                },
            ]),
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Five,
                suit: Suit::Spade,
            }],
        };

        let result = validate_no_repeated_cards(&hand);
        assert!(result.is_ok(),);
    }

    #[test]
    fn test_error_repeated_cards_four_card_omaha_hi() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FourCardOmahaHi,
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
                            suit: Suit::Heart,
                        },
                    ],
                },
            ],
            board: Some(vec![
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
                Card {
                    rank: Rank::Five,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Diamond,
                },
            ]),
            burn_cards: Some(vec![]),
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Seven,
                suit: Suit::Diamond,
            }],
        };

        let result = validate_no_repeated_cards(&hand).unwrap_err();
        assert_eq!(result, "Repeated cards: Jack of Hearts");
    }

    #[test]
    fn test_valid_no_repeated_cards_four_card_omaha_hi() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FourCardOmahaHi,
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
                    ],
                },
            ],
            board: Some(vec![
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
                Card {
                    rank: Rank::Five,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Diamond,
                },
            ]),
            burn_cards: Some(vec![
                Card {
                    rank: Rank::Two,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Spade,
                },
            ]),
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Five,
                suit: Suit::Spade,
            }],
        };

        let result = validate_no_repeated_cards(&hand);
        assert!(result.is_ok(),);
    }
}
