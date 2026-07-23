mod evaluate_five_cards_hi;
mod evaluate_seven_cards_hi;

use crate::{
    constants::{BROADWAY_STRAIGHT_INDICATOR, WHEEL_STRAIGHT_INDICATOR},
    evaluation::{
        evaluate_five_cards_hi::evaluate_five_cards_hi,
        evaluate_seven_cards_hi::evaluate_seven_cards_hi,
    },
    types::{Evaluation, Hand, PlayerEval, Variant},
};

struct PlayerEvaluatedHand {
    id: String,
    result: SingleHandEval,
}
// LIME: will need more texas holdem evaluates
pub fn evaluate(hand: Hand) -> Evaluation {
    let hand_evals: Vec<PlayerEvaluatedHand> = hand
        .players
        .iter()
        .map(|player| match hand.variant {
            Variant::FiveCardDraw => PlayerEvaluatedHand {
                id: player.id.clone(),
                result: evaluate_five_cards_hi(&player.cards),
            },
            Variant::TexasHoldem => PlayerEvaluatedHand {
                id: player.id.clone(),
                result: evaluate_seven_cards_hi(&player.cards),
            },
        })
        .collect();

    let max_rank_value = hand_evals
        .iter()
        .map(|eval| eval.result.rank_value)
        .max()
        .unwrap();

    Evaluation {
        id: hand.id.clone(),
        players: hand_evals
            .iter()
            .map(|eval| PlayerEval {
                id: eval.id.clone(),
                hand: eval.result.hand_description.clone(),
            })
            .collect(),
        winners: hand_evals
            .iter()
            .filter_map(|eval| {
                (eval.result.rank_value == max_rank_value).then_some(eval.id.clone())
            })
            .collect(),
        winning_hand: hand_evals
            .iter()
            .find(|eval| eval.result.rank_value == max_rank_value)
            .unwrap()
            .result
            .hand_description
            .clone(),
    }
}

#[derive(Debug, PartialEq)]
struct SingleHandEval {
    hand_description: String,
    rank_value: usize,
}

struct HandRanks {
    straight_flush: bool,
    four_of_a_kind: bool,
    full_house: bool,
    flush: bool,
    straight: bool,
    three_of_a_kind: bool,
    two_pair: bool,
    pair: bool,
}
enum HandRankMultipliers {
    StraightFlush = 9,
    Quads = 8,
    FullHouse = 7,
    Flush = 6,
    Straight = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    Pair = 2,
    HighCard = 1,
}
impl HandRanks {
    pub fn obtain_hand_multiplier(&self) -> usize {
        match true {
            _ if self.straight_flush => HandRankMultipliers::StraightFlush as usize,
            _ if self.four_of_a_kind => HandRankMultipliers::Quads as usize,
            _ if self.full_house => HandRankMultipliers::FullHouse as usize,
            _ if self.flush => HandRankMultipliers::Flush as usize,
            _ if self.straight => HandRankMultipliers::Straight as usize,
            _ if self.three_of_a_kind => HandRankMultipliers::ThreeOfAKind as usize,
            _ if self.two_pair => HandRankMultipliers::TwoPair as usize,
            _ if self.pair => HandRankMultipliers::Pair as usize,
            _ => HandRankMultipliers::HighCard as usize,
        }
    }
    pub fn generate_hand_description(&self, rank_value: usize) -> String {
        match true {
            _ if self.straight_flush => {
                if rank_value == BROADWAY_STRAIGHT_INDICATOR {
                    return String::from("Royal Flush");
                } else if rank_value == WHEEL_STRAIGHT_INDICATOR {
                    return String::from("Wheel Flush");
                }
                String::from("Straight Flush")
            }
            _ if self.four_of_a_kind => String::from("Four of a Kind"),
            _ if self.full_house => String::from("Full House"),
            _ if self.flush => String::from("Flush"),
            _ if self.straight => {
                if rank_value == BROADWAY_STRAIGHT_INDICATOR {
                    return String::from("Broadway");
                } else if rank_value == WHEEL_STRAIGHT_INDICATOR {
                    return String::from("Wheel");
                }
                String::from("Straight")
            }
            _ if self.three_of_a_kind => String::from("Three of a Kind"),
            _ if self.two_pair => String::from("Two Pair"),
            _ if self.pair => String::from("Pair"),
            _ => String::from("High Card"),
        }
    }
}

