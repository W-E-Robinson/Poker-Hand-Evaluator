use std::collections::HashSet;

use crate::{Card, Hand, Variant};

pub fn validate(hand: &Hand) -> Result<(), String> {
    validate_no_repeated_cards(&hand)?;
    validate_board(&hand)?;
    validate_number_players(&hand)?;
    validate_players_displays(&hand)?;
    validate_players_cards(&hand)?;

    Ok(())
}

fn validate_no_repeated_cards(hand: &Hand) -> Result<(), String> {
    let mut given_cards: HashSet<Card> = HashSet::new();

    for player in hand.players.iter() {
        for card in player.cards.iter() {
            if given_cards.get(card) == None {
                given_cards.insert(card.clone());
            } else {
                return Err(format!("Repeated card: {}", card.clone().to_card_string()));
            }
        }
    }

    Ok(())
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
        Variant::TexasHoldem => {
            if hand.board_len() != hand.variant.number_board_cards() {
                return Err(format!(
                    "The board must be {} cards long for {}",
                    hand.variant.number_board_cards(),
                    hand.variant.to_string(),
                ));
            }
        }
    }

    Ok(())
}

fn validate_number_players(hand: &Hand) -> Result<(), String> {
    if hand.players.len() == 0 {
        return Err(String::from("At least one player must be provided"));
    }

    let max_players = (52 - hand.variant.number_board_cards() - hand.variant.number_burn_cards())
        / hand.variant.number_player_cards();
    let number_players = hand.players.iter().len();
    if number_players > max_players {
        return Err(format!(
            "Maximum number of players is {}, {} provided",
            max_players, number_players,
        ));
    }

    Ok(())
}

fn validate_players_displays(hand: &Hand) -> Result<(), String> {
    let mut displays: HashSet<String> = HashSet::new();

    for player in hand.players.iter() {
        if displays.get(&player.name) == None {
            displays.insert(player.name.clone());
        } else {
            return Err(format!("Repeated player display: {}", player.name,));
        }
    }

    Ok(())
}

fn validate_players_cards(hand: &Hand) -> Result<(), String> {
    let expected_number_cards = hand.variant.number_player_cards();
    for player in hand.players.iter() {
        if player.cards.len() != expected_number_cards {
            return Err(format!(
                "Expected number of cards per player is {} for {}, {} provided for {}",
                expected_number_cards,
                hand.variant.to_string(),
                player.cards.len(),
                player.name,
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Player, Rank, Suit};

    use super::*;

    #[test]
    fn test_errors_when_repeated_cards() {
        let hand = Hand {
            variant: Variant::FiveCardDraw,
            players: vec![Player {
                name: String::from("Player 1"),
                cards: vec![
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Heart,
                    },
                ],
            }],
            board: None,
        };

        let error = validate(&hand).unwrap_err();
        assert_eq!(error, "Repeated card: Ace of Hearts");
    }

    #[test]
    fn test_errors_when_board_given_for_five_card_draw() {
        let hand = Hand {
            variant: Variant::FiveCardDraw,
            players: vec![Player {
                name: String::from("Player 1"),
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
                suit: Suit::Club,
            }]),
        };

        let error = validate(&hand).unwrap_err();
        assert_eq!(error, "There should be no board for Five-card draw");
    }

    #[test]
    fn test_errors_when_board_given_for_texas_holdem_is_wrong_length() {
        let hand = Hand {
            variant: Variant::TexasHoldem,
            players: vec![Player {
                name: String::from("Player 1"),
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
                suit: Suit::Club,
            }]),
        };

        let error = validate(&hand).unwrap_err();
        assert_eq!(error, "The board must be 5 cards long for Texas hold 'em");
    }

    #[test]
    fn test_errors_when_no_players_provided() {
        let hand = Hand {
            variant: Variant::FiveCardDraw,
            players: vec![],
            board: None,
        };

        let error = validate(&hand).unwrap_err();
        assert_eq!(error, "At least one player must be provided");
    }

    #[test]
    fn test_errors_when_players_displays_repeated() {
        let hand = Hand {
            variant: Variant::TexasHoldem,
            players: vec![
                Player {
                    name: String::from("Player 1"),
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
                    name: String::from("Player 1"),
                    cards: vec![
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
            ],
            board: Some(vec![
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
            ]),
        };

        let error = validate(&hand).unwrap_err();
        assert_eq!(error, "Repeated player display: Player 1");
    }

    #[test]
    fn test_errors_when_player_wrong_number_cards() {
        let hand = Hand {
            variant: Variant::TexasHoldem,
            players: vec![Player {
                name: String::from("Player 1"),
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
                        rank: Rank::King,
                        suit: Suit::Spade,
                    },
                ],
            }],
            board: Some(vec![
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
            ]),
        };

        let error = validate(&hand).unwrap_err();
        assert_eq!(
            error,
            "Expected number of cards per player is 2 for Texas hold 'em, 3 provided for Player 1"
        );
    }

    #[test]
    fn test_ok_when_supplied_hand_is_valid_five_card_draw() {
        let hand = Hand {
            variant: Variant::FiveCardDraw,
            players: vec![Player {
                name: String::from("Player 1"),
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
                ],
            }],
            board: None,
        };

        let result = validate(&hand);
        assert!(result.is_ok());
    }
}
