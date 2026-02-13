use crate::{Evaluation, Hand, Player, PlayerEval};

const NUM_SUITS: usize = 4;
const NUM_RANKS: usize = 130;
const ACE_VALUE: usize = 2_usize.pow(13);
const NUM_CARDS_IN_HAND: usize = 5;

pub fn evaluation(hand: Hand) -> Evaluation {
    let mut highest_hand_score;
    let mut player_evals = vec![];

    hand.players.iter().for_each(|player| {
        player_evals.push(player_evaluation(player));
    });

    Evaluation {
        variant: hand.variant,
        players: player_evals,
        board: hand.board,
        // winners: player_evals.iter().filter_map(|player| )
        winners: vec![],
        display: String::from("Hello"),
    }
}

struct AvailableRanks {
    straight_flush: bool,
    quads: bool,
    full_house: bool,
    flush: bool,
    straight: bool,
    three_of_a_kind: bool,
    two_pair: bool,
    pair: bool,
    high_card: bool,
}

fn player_evaluation(player: Player) -> PlayerEval {
    let mut suits = [0; NUM_SUITS];
    let mut ranks = [0; NUM_RANKS];

    player.cards.iter().for_each(|card| {
        suits[card.matrix_value() / NUM_SUITS] += 1;
        ranks[card.matrix_value() % NUM_RANKS] += 1;
    });

    let rank_value = ranks.iter().enumerate().fold(0, |acc, (idx, &rank)| {
        if rank == 1 {
            acc + 2_usize.pow((idx + 1) as u32)
        } else if rank > 1 {
            acc + 2_usize.pow((idx + 1) as u32) * ACE_VALUE * rank
        } else {
            acc
        }
    });
    let first_card_idx = ranks.iter().position(|rank| *rank == 1);
    let available_ranks = AvailableRanks {
        // NOTE: is this needed?
        straight_flush: false,
        quads: ranks.iter().any(|rank| *rank == 4),
        full_house: ranks.iter().any(|rank| *rank == 2) && ranks.iter().any(|rank| *rank == 3),
        flush: suits.iter().any(|suit| *suit == 5),
        straight: &ranks[first_card_idx..first_card_idx + 5]
            .iter()
            .filter(|rank| rank == 1)
            .collect()
            .length
            == 5,
        three_of_a_kind: ranks.iter().any(|rank| *rank == 2),
        two_pair: ranks.iter().filter(|&&rank| rank == 2).count() == 2,
        pair: ranks.iter().filter(|&&rank| rank == 2).count() == 1,
        high_card: true,
    };
    available_ranks.straight_flush = available_ranks.straight && available_ranks.flush;
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
                    display: String::from("High card Ace with King, Queen, Three, and Two kickers"),
                }],
                board: None,
                winners: vec![String::from("Player 1")],
                display: String::from("High card Ace with King, Queen, Three, and Two kickers"),
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
                    display: String::from("Pair of Aces with King, Queen, and Three kickers"),
                }],
                winners: vec![String::from("Player 1")],
                display: String::from("Pair of Aces with King, Queen, and Three kickers"),
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
                    display: String::from("Two pair, Aces over Kings with Two kicker"),
                }],
                winners: vec![String::from("Player 1")],
                display: String::from("Two pair, Aces over Kings with Two kicker"),
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
                    display: String::from("Three of a kind, Aces with King, Jack kickers"),
                }],
                winners: vec![String::from("Player 1")],
                display: String::from("Three of a kind, Aces with King, Jack kickers"),
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
                    display: String::from("Jack high straight"),
                }],
                winners: vec![String::from("Player 1")],
                display: String::from("Jack high straight"),
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
                    display: String::from("Ace high flush, with Jing, Seven, Three, Two"),
                }],
                winners: vec![String::from("Player 1")],
                display: String::from("Ace high flush, with Jing, Seven, Three, Two"),
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
                    display: String::from("Ace high flush, with Jing, Seven, Three, Two"),
                }],
                winners: vec![String::from("Player 1")],
                display: String::from("Full house, Fives full of Fours"),
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
                    display: String::from("Four of a kind, Fives with Four kicker"),
                }],
                winners: vec![String::from("Player 1")],
                display: String::from("Four of a kind, Fives with Four kicker"),
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
                    display: String::from("Six high straight flush"),
                }],
                winners: vec![String::from("Player 1")],
                display: String::from("Six high straight flush"),
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
                    display: String::from("Wheel flush"),
                }],
                winners: vec![String::from("Player 1")],
                display: String::from("Wheel flush"),
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
                    display: String::from("Royal flush"),
                }],
                winners: vec![String::from("Player 1")],
                display: String::from("Royal flush"),
                board: None,
            }
        );
    }
}