mod tests {
    use crate::types::{Card, Player, Rank, Suit};

    use super::*;

    #[test]
    fn test_hand_ranks_impl_obtain_hand_multiplier_straight_flush() {
        let hand_rank = HandRanks {
            straight_flush: true,
            four_of_a_kind: true,
            full_house: true,
            flush: true,
            straight: true,
            three_of_a_kind: true,
            two_pair: true,
            pair: true,
        };
        assert_eq!(hand_rank.obtain_hand_multiplier(), 9);
    }

    #[test]
    fn test_hand_ranks_impl_obtain_hand_multiplier_four_of_a_kind() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: true,
            full_house: true,
            flush: true,
            straight: true,
            three_of_a_kind: true,
            two_pair: true,
            pair: true,
        };
        assert_eq!(hand_rank.obtain_hand_multiplier(), 8);
    }

    #[test]
    fn test_hand_ranks_impl_obtain_hand_multiplier_full_house() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: false,
            full_house: true,
            flush: true,
            straight: true,
            three_of_a_kind: true,
            two_pair: true,
            pair: true,
        };
        assert_eq!(hand_rank.obtain_hand_multiplier(), 7);
    }

    #[test]
    fn test_hand_ranks_impl_obtain_hand_multiplier_flush() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: false,
            full_house: false,
            flush: true,
            straight: true,
            three_of_a_kind: true,
            two_pair: true,
            pair: true,
        };
        assert_eq!(hand_rank.obtain_hand_multiplier(), 6);
    }

    #[test]
    fn test_hand_ranks_impl_obtain_hand_multiplier_straight() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: false,
            full_house: false,
            flush: false,
            straight: true,
            three_of_a_kind: true,
            two_pair: true,
            pair: true,
        };
        assert_eq!(hand_rank.obtain_hand_multiplier(), 5);
    }

    #[test]
    fn test_hand_ranks_impl_obtain_hand_multiplier_three_of_a_kind() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: false,
            full_house: false,
            flush: false,
            straight: false,
            three_of_a_kind: true,
            two_pair: true,
            pair: true,
        };
        assert_eq!(hand_rank.obtain_hand_multiplier(), 4);
    }

    #[test]
    fn test_hand_ranks_impl_obtain_hand_multiplier_two_pair() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: false,
            full_house: false,
            flush: false,
            straight: false,
            three_of_a_kind: false,
            two_pair: true,
            pair: true,
        };
        assert_eq!(hand_rank.obtain_hand_multiplier(), 3);
    }

    #[test]
    fn test_hand_ranks_impl_obtain_hand_multiplier_pair() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: false,
            full_house: false,
            flush: false,
            straight: false,
            three_of_a_kind: false,
            two_pair: false,
            pair: true,
        };
        assert_eq!(hand_rank.obtain_hand_multiplier(), 2);
    }

    #[test]
    fn test_hand_ranks_impl_obtain_hand_multiplier_high_card() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: false,
            full_house: false,
            flush: false,
            straight: false,
            three_of_a_kind: false,
            two_pair: false,
            pair: false,
        };
        assert_eq!(hand_rank.obtain_hand_multiplier(), 1);
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_royal_flush() {
        let hand_rank = HandRanks {
            straight_flush: true,
            four_of_a_kind: true,
            full_house: true,
            flush: true,
            straight: true,
            three_of_a_kind: true,
            two_pair: true,
            pair: true,
        };
        assert_eq!(
            hand_rank.generate_hand_description(15_872),
            String::from("Royal Flush")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_wheel_flush() {
        let hand_rank = HandRanks {
            straight_flush: true,
            four_of_a_kind: true,
            full_house: true,
            flush: true,
            straight: true,
            three_of_a_kind: true,
            two_pair: true,
            pair: true,
        };
        assert_eq!(
            hand_rank.generate_hand_description(8_222),
            String::from("Wheel Flush")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_straight_flush() {
        let hand_rank = HandRanks {
            straight_flush: true,
            four_of_a_kind: true,
            full_house: true,
            flush: true,
            straight: true,
            three_of_a_kind: true,
            two_pair: true,
            pair: true,
        };
        assert_eq!(
            hand_rank.generate_hand_description(1),
            String::from("Straight Flush")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_four_of_a_kind() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: true,
            full_house: true,
            flush: true,
            straight: true,
            three_of_a_kind: true,
            two_pair: true,
            pair: true,
        };
        assert_eq!(
            hand_rank.generate_hand_description(1),
            String::from("Four of a Kind")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_full_house() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: false,
            full_house: true,
            flush: true,
            straight: true,
            three_of_a_kind: true,
            two_pair: true,
            pair: true,
        };
        assert_eq!(
            hand_rank.generate_hand_description(1),
            String::from("Full House")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_flush() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: false,
            full_house: false,
            flush: true,
            straight: true,
            three_of_a_kind: true,
            two_pair: true,
            pair: true,
        };
        assert_eq!(
            hand_rank.generate_hand_description(1),
            String::from("Flush")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_broadway() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: false,
            full_house: false,
            flush: false,
            straight: true,
            three_of_a_kind: true,
            two_pair: true,
            pair: true,
        };
        assert_eq!(
            hand_rank.generate_hand_description(15_872),
            String::from("Broadway")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_wheel() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: false,
            full_house: false,
            flush: false,
            straight: true,
            three_of_a_kind: true,
            two_pair: true,
            pair: true,
        };
        assert_eq!(
            hand_rank.generate_hand_description(8_222),
            String::from("Wheel")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_straight() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: false,
            full_house: false,
            flush: false,
            straight: true,
            three_of_a_kind: true,
            two_pair: true,
            pair: true,
        };
        assert_eq!(
            hand_rank.generate_hand_description(1),
            String::from("Straight")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_three_of_a_kind() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: false,
            full_house: false,
            flush: false,
            straight: false,
            three_of_a_kind: true,
            two_pair: true,
            pair: true,
        };
        assert_eq!(
            hand_rank.generate_hand_description(1),
            String::from("Three of a Kind")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_two_pair() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: false,
            full_house: false,
            flush: false,
            straight: false,
            three_of_a_kind: false,
            two_pair: true,
            pair: true,
        };
        assert_eq!(
            hand_rank.generate_hand_description(1),
            String::from("Two Pair")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_pair() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: false,
            full_house: false,
            flush: false,
            straight: false,
            three_of_a_kind: false,
            two_pair: false,
            pair: true,
        };
        assert_eq!(hand_rank.generate_hand_description(1), String::from("Pair"));
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_high_card() {
        let hand_rank = HandRanks {
            straight_flush: false,
            four_of_a_kind: false,
            full_house: false,
            flush: false,
            straight: false,
            three_of_a_kind: false,
            two_pair: false,
            pair: false,
        };
        assert_eq!(
            hand_rank.generate_hand_description(1),
            String::from("High Card")
        );
    }

    #[test]
    fn test_evaluate_five_card_draw_two_player_royal_vs_broadway() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            board: None,
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Two,
                suit: Suit::Club,
            }],
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
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Jack,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Queen,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::King,
                            suit: Suit::Spade,
                        },
                    ],
                },
            ],
        };
        assert_eq!(
            evaluate(hand),
            Evaluation {
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
            }
        );
    }

    #[test]
    fn test_evaluate_five_card_draw_two_player_royal_vs_royal() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            board: None,
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Two,
                suit: Suit::Club,
            }],
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
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Jack,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Queen,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::King,
                            suit: Suit::Spade,
                        },
                    ],
                },
            ],
        };
        assert_eq!(
            evaluate(hand),
            Evaluation {
                id: String::from("hand-id"),
                players: vec![
                    PlayerEval {
                        id: String::from("player-1-id"),
                        hand: String::from("Royal Flush"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Royal Flush"),
                    }
                ],
                winners: vec![String::from("player-1-id"), String::from("player-2-id")],
                winning_hand: String::from("Royal Flush"),
            }
        );
    }

    #[test]
    fn test_evaluate_five_card_draw_two_player_six_high_straight_vs_wheel() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            board: None,
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Two,
                suit: Suit::Club,
            }],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
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
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Diamond,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Spade,
                        },
                    ],
                },
            ],
        };
        assert_eq!(
            evaluate(hand),
            Evaluation {
                id: String::from("hand-id"),
                players: vec![
                    PlayerEval {
                        id: String::from("player-1-id"),
                        hand: String::from("Straight"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Wheel"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Straight"),
            }
        );
    }

    #[test]
    fn test_evaluate_five_card_draw_two_player_full_house_vs_full_house_no_chop() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            board: None,
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Two,
                suit: Suit::Club,
            }],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Diamond,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Club,
                        },
                    ],
                },
            ],
        };
        assert_eq!(
            evaluate(hand),
            Evaluation {
                id: String::from("hand-id"),
                players: vec![
                    PlayerEval {
                        id: String::from("player-1-id"),
                        hand: String::from("Full House"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Full House"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Full House"),
            }
        );
    }

    #[test]
    fn test_evaluate_five_card_draw_two_player_two_pair_vs_two_pair_chopped() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            board: None,
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Two,
                suit: Suit::Club,
            }],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Diamond,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Club,
                        },
                    ],
                },
            ],
        };
        assert_eq!(
            evaluate(hand),
            Evaluation {
                id: String::from("hand-id"),
                players: vec![
                    PlayerEval {
                        id: String::from("player-1-id"),
                        hand: String::from("Two Pair"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Two Pair"),
                    }
                ],
                winners: vec![String::from("player-1-id"), String::from("player-2-id")],
                winning_hand: String::from("Two Pair"),
            }
        );
    }

    #[test]
    fn test_evaluate_five_card_draw_two_player_two_pair_vs_two_pair_no_chopped_kicker() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            board: None,
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Two,
                suit: Suit::Club,
            }],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Diamond,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Club,
                        },
                    ],
                },
            ],
        };
        assert_eq!(
            evaluate(hand),
            Evaluation {
                id: String::from("hand-id"),
                players: vec![
                    PlayerEval {
                        id: String::from("player-1-id"),
                        hand: String::from("Two Pair"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Two Pair"),
                    }
                ],
                winners: vec![String::from("player-2-id")],
                winning_hand: String::from("Two Pair"),
            }
        );
    }

    #[test]
    fn test_evaluate_five_card_draw_two_player_wheel_vs_wheel() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            board: None,
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Two,
                suit: Suit::Club,
            }],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
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
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Diamond,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Spade,
                        },
                    ],
                },
            ],
        };
        assert_eq!(
            evaluate(hand),
            Evaluation {
                id: String::from("hand-id"),
                players: vec![
                    PlayerEval {
                        id: String::from("player-1-id"),
                        hand: String::from("Wheel"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Wheel"),
                    }
                ],
                winners: vec![String::from("player-1-id"), String::from("player-2-id")],
                winning_hand: String::from("Wheel"),
            }
        );
    }

    #[test]
    fn test_evaluate_five_card_draw_two_player_flush_vs_flush_no_chopped() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            board: None,
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Two,
                suit: Suit::Club,
            }],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
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
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Diamond,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Spade,
                        },
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
                        Card {
                            rank: Rank::King,
                            suit: Suit::Spade,
                        },
                    ],
                },
            ],
        };
        assert_eq!(
            evaluate(hand),
            Evaluation {
                id: String::from("hand-id"),
                players: vec![
                    PlayerEval {
                        id: String::from("player-1-id"),
                        hand: String::from("Flush"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Flush"),
                    }
                ],
                winners: vec![String::from("player-2-id")],
                winning_hand: String::from("Flush"),
            }
        );
    }

    #[test]
    fn test_evaluate_five_card_draw_two_player_flush_vs_flush_chopped() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            board: None,
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Two,
                suit: Suit::Club,
            }],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
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
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Diamond,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Spade,
                        },
                    ],
                },
            ],
        };
        assert_eq!(
            evaluate(hand),
            Evaluation {
                id: String::from("hand-id"),
                players: vec![
                    PlayerEval {
                        id: String::from("player-1-id"),
                        hand: String::from("Flush"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Flush"),
                    }
                ],
                winners: vec![String::from("player-1-id"), String::from("player-2-id")],
                winning_hand: String::from("Flush"),
            }
        );
    }

    #[test]
    fn test_evaluate_five_card_draw_two_player_flush_vs_straight() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            board: None,
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Two,
                suit: Suit::Club,
            }],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
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
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Diamond,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Seven,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Spade,
                        },
                    ],
                },
            ],
        };
        assert_eq!(
            evaluate(hand),
            Evaluation {
                id: String::from("hand-id"),
                players: vec![
                    PlayerEval {
                        id: String::from("player-1-id"),
                        hand: String::from("Flush"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Straight"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Flush"),
            }
        );
    }

    #[test]
    fn test_evaluate_five_card_draw_two_player_high_card_vs_high_card_no_chopped() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            board: None,
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Two,
                suit: Suit::Club,
            }],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Seven,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Diamond,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Seven,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::King,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Spade,
                        },
                    ],
                },
            ],
        };
        assert_eq!(
            evaluate(hand),
            Evaluation {
                id: String::from("hand-id"),
                players: vec![
                    PlayerEval {
                        id: String::from("player-1-id"),
                        hand: String::from("High Card"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("High Card"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("High Card"),
            }
        );
    }

    #[test]
    fn test_evaluate_five_card_draw_three_player_all_chopped() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            board: None,
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Two,
                suit: Suit::Club,
            }],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Seven,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Diamond,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Seven,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Club,
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
                            rank: Rank::Six,
                            suit: Suit::Spade,
                        },
                    ],
                },
                Player {
                    id: String::from("player-3-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Seven,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Club,
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
                            rank: Rank::Six,
                            suit: Suit::Spade,
                        },
                    ],
                },
            ],
        };
        assert_eq!(
            evaluate(hand),
            Evaluation {
                id: String::from("hand-id"),
                players: vec![
                    PlayerEval {
                        id: String::from("player-1-id"),
                        hand: String::from("High Card"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("High Card"),
                    },
                    PlayerEval {
                        id: String::from("player-3-id"),
                        hand: String::from("High Card"),
                    }
                ],
                winners: vec![
                    String::from("player-1-id"),
                    String::from("player-2-id"),
                    String::from("player-3-id")
                ],
                winning_hand: String::from("High Card"),
            }
        );
    }

    #[test]
    fn test_evaluate_five_card_draw_three_player_two_chopped() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            board: None,
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Two,
                suit: Suit::Club,
            }],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Seven,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Diamond,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Seven,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Club,
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
                            rank: Rank::Six,
                            suit: Suit::Spade,
                        },
                    ],
                },
                Player {
                    id: String::from("player-3-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Seven,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::King,
                            suit: Suit::Club,
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
                            rank: Rank::Six,
                            suit: Suit::Spade,
                        },
                    ],
                },
            ],
        };
        assert_eq!(
            evaluate(hand),
            Evaluation {
                id: String::from("hand-id"),
                players: vec![
                    PlayerEval {
                        id: String::from("player-1-id"),
                        hand: String::from("High Card"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("High Card"),
                    },
                    PlayerEval {
                        id: String::from("player-3-id"),
                        hand: String::from("High Card"),
                    }
                ],
                winners: vec![String::from("player-1-id"), String::from("player-2-id")],
                winning_hand: String::from("High Card"),
            }
        );
    }

    #[test]
    fn test_evaluate_five_card_draw_three_player_one_winner() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            board: None,
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![Card {
                rank: Rank::Two,
                suit: Suit::Club,
            }],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Seven,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Diamond,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Seven,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Queen,
                            suit: Suit::Club,
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
                            rank: Rank::Six,
                            suit: Suit::Spade,
                        },
                    ],
                },
                Player {
                    id: String::from("player-3-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Seven,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::King,
                            suit: Suit::Club,
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
                            rank: Rank::Six,
                            suit: Suit::Spade,
                        },
                    ],
                },
            ],
        };
        assert_eq!(
            evaluate(hand),
            Evaluation {
                id: String::from("hand-id"),
                players: vec![
                    PlayerEval {
                        id: String::from("player-1-id"),
                        hand: String::from("High Card"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("High Card"),
                    },
                    PlayerEval {
                        id: String::from("player-3-id"),
                        hand: String::from("High Card"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("High Card"),
            }
        );
    }
}
