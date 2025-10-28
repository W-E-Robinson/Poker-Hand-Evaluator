use crate::{Card, Evaluation, Hand, PlayerEval, Rank, Suit};

pub fn evaluation(hand: Hand) -> Evaluation {
    Evaluation {
        variant: hand.variant,
        players: vec![PlayerEval {
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
                    rank: Rank::Three,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Heart,
                },
            ],
            winner: true,
            display: String::from("Hello World"),
        }],
        board: hand.board,
    }
}

#[cfg(test)]
mod tests {
    use crate::{Card, Evaluation, Hand, Player, PlayerEval, Rank, Suit, Variant};

    use super::*;

    #[test]
    fn test_five_card_draw_one_player_evaluation_high_card() {
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
                        rank: Rank::Three,
                        suit: Suit::Club,
                    },
                    Card {
                        rank: Rank::Two,
                        suit: Suit::Heart,
                    },
                ],
            }],
            board: None,
        };

        assert_eq!(
            evaluation(hand),
            Evaluation {
                variant: Variant::FiveCardDraw,
                players: vec![PlayerEval {
                    name: String::from("Player 1"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Heart,
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
                            rank: Rank::Three,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Heart,
                        },
                    ],
                    winner: true,
                    display: String::from("High card Ace with King, Queen, Three, and Two kickers"),
                }],
                board: None,
            }
        );
    }

    #[test]
    fn test_five_card_draw_one_player_evaluation_pair() {
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
                        suit: Suit::Club,
                    },
                    Card {
                        rank: Rank::Queen,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Three,
                        suit: Suit::Club,
                    },
                    Card {
                        rank: Rank::Two,
                        suit: Suit::Heart,
                    },
                ],
            }],
            board: None,
        };

        assert_eq!(
            evaluation(hand),
            Evaluation {
                variant: Variant::FiveCardDraw,
                players: vec![PlayerEval {
                    name: String::from("Player 1"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Heart,
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
                            rank: Rank::Three,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Heart,
                        },
                    ],
                    winner: true,
                    display: String::from("Pair of Aces with King, Queen, and Three kickers"),
                }],
                board: None,
            }
        );
    }

    #[test]
    fn test_five_card_draw_one_player_evaluation_two_pair() {
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
                        suit: Suit::Club,
                    },
                    Card {
                        rank: Rank::King,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Club,
                    },
                    Card {
                        rank: Rank::Two,
                        suit: Suit::Heart,
                    },
                ],
            }],
            board: None,
        };

        assert_eq!(
            evaluation(hand),
            Evaluation {
                variant: Variant::FiveCardDraw,
                players: vec![PlayerEval {
                    name: String::from("Player 1"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::King,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::King,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Heart,
                        },
                    ],
                    winner: true,
                    display: String::from("Two pair, Aces over Kings with Two kicker"),
                }],
                board: None,
            }
        );
    }

    #[test]
    fn test_five_card_draw_one_player_evaluation_three_of_a_kind() {
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
                        suit: Suit::Club,
                    },
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Spade,
                    },
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Club,
                    },
                    Card {
                        rank: Rank::Two,
                        suit: Suit::Heart,
                    },
                ],
            }],
            board: None,
        };

        assert_eq!(
            evaluation(hand),
            Evaluation {
                variant: Variant::FiveCardDraw,
                players: vec![PlayerEval {
                    name: String::from("Player 1"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::King,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Heart,
                        },
                    ],
                    winner: true,
                    display: String::from("Three of a kind, Aces with King, Jack kickers"),
                }],
                board: None,
            }
        );
    }

    #[test]
    fn test_five_card_draw_one_player_evaluation_straight() {
        let hand = Hand {
            variant: Variant::FiveCardDraw,
            players: vec![Player {
                name: String::from("Player 1"),
                cards: vec![
                    Card {
                        rank: Rank::Jack,
                        suit: Suit::Club,
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
                ],
            }],
            board: None,
        };

        assert_eq!(
            evaluation(hand),
            Evaluation {
                variant: Variant::FiveCardDraw,
                players: vec![PlayerEval {
                    name: String::from("Player 1"),
                    cards: vec![
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
                    ],
                    winner: true,
                    display: String::from("Jack high straight"),
                }],
                board: None,
            }
        );
    }

    #[test]
    fn test_five_card_draw_one_player_evaluation_flush() {
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
                        rank: Rank::Seven,
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
            }],
            board: None,
        };

        assert_eq!(
            evaluation(hand),
            Evaluation {
                variant: Variant::FiveCardDraw,
                players: vec![PlayerEval {
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
                            rank: Rank::Seven,
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
                    winner: true,
                    display: String::from("Ace high flush, with Jing, Seven, Three, Two"),
                }],
                board: None,
            }
        );
    }

    #[test]
    fn test_five_card_draw_one_player_evaluation_full_house() {
        let hand = Hand {
            variant: Variant::FiveCardDraw,
            players: vec![Player {
                name: String::from("Player 1"),
                cards: vec![
                    Card {
                        rank: Rank::Five,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Five,
                        suit: Suit::Club,
                    },
                    Card {
                        rank: Rank::Five,
                        suit: Suit::Spade,
                    },
                    Card {
                        rank: Rank::Four,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Four,
                        suit: Suit::Club,
                    },
                ],
            }],
            board: None,
        };

        assert_eq!(
            evaluation(hand),
            Evaluation {
                variant: Variant::FiveCardDraw,
                players: vec![PlayerEval {
                    name: String::from("Player 1"),
                    cards: vec![
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Club,
                        },
                    ],
                    winner: true,
                    display: String::from("Full house, Fives full of Fours"),
                }],
                board: None,
            }
        );
    }

    #[test]
    fn test_five_card_draw_one_player_evaluation_four_of_a_kind() {
        let hand = Hand {
            variant: Variant::FiveCardDraw,
            players: vec![Player {
                name: String::from("Player 1"),
                cards: vec![
                    Card {
                        rank: Rank::Five,
                        suit: Suit::Heart,
                    },
                    Card {
                        rank: Rank::Five,
                        suit: Suit::Club,
                    },
                    Card {
                        rank: Rank::Five,
                        suit: Suit::Spade,
                    },
                    Card {
                        rank: Rank::Five,
                        suit: Suit::Diamond,
                    },
                    Card {
                        rank: Rank::Four,
                        suit: Suit::Club,
                    },
                ],
            }],
            board: None,
        };

        assert_eq!(
            evaluation(hand),
            Evaluation {
                variant: Variant::FiveCardDraw,
                players: vec![PlayerEval {
                    name: String::from("Player 1"),
                    cards: vec![
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Club,
                        },
                    ],
                    winner: true,
                    display: String::from("Four of a kind, Fives with Four kicker"),
                }],
                board: None,
            }
        );
    }

    #[test]
    fn test_five_card_draw_one_player_evaluation_straight_flush() {
        let hand = Hand {
            variant: Variant::FiveCardDraw,
            players: vec![Player {
                name: String::from("Player 1"),
                cards: vec![
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
                ],
            }],
            board: None,
        };

        assert_eq!(
            evaluation(hand),
            Evaluation {
                variant: Variant::FiveCardDraw,
                players: vec![PlayerEval {
                    name: String::from("Player 1"),
                    cards: vec![
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
                    ],
                    winner: true,
                    display: String::from("Six high straight flush"),
                }],
                board: None,
            }
        );
    }

    #[test]
    fn test_five_card_draw_one_player_evaluation_wheel_flush() {
        let hand = Hand {
            variant: Variant::FiveCardDraw,
            players: vec![Player {
                name: String::from("Player 1"),
                cards: vec![
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
                        rank: Rank::Ace,
                        suit: Suit::Diamond,
                    },
                ],
            }],
            board: None,
        };

        assert_eq!(
            evaluation(hand),
            Evaluation {
                variant: Variant::FiveCardDraw,
                players: vec![PlayerEval {
                    name: String::from("Player 1"),
                    cards: vec![
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
                            rank: Rank::Ace,
                            suit: Suit::Diamond,
                        },
                    ],
                    winner: true,
                    display: String::from("Wheel flush"),
                }],
                board: None,
            }
        );
    }

    #[test]
    fn test_five_card_draw_one_player_evaluation_royal_flush() {
        let hand = Hand {
            variant: Variant::FiveCardDraw,
            players: vec![Player {
                name: String::from("Player 1"),
                cards: vec![
                    Card {
                        rank: Rank::Ten,
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
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Diamond,
                    },
                ],
            }],
            board: None,
        };

        assert_eq!(
            evaluation(hand),
            Evaluation {
                variant: Variant::FiveCardDraw,
                players: vec![PlayerEval {
                    name: String::from("Player 1"),
                    cards: vec![
                        Card {
                            rank: Rank::Ten,
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
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Diamond,
                        },
                    ],
                    winner: true,
                    display: String::from("Royal flush"),
                }],
                board: None,
            }
        );
    }
}
