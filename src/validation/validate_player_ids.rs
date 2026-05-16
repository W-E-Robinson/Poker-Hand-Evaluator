use std::collections::HashSet;

use crate::types::Hand;

pub fn validate_player_ids(hand: &Hand) -> Result<(), String> {
    if hand.players.len() == 0 {
        return Err(String::from("At least one player must be provided."));
    }

    let mut given_ids: HashSet<String> = HashSet::new();
    let mut repeated_ids: HashSet<String> = HashSet::new();

    for player in hand.players.iter() {
        if given_ids.get(&player.id) == None {
            given_ids.insert(player.id.clone());
        } else {
            repeated_ids.insert(player.id.clone());
        }
    }

    if !repeated_ids.is_empty() {
        return Err(String::from("All players ids must be unique."));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::types::{Card, Player, Rank, Suit, Variant};

    use super::*;

    #[test]
    fn test_no_players_given() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            players: vec![],
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

        let result = validate_player_ids(&hand).unwrap_err();
        assert_eq!(result, "At least one player must be provided.",);
    }

    #[test]
    fn test_duplicate_ids_given() {
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
            ],
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

        let result = validate_player_ids(&hand).unwrap_err();
        assert_eq!(result, "All players ids must be unique.",);
    }

    #[test]
    fn test_valid_player_ids() {
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
            ],
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

        let result = validate_player_ids(&hand);
        assert!(result.is_ok());
    }
}
