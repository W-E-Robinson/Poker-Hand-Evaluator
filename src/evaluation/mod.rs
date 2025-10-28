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

enum HandRank<'a> {
    HighCard {
        high_card: &'a Card,
        first_kicker: &'a Card,
        second_kicker: &'a Card,
        third_kicker: &'a Card,
        fourth_kicker: &'a Card,
    },
    Pair {
        pair: [&'a Card; 2],
        first_kicker: &'a Card,
        second_kicker: &'a Card,
        third_kicker: &'a Card,
    },
    TwoPair {
        high_pair: [&'a Card; 2],
        low_pair: [&'a Card; 2],
        kicker: &'a Card,
    },
    ThreeOfAKind {
        trips: [&'a Card; 3],
        first_kicker: &'a Card,
        second_kicker: &'a Card,
    },
    Straight {
        high_card: &'a Card,
        low_card: &'a Card,
    },
    Flush {
        first_card: &'a Card,
        second_card: &'a Card,
        third_card: &'a Card,
        fourth_card: &'a Card,
        fifth_card: &'a Card,
    },
    FullHouse {
        trips: [&'a Card; 3],
        pair: [&'a Card; 2],
    },
    FourOfAKind {
        quads: [&'a Card; 4],
        kicker: &'a Card,
    },
    StraightFlush {
        high_card: &'a Card,
        low_card: &'a Card,
    },
}

// NOTE: move to a display module?
impl<'a> HandRank<'a> {
    fn to_display(&self) -> String {
        match self {
            HandRank::HighCard {
                high_card,
                first_kicker,
                second_kicker,
                third_kicker,
                fourth_kicker,
            } => {
                format!(
                    "High card {} with {}, {}, {}, and {} kickers",
                    high_card.to_rank_string(false),
                    first_kicker.to_rank_string(false),
                    second_kicker.to_rank_string(false),
                    third_kicker.to_rank_string(false),
                    fourth_kicker.to_rank_string(false),
                )
            }
            HandRank::Pair {
                pair,
                first_kicker,
                second_kicker,
                third_kicker,
            } => {
                format!(
                    "Pair of {} with {}, {}, and {} kickers",
                    pair[0].to_rank_string(true),
                    first_kicker.to_rank_string(false),
                    second_kicker.to_rank_string(false),
                    third_kicker.to_rank_string(false),
                )
            }
            HandRank::TwoPair {
                high_pair,
                low_pair,
                kicker,
            } => {
                format!(
                    "Two pair, {} over {} with {} kicker",
                    high_pair[0].to_rank_string(true),
                    low_pair[0].to_rank_string(true),
                    kicker.to_rank_string(false),
                )
            }
            HandRank::ThreeOfAKind {
                trips,
                first_kicker,
                second_kicker,
            } => {
                format!(
                    "Three of a kind, {} and {}, {} kickers",
                    trips[0].to_rank_string(true),
                    first_kicker.to_rank_string(false),
                    second_kicker.to_rank_string(false),
                )
            }
            HandRank::Straight {
                high_card,
                low_card,
            } => {
                let straight;
                let is_high_card_ace = high_card.rank == Rank::Ace;
                let is_low_card_ten = low_card.rank == Rank::Ten;
                let is_low_card_two = low_card.rank == Rank::Two;

                if is_high_card_ace && is_low_card_ten {
                    straight = String::from("Broadway");
                } else if is_high_card_ace && is_low_card_two {
                    straight = String::from("Wheel");
                } else {
                    straight = format!("{} high straight", high_card.to_rank_string(false));
                }

                format!("{}", straight,)
            }
            HandRank::Flush {
                first_card,
                second_card,
                third_card,
                fourth_card,
                fifth_card,
            } => {
                format!(
                    "{} high flush, with {}, {}, {}, {}",
                    first_card.to_rank_string(false),
                    second_card.to_rank_string(false),
                    third_card.to_rank_string(false),
                    fourth_card.to_rank_string(false),
                    fifth_card.to_rank_string(false),
                )
            }
            HandRank::FullHouse { trips, pair } => format!(
                "Full house, {} full of {}",
                trips[0].to_rank_string(true),
                pair[0].to_rank_string(true),
            ),
            HandRank::FourOfAKind { quads, kicker } => {
                format!(
                    "Four of a kind, {} with {} kicker",
                    quads[0].to_rank_string(true),
                    kicker.to_rank_string(false),
                )
            }
            HandRank::StraightFlush {
                high_card,
                low_card,
            } => {
                let straight_flush;
                let is_high_card_ace = high_card.rank == Rank::Ace;
                let is_low_card_ten = low_card.rank == Rank::Ten;
                let is_low_card_two = low_card.rank == Rank::Two;

                if is_high_card_ace && is_low_card_ten {
                    straight_flush = String::from("Royal flush");
                } else if is_high_card_ace && is_low_card_two {
                    straight_flush = String::from("Wheel flush");
                } else {
                    straight_flush =
                        format!("{} high straight flush", high_card.to_rank_string(false));
                }

                format!("{}", straight_flush)
            }
        }
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
                    display: String::from("Two pair, Aves over Kings with Two kicker"),
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

    #[test]
    fn test_high_card_to_display() {
        let cards = HandRank::HighCard {
            high_card: (&Card {
                suit: Suit::Club,
                rank: Rank::Ace,
            }),
            first_kicker: (&Card {
                suit: Suit::Spade,
                rank: Rank::King,
            }),
            second_kicker: (&Card {
                suit: Suit::Club,
                rank: Rank::Jack,
            }),
            third_kicker: (&Card {
                suit: Suit::Club,
                rank: Rank::Three,
            }),
            fourth_kicker: (&Card {
                suit: Suit::Heart,
                rank: Rank::Two,
            }),
        };
        assert_eq!(
            cards.to_display(),
            "High card Ace with King, Jack, Three, and Two kickers"
        );
    }

    #[test]
    fn test_pair_to_display() {
        let cards = HandRank::Pair {
            pair: ([
                &Card {
                    suit: Suit::Heart,
                    rank: Rank::Two,
                },
                &Card {
                    suit: Suit::Club,
                    rank: Rank::Two,
                },
            ]),
            first_kicker: (&Card {
                suit: Suit::Club,
                rank: Rank::Ace,
            }),
            second_kicker: (&Card {
                suit: Suit::Spade,
                rank: Rank::King,
            }),
            third_kicker: (&Card {
                suit: Suit::Club,
                rank: Rank::Jack,
            }),
        };
        assert_eq!(
            cards.to_display(),
            "Pair of Twos with Ace, King, and Jack kickers"
        );
    }

    #[test]
    fn test_two_pair_to_display() {
        let cards = HandRank::TwoPair {
            high_pair: ([
                &Card {
                    suit: Suit::Spade,
                    rank: Rank::King,
                },
                &Card {
                    suit: Suit::Club,
                    rank: Rank::King,
                },
            ]),
            low_pair: ([
                &Card {
                    suit: Suit::Heart,
                    rank: Rank::Six,
                },
                &Card {
                    suit: Suit::Club,
                    rank: Rank::Six,
                },
            ]),
            kicker: (&Card {
                suit: Suit::Club,
                rank: Rank::Jack,
            }),
        };
        assert_eq!(
            cards.to_display(),
            "Two pair, Kings over Sixes with Jack kicker",
        );
    }

    #[test]
    fn test_three_of_a_kind_to_display() {
        let cards = HandRank::ThreeOfAKind {
            trips: ([
                &Card {
                    suit: Suit::Heart,
                    rank: Rank::Two,
                },
                &Card {
                    suit: Suit::Club,
                    rank: Rank::Two,
                },
                &Card {
                    suit: Suit::Spade,
                    rank: Rank::Two,
                },
            ]),
            first_kicker: (&Card {
                suit: Suit::Heart,
                rank: Rank::King,
            }),
            second_kicker: (&Card {
                suit: Suit::Club,
                rank: Rank::Jack,
            }),
        };
        assert_eq!(
            cards.to_display(),
            "Three of a kind, Twos and King, Jack kickers",
        );
    }

    #[test]
    fn test_straight_to_display() {
        let cards = HandRank::Straight {
            high_card: (&Card {
                suit: Suit::Club,
                rank: Rank::Six,
            }),
            low_card: (&Card {
                suit: Suit::Heart,
                rank: Rank::Two,
            }),
        };
        assert_eq!(cards.to_display(), "Six high straight",);
    }

    #[test]
    fn test_wheel_straight_to_display() {
        let cards = HandRank::Straight {
            high_card: (&Card {
                suit: Suit::Club,
                rank: Rank::Ace,
            }),
            low_card: (&Card {
                suit: Suit::Club,
                rank: Rank::Two,
            }),
        };
        assert_eq!(cards.to_display(), "Wheel",);
    }

    #[test]
    fn test_broadway_straight_to_display() {
        let cards = HandRank::Straight {
            high_card: (&Card {
                suit: Suit::Club,
                rank: Rank::Ace,
            }),
            low_card: (&Card {
                suit: Suit::Heart,
                rank: Rank::Ten,
            }),
        };
        assert_eq!(cards.to_display(), "Broadway",);
    }

    #[test]
    fn test_flush_to_display() {
        let cards = HandRank::Flush {
            first_card: (&Card {
                suit: Suit::Heart,
                rank: Rank::King,
            }),
            second_card: (&Card {
                suit: Suit::Heart,
                rank: Rank::Jack,
            }),
            third_card: (&Card {
                suit: Suit::Heart,
                rank: Rank::Ten,
            }),
            fourth_card: (&Card {
                suit: Suit::Heart,
                rank: Rank::Seven,
            }),
            fifth_card: (&Card {
                suit: Suit::Heart,
                rank: Rank::Six,
            }),
        };
        assert_eq!(
            cards.to_display(),
            "King high flush, with Jack, Ten, Seven, Six"
        );
    }

    #[test]
    fn test_full_house_to_display() {
        let cards = HandRank::FullHouse {
            trips: ([
                &Card {
                    suit: Suit::Spade,
                    rank: Rank::Seven,
                },
                &Card {
                    suit: Suit::Heart,
                    rank: Rank::Seven,
                },
                &Card {
                    suit: Suit::Club,
                    rank: Rank::Ten,
                },
            ]),
            pair: ([
                &Card {
                    suit: Suit::Spade,
                    rank: Rank::Ten,
                },
                &Card {
                    suit: Suit::Heart,
                    rank: Rank::Ten,
                },
            ]),
        };
        assert_eq!(cards.to_display(), "Full house, Sevens full of Tens");
    }

    #[test]
    fn test_four_of_a_kind_house_to_display() {
        let cards = HandRank::FourOfAKind {
            quads: ([
                &Card {
                    suit: Suit::Diamond,
                    rank: Rank::Ten,
                },
                &Card {
                    suit: Suit::Spade,
                    rank: Rank::Ten,
                },
                &Card {
                    suit: Suit::Heart,
                    rank: Rank::Ten,
                },
                &Card {
                    suit: Suit::Club,
                    rank: Rank::Ten,
                },
            ]),
            kicker: (&Card {
                suit: Suit::Spade,
                rank: Rank::Seven,
            }),
        };
        assert_eq!(cards.to_display(), "Four of a kind, Tens with Seven kicker");
    }

    #[test]
    fn test_straight_flush_to_display() {
        let cards = HandRank::StraightFlush {
            high_card: (&Card {
                suit: Suit::Diamond,
                rank: Rank::King,
            }),
            low_card: (&Card {
                suit: Suit::Diamond,
                rank: Rank::Nine,
            }),
        };
        assert_eq!(cards.to_display(), "King high straight flush");
    }

    #[test]
    fn test_wheel_flush_to_display() {
        let cards = HandRank::StraightFlush {
            high_card: (&Card {
                suit: Suit::Diamond,
                rank: Rank::Ace,
            }),
            low_card: (&Card {
                suit: Suit::Diamond,
                rank: Rank::Two,
            }),
        };
        assert_eq!(cards.to_display(), "Wheel flush");
    }

    #[test]
    fn test_royal_flush_to_display() {
        let cards = HandRank::StraightFlush {
            high_card: (&Card {
                suit: Suit::Diamond,
                rank: Rank::Ace,
            }),
            low_card: (&Card {
                suit: Suit::Diamond,
                rank: Rank::Ten,
            }),
        };
        assert_eq!(cards.to_display(), "Royal flush");
    }
}
