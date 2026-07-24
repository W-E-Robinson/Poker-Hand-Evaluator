use crate::{
    evaluation::evaluate,
    types::{Error, ErrorType, Evaluation, Hand},
    validation::validate,
};

mod constants;
pub mod evaluation;
pub mod types;
mod validation;

pub fn evaluate_hand(hand: Hand) -> Result<Evaluation, Error> {
    if let Err(err) = validate(&hand) {
        return Err(Error {
            id: hand.id,
            error_type: ErrorType::Validation,
            message: err,
        });
    }

    Ok(evaluate(hand))
}

#[cfg(test)]
mod tests {
    use crate::types::{Card, Player, PlayerEval, Rank, Suit, Variant};

    use super::*;

    #[test]
    fn test_validation_error() {
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

        let result = evaluate_hand(hand).unwrap_err();
        assert_eq!(
            result,
            Error {
                id: String::from("hand-id"),
                error_type: ErrorType::Validation,
                message: String::from("At least one player must be provided."),
            }
        );
    }

    #[test]
    fn test_evaluation_five_card_draw() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            board: None,
            burn_cards: None,
            discarded_cards: Some(vec![]),
            remaining_deck: vec![
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
            ],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ten,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Jack,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Queen,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::King,
                            suit: Suit::Diamond,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ten,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Jack,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Queen,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::King,
                            suit: Suit::Club,
                        },
                    ],
                },
            ],
        };
        assert_eq!(
            evaluate_hand(hand),
            Ok(Evaluation {
                id: String::from("hand-id"),
                players: vec![
                    PlayerEval {
                        id: String::from("player-1-id"),
                        hand: String::from("Royal Flush"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Broadway"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Royal Flush"),
            })
        );
    }

    #[test]
    fn test_evaluation_texas_hold_em() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Club,
                },
            ]),
            burn_cards: Some(vec![
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Ten,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Heart,
                },
            ]),
            discarded_cards: None,
            remaining_deck: vec![
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
                    rank: Rank::Jack,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Club,
                },
            ],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::King,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Diamond,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ten,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Club,
                        },
                    ],
                },
            ],
        };
        assert_eq!(
            evaluate_hand(hand),
            Ok(Evaluation {
                id: String::from("hand-id"),
                players: vec![
                    PlayerEval {
                        id: String::from("player-1-id"),
                        hand: String::from("Royal Flush"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Pair"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Royal Flush"),
            })
        );
    }
}
