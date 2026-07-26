use std::collections::HashSet;

use crate::types::Hand;

pub fn validate_number_player_cards(hand: &Hand) -> Result<(), String> {
    let mut invalid_players: HashSet<String> = HashSet::new();
    let expected_cards_per_player = hand.variant.cards_per_player();

    for player in hand.players.iter() {
        if player.cards.iter().len() != expected_cards_per_player {
            invalid_players.insert(player.id.clone());
        }
    }

    if !invalid_players.is_empty() {
        let mut players: Vec<String> = invalid_players.iter().map(|c| c.to_string()).collect();
        players.sort();

        return Err(format!("Each provided player should have {} cards for {}. Players with the incorrect number of cards: {}.",
               expected_cards_per_player, hand.variant.to_string(), players.join(", ") ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::types::{Card, Player, Rank, Suit, Variant};

    use super::*;

    #[test]
    fn test_invalid_cards_per_player_five_card_draw() {
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
                    ],
                },
                Player {
                    id: String::from("player-3-id"),
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

        let result = validate_number_player_cards(&hand).unwrap_err();
        assert_eq!(result, "Each provided player should have 5 cards for Five-card draw. Players with the incorrect number of cards: player-2-id, player-3-id.",);
    }

    #[test]
    fn test_valid_cards_per_player_five_card_draw() {
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
                    ],
                },
            ],
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

        let result = validate_number_player_cards(&hand);
        assert!(result.is_ok())
    }

    #[test]
    fn test_invalid_cards_per_player_texas_holdem() {
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
                    cards: vec![Card {
                        rank: Rank::Ace,
                        suit: Suit::Diamond,
                    }],
                },
                Player {
                    id: String::from("player-3-id"),
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
                    ],
                },
            ],
            burn_cards: Some(vec![]),
            board: Some(vec![]),
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Spade,
            }],
        };

        let result = validate_number_player_cards(&hand).unwrap_err();
        assert_eq!(result, "Each provided player should have 2 cards for Texas Hold 'em. Players with the incorrect number of cards: player-2-id, player-3-id.",);
    }

    #[test]
    fn test_valid_cards_per_player_texas_holdem() {
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
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::King,
                            suit: Suit::Diamond,
                        },
                    ],
                },
            ],
            burn_cards: Some(vec![]),
            board: Some(vec![]),
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Spade,
            }],
        };

        let result = validate_number_player_cards(&hand);
        assert!(result.is_ok())
    }

    #[test]
    fn test_invalid_cards_per_player_four_card_omaha_hi() {
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
                    cards: vec![Card {
                        rank: Rank::Ace,
                        suit: Suit::Diamond,
                    }],
                },
                Player {
                    id: String::from("player-3-id"),
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
                    ],
                },
            ],
            burn_cards: Some(vec![]),
            board: Some(vec![]),
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Spade,
            }],
        };

        let result = validate_number_player_cards(&hand).unwrap_err();
        assert_eq!(result, "Each provided player should have 4 cards for Omaha Hold 'em. Players with the incorrect number of cards: player-2-id, player-3-id.",);
    }

    #[test]
    fn test_valid_cards_per_player_four_card_omaha_hi() {
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
                    ],
                },
            ],
            burn_cards: Some(vec![]),
            board: Some(vec![]),
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Spade,
            }],
        };

        let result = validate_number_player_cards(&hand);
        assert!(result.is_ok())
    }
}
