mod evaluate_five_cards_hi;
mod evaluate_omaha_five_cards_hi;
mod evaluate_seven_cards_hi;

use crate::{
    constants::{BROADWAY_STRAIGHT_INDICATOR, WHEEL_STRAIGHT_INDICATOR},
    evaluation::{
        evaluate_five_cards_hi::evaluate_five_cards_hi,
        evaluate_omaha_five_cards_hi::evaluate_omaha_five_cards_hi,
        evaluate_seven_cards_hi::evaluate_seven_cards_hi,
    },
    types::{Evaluation, Hand, PlayerEval, Rank, Variant},
};

struct PlayerEvaluatedHand {
    id: String,
    result: SingleHandEval,
}

pub fn evaluate(hand: Hand) -> Evaluation {
    let hand_evals: Vec<PlayerEvaluatedHand> = hand
        .players
        .iter()
        .map(|player| match hand.variant {
            Variant::FiveCardDraw => PlayerEvaluatedHand {
                id: player.id.clone(),
                result: evaluate_five_cards_hi(&player.cards),
            },
            Variant::TexasHoldem => {
                let mut all_cards = player.cards.clone();
                all_cards.extend(hand.board.clone().unwrap_or_default());
                PlayerEvaluatedHand {
                    id: player.id.clone(),
                    result: evaluate_seven_cards_hi(&all_cards),
                }
            }
            Variant::FourCardOmahaHi => PlayerEvaluatedHand {
                id: player.id.clone(),
                result: evaluate_omaha_five_cards_hi(&player.cards /*, &hand.board*/),
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
    fn find_highest_rank_x_num_cards(values: &[usize; 13], num_cards: usize) -> Rank {
        let highest_idx = values
            .iter()
            .rev()
            .position(|&value| value == num_cards)
            .unwrap();
        match highest_idx {
            0 => Rank::Ace,
            1 => Rank::King,
            2 => Rank::Queen,
            3 => Rank::Jack,
            4 => Rank::Ten,
            5 => Rank::Nine,
            6 => Rank::Eight,
            7 => Rank::Seven,
            8 => Rank::Six,
            9 => Rank::Five,
            10 => Rank::Four,
            11 => Rank::Three,
            12 => Rank::Two,
            _ => unreachable!(),
        }
    }
    pub fn generate_hand_description(&self, pre_rank_value: usize, values: [usize; 13]) -> String {
        fn plural_adjust(rank: Rank) -> String {
            format!(
                "{}{}",
                rank.to_string(),
                if rank == Rank::Six { "es" } else { "s" }
            )
        }

        match true {
            _ if self.straight_flush => {
                if pre_rank_value == BROADWAY_STRAIGHT_INDICATOR {
                    return String::from("Royal Flush");
                } else if pre_rank_value == WHEEL_STRAIGHT_INDICATOR {
                    return String::from("Wheel Flush");
                }
                format!(
                    "{} High Straight Flush",
                    HandRanks::find_highest_rank_x_num_cards(&values, 1).to_string()
                )
            }
            _ if self.four_of_a_kind => {
                let rank = HandRanks::find_highest_rank_x_num_cards(&values, 4);
                format!("Four of a Kind {}", plural_adjust(rank))
            }
            _ if self.full_house => {
                let high_rank = HandRanks::find_highest_rank_x_num_cards(&values, 3);
                let low_rank = HandRanks::find_highest_rank_x_num_cards(&values, 2);
                format!(
                    "Full House, {} full of {}",
                    plural_adjust(high_rank),
                    plural_adjust(low_rank)
                )
            }
            _ if self.flush => {
                format!(
                    "{} High Flush",
                    HandRanks::find_highest_rank_x_num_cards(&values, 1).to_string()
                )
            }
            _ if self.straight => {
                if pre_rank_value == BROADWAY_STRAIGHT_INDICATOR {
                    return String::from("Broadway");
                } else if pre_rank_value == WHEEL_STRAIGHT_INDICATOR {
                    return String::from("Wheel");
                }
                format!(
                    "{} High Straight",
                    HandRanks::find_highest_rank_x_num_cards(&values, 1).to_string()
                )
            }
            _ if self.three_of_a_kind => {
                let rank = HandRanks::find_highest_rank_x_num_cards(&values, 3);
                format!("Three of a Kind {}", plural_adjust(rank))
            }
            _ if self.two_pair => {
                let high_rank = HandRanks::find_highest_rank_x_num_cards(&values, 2);
                let mut remaining_values = values;
                remaining_values[high_rank.clone() as usize] = 0;
                let low_rank = HandRanks::find_highest_rank_x_num_cards(&remaining_values, 2);
                format!(
                    "Two Pair, {} and {}",
                    plural_adjust(high_rank),
                    plural_adjust(low_rank)
                )
            }
            _ if self.pair => {
                let rank = HandRanks::find_highest_rank_x_num_cards(&values, 2);
                format!("Pair of {}", plural_adjust(rank),)
            }
            _ => {
                format!(
                    "High Card {}",
                    HandRanks::find_highest_rank_x_num_cards(&values, 1).to_string()
                )
            }
        }
    }
}

#[cfg(test)]
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
        let mut values = [0; 13];
        values[12] = 1;

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
            hand_rank.generate_hand_description(15_872, values),
            String::from("Royal Flush")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_wheel_flush() {
        let mut values = [0; 13];
        values[3] = 1;

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
            hand_rank.generate_hand_description(8_222, values),
            String::from("Wheel Flush")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_straight_flush() {
        let mut values = [0; 13];
        values[11] = 1;

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
            hand_rank.generate_hand_description(7_936, values),
            String::from("King High Straight Flush")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_four_of_a_kind() {
        let mut values = [0; 13];
        values[12] = 4;

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
            hand_rank.generate_hand_description(268_439_552, values),
            String::from("Four of a Kind Aces")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_four_of_a_kind_sixes() {
        let mut values = [0; 13];
        values[4] = 4;

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
            hand_rank.generate_hand_description(0, values),
            String::from("Four of a Kind Sixes")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_full_house() {
        let mut values = [0; 13];
        values[12] = 3;
        values[11] = 2;

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
            hand_rank.generate_hand_description(268_435_456, values),
            String::from("Full House, Aces full of Kings")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_full_house_sixes() {
        let mut values = [0; 13];
        values[4] = 3;
        values[2] = 2;

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
            hand_rank.generate_hand_description(0, values),
            String::from("Full House, Sixes full of Fours")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_flush() {
        let mut values = [0; 13];
        values[12] = 1;

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
            hand_rank.generate_hand_description(9_320, values),
            String::from("Ace High Flush")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_broadway() {
        let mut values = [0; 13];
        values[12] = 1;

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
            hand_rank.generate_hand_description(15_872, values),
            String::from("Broadway")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_wheel() {
        let mut values = [0; 13];
        values[3] = 1;

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
            hand_rank.generate_hand_description(8_222, values),
            String::from("Wheel")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_straight() {
        let mut values = [0; 13];
        values[11] = 1;

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
            hand_rank.generate_hand_description(7_936, values),
            String::from("King High Straight")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_three_of_a_kind() {
        let mut values = [0; 13];
        values[12] = 3;

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
            hand_rank.generate_hand_description(201_332_736, values),
            String::from("Three of a Kind Aces")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_three_of_a_kind_sixes() {
        let mut values = [0; 13];
        values[4] = 3;

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
            hand_rank.generate_hand_description(0, values),
            String::from("Three of a Kind Sixes")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_two_pair() {
        let mut values = [0; 13];
        values[12] = 2;
        values[11] = 2;

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
            hand_rank.generate_hand_description(201_328_640, values),
            String::from("Two Pair, Aces and Kings")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_two_pair_sixes_high() {
        let mut values = [0; 13];
        values[4] = 2;
        values[2] = 2;

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
            hand_rank.generate_hand_description(0, values),
            String::from("Two Pair, Sixes and Fours")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_two_pair_sixes_low() {
        let mut values = [0; 13];
        values[7] = 2;
        values[4] = 2;

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
            hand_rank.generate_hand_description(0, values),
            String::from("Two Pair, Nines and Sixes")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_pair() {
        let mut values = [0; 13];
        values[12] = 2;

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
        assert_eq!(
            hand_rank.generate_hand_description(134_224_896, values),
            String::from("Pair of Aces")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_pair_sixes() {
        let mut values = [0; 13];
        values[4] = 2;

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
        assert_eq!(
            hand_rank.generate_hand_description(0, values),
            String::from("Pair of Sixes")
        );
    }

    #[test]
    fn test_hand_ranks_impl_generate_hand_description_high_card() {
        let mut values = [0; 13];
        values[12] = 1;

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
            hand_rank.generate_hand_description(9_986, values),
            String::from("High Card Ace")
        );
    }

    #[test]
    fn test_hand_ranks_find_highest_rank_x_num_cards_ace_return() {
        let mut values = [0; 13];
        values[12] = 1;

        assert_eq!(
            HandRanks::find_highest_rank_x_num_cards(&values, 1),
            Rank::Ace,
        );
    }

    #[test]
    fn test_hand_ranks_find_highest_rank_x_num_cards_king_return() {
        let mut values = [0; 13];
        values[11] = 1;

        assert_eq!(
            HandRanks::find_highest_rank_x_num_cards(&values, 1),
            Rank::King,
        );
    }

    #[test]
    fn test_hand_ranks_find_highest_rank_x_num_cards_queen_return() {
        let mut values = [0; 13];
        values[10] = 1;

        assert_eq!(
            HandRanks::find_highest_rank_x_num_cards(&values, 1),
            Rank::Queen,
        );
    }

    #[test]
    fn test_hand_ranks_find_highest_rank_x_num_cards_jack_return() {
        let mut values = [0; 13];
        values[9] = 1;

        assert_eq!(
            HandRanks::find_highest_rank_x_num_cards(&values, 1),
            Rank::Jack,
        );
    }

    #[test]
    fn test_hand_ranks_find_highest_rank_x_num_cards_ten_return() {
        let mut values = [0; 13];
        values[8] = 1;

        assert_eq!(
            HandRanks::find_highest_rank_x_num_cards(&values, 1),
            Rank::Ten,
        );
    }

    #[test]
    fn test_hand_ranks_find_highest_rank_x_num_cards_nine_return() {
        let mut values = [0; 13];
        values[7] = 1;

        assert_eq!(
            HandRanks::find_highest_rank_x_num_cards(&values, 1),
            Rank::Nine,
        );
    }

    #[test]
    fn test_hand_ranks_find_highest_rank_x_num_cards_eight_return() {
        let mut values = [0; 13];
        values[6] = 1;

        assert_eq!(
            HandRanks::find_highest_rank_x_num_cards(&values, 1),
            Rank::Eight,
        );
    }

    #[test]
    fn test_hand_ranks_find_highest_rank_x_num_cards_seven_return() {
        let mut values = [0; 13];
        values[5] = 1;

        assert_eq!(
            HandRanks::find_highest_rank_x_num_cards(&values, 1),
            Rank::Seven,
        );
    }

    #[test]
    fn test_hand_ranks_find_highest_rank_x_num_cards_six_return() {
        let mut values = [0; 13];
        values[4] = 1;

        assert_eq!(
            HandRanks::find_highest_rank_x_num_cards(&values, 1),
            Rank::Six,
        );
    }

    #[test]
    fn test_hand_ranks_find_highest_rank_x_num_cards_five_return() {
        let mut values = [0; 13];
        values[3] = 1;

        assert_eq!(
            HandRanks::find_highest_rank_x_num_cards(&values, 1),
            Rank::Five,
        );
    }

    #[test]
    fn test_hand_ranks_find_highest_rank_x_num_cards_four_return() {
        let mut values = [0; 13];
        values[2] = 1;

        assert_eq!(
            HandRanks::find_highest_rank_x_num_cards(&values, 1),
            Rank::Four,
        );
    }

    #[test]
    fn test_hand_ranks_find_highest_rank_x_num_cards_three_return() {
        let mut values = [0; 13];
        values[1] = 1;

        assert_eq!(
            HandRanks::find_highest_rank_x_num_cards(&values, 1),
            Rank::Three,
        );
    }

    #[test]
    fn test_hand_ranks_find_highest_rank_x_num_cards_two_return() {
        let mut values = [0; 13];
        values[0] = 1;

        assert_eq!(
            HandRanks::find_highest_rank_x_num_cards(&values, 1),
            Rank::Two,
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
                        hand: String::from("Six High Straight"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Wheel"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Six High Straight"),
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
                        hand: String::from("Full House, Sixes full of Threes"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Full House, Fours full of Aces"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Full House, Sixes full of Threes"),
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
                        hand: String::from("Two Pair, Sixes and Fours"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Two Pair, Sixes and Fours"),
                    }
                ],
                winners: vec![String::from("player-1-id"), String::from("player-2-id")],
                winning_hand: String::from("Two Pair, Sixes and Fours"),
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
                        hand: String::from("Two Pair, Sixes and Fours"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Two Pair, Sixes and Fours"),
                    }
                ],
                winners: vec![String::from("player-2-id")],
                winning_hand: String::from("Two Pair, Sixes and Fours"),
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
                        hand: String::from("Ace High Flush"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Ace High Flush"),
                    }
                ],
                winners: vec![String::from("player-2-id")],
                winning_hand: String::from("Ace High Flush"),
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
                        hand: String::from("Ace High Flush"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Ace High Flush"),
                    }
                ],
                winners: vec![String::from("player-1-id"), String::from("player-2-id")],
                winning_hand: String::from("Ace High Flush"),
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
                        hand: String::from("Ace High Flush"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Seven High Straight"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Ace High Flush"),
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
                        hand: String::from("High Card Ace"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("High Card King"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("High Card Ace"),
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
                        hand: String::from("High Card Ace"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("High Card Ace"),
                    },
                    PlayerEval {
                        id: String::from("player-3-id"),
                        hand: String::from("High Card Ace"),
                    }
                ],
                winners: vec![
                    String::from("player-1-id"),
                    String::from("player-2-id"),
                    String::from("player-3-id")
                ],
                winning_hand: String::from("High Card Ace"),
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
                        hand: String::from("High Card Ace"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("High Card Ace"),
                    },
                    PlayerEval {
                        id: String::from("player-3-id"),
                        hand: String::from("High Card King"),
                    }
                ],
                winners: vec![String::from("player-1-id"), String::from("player-2-id")],
                winning_hand: String::from("High Card Ace"),
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
                        hand: String::from("High Card Ace"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("High Card Queen"),
                    },
                    PlayerEval {
                        id: String::from("player-3-id"),
                        hand: String::from("High Card King"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("High Card Ace"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_royal_flush_vs_pair_single_winner() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
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
                    rank: Rank::Two,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Heart,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
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
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Nine,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Nine,
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
                        hand: String::from("Pair of Nines"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Royal Flush"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_straight_flush_beats_quads() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
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
                    rank: Rank::Six,
                    suit: Suit::Club,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Heart,
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
                            rank: Rank::Six,
                            suit: Suit::Diamond,
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
                        hand: String::from("Nine High Straight Flush"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Four of a Kind Sixes"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Nine High Straight Flush"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_quads_beats_full_house() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::King,
                    suit: Suit::Diamond,
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
                    rank: Rank::Four,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Club,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::King,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Heart,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Nine,
                            suit: Suit::Diamond,
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
                        hand: String::from("Four of a Kind Kings"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Full House, Kings full of Fours"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Four of a Kind Kings"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_full_house_vs_full_house_no_chop_pair_kicker() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Queen,
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
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Heart,
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
                            rank: Rank::Nine,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Nine,
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
                        hand: String::from("Full House, Queens full of Fours"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Full House, Queens full of Nines"),
                    }
                ],
                winners: vec![String::from("player-2-id")],
                winning_hand: String::from("Full House, Queens full of Nines"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_flush_on_board_is_chopped() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Diamond,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Nine,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Heart,
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
                            suit: Suit::Heart,
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
                        hand: String::from("Ace High Flush"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Ace High Flush"),
                    }
                ],
                winners: vec![String::from("player-1-id"), String::from("player-2-id")],
                winning_hand: String::from("Ace High Flush"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_flush_vs_flush_no_chop_kicker() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Club,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::King,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Club,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Nine,
                            suit: Suit::Diamond,
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
                        hand: String::from("Ace High Flush"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Ace High Flush"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Ace High Flush"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_straight_vs_straight_no_chop() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Club,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ten,
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
                            rank: Rank::Five,
                            suit: Suit::Heart,
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
                        hand: String::from("Ten High Straight"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Nine High Straight"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Ten High Straight"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_trips_vs_trips_no_chop_kicker() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Club,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Nine,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::King,
                            suit: Suit::Club,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Nine,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Jack,
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
                        hand: String::from("Three of a Kind Nines"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Three of a Kind Nines"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Three of a Kind Nines"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_two_pair_on_board_chopped_via_matching_kicker() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::King,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Club,
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
                    rank: Rank::Two,
                    suit: Suit::Club,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Nine,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Heart,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Nine,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Heart,
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
                        hand: String::from("Two Pair, Kings and Fours"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Two Pair, Kings and Fours"),
                    }
                ],
                winners: vec![String::from("player-1-id"), String::from("player-2-id")],
                winning_hand: String::from("Two Pair, Kings and Fours"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_two_pair_vs_two_pair_no_chop_kicker() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::King,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Club,
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
                    rank: Rank::Two,
                    suit: Suit::Club,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Nine,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Heart,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Jack,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Heart,
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
                        hand: String::from("Two Pair, Kings and Fours"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Two Pair, Kings and Fours"),
                    }
                ],
                winners: vec![String::from("player-2-id")],
                winning_hand: String::from("Two Pair, Kings and Fours"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_pair_beats_high_card_single_winner() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Jack,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Spade,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Seven,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Heart,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Diamond,
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
                        hand: String::from("Pair of Sevens"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("High Card Jack"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Pair of Sevens"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_high_card_vs_high_card_no_chop_kicker() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Heart,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Queen,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Seven,
                            suit: Suit::Diamond,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Jack,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Diamond,
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
                        hand: String::from("High Card Ace"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("High Card Ace"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("High Card Ace"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_three_player_royal_flush_on_board_all_chopped() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
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
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Heart,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Heart,
                        },
                    ],
                },
                Player {
                    id: String::from("player-3-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Seven,
                            suit: Suit::Heart,
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
                    },
                    PlayerEval {
                        id: String::from("player-3-id"),
                        hand: String::from("Royal Flush"),
                    }
                ],
                winners: vec![
                    String::from("player-1-id"),
                    String::from("player-2-id"),
                    String::from("player-3-id")
                ],
                winning_hand: String::from("Royal Flush"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_three_player_two_way_chop_one_loser() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Diamond,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Heart,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Spade,
                        },
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Heart,
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
                            suit: Suit::Diamond,
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
                        hand: String::from("Nine High Straight"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Nine High Straight"),
                    },
                    PlayerEval {
                        id: String::from("player-3-id"),
                        hand: String::from("High Card Ace"),
                    }
                ],
                winners: vec![String::from("player-1-id"), String::from("player-2-id")],
                winning_hand: String::from("Nine High Straight"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_three_player_quads_on_board_chop_hole_cards_irrelevant() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Heart,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Two,
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
                            rank: Rank::Four,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Diamond,
                        },
                    ],
                },
                Player {
                    id: String::from("player-3-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Seven,
                            suit: Suit::Diamond,
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
                        hand: String::from("Four of a Kind Eights"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Four of a Kind Eights"),
                    },
                    PlayerEval {
                        id: String::from("player-3-id"),
                        hand: String::from("Four of a Kind Eights"),
                    }
                ],
                winners: vec![
                    String::from("player-1-id"),
                    String::from("player-2-id"),
                    String::from("player-3-id")
                ],
                winning_hand: String::from("Four of a Kind Eights"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_three_player_straight_flush_on_board_chop_hole_cards_irrelevant()
    {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
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
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Two,
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
                            rank: Rank::Four,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Club,
                        },
                    ],
                },
                Player {
                    id: String::from("player-3-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Six,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Seven,
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
                        hand: String::from("Nine High Straight Flush"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Nine High Straight Flush"),
                    },
                    PlayerEval {
                        id: String::from("player-3-id"),
                        hand: String::from("Nine High Straight Flush"),
                    }
                ],
                winners: vec![
                    String::from("player-1-id"),
                    String::from("player-2-id"),
                    String::from("player-3-id")
                ],
                winning_hand: String::from("Nine High Straight Flush"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_wheel_flush_on_board_chop_hole_cards_irrelevant() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Diamond,
                },
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
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Nine,
                            suit: Suit::Club,
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
                            rank: Rank::King,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Queen,
                            suit: Suit::Heart,
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
                        hand: String::from("Wheel Flush"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Wheel Flush"),
                    }
                ],
                winners: vec![String::from("player-1-id"), String::from("player-2-id")],
                winning_hand: String::from("Wheel Flush"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_straight_on_board_chop_despite_incidental_pair() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Eight,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Six,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Club,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Heart,
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
                            rank: Rank::King,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Queen,
                            suit: Suit::Diamond,
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
                        hand: String::from("Nine High Straight"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Nine High Straight"),
                    }
                ],
                winners: vec![String::from("player-1-id"), String::from("player-2-id")],
                winning_hand: String::from("Nine High Straight"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_wheel_on_board_chop_hole_cards_irrelevant() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Four,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Diamond,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Nine,
                            suit: Suit::Club,
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
                            rank: Rank::King,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Queen,
                            suit: Suit::Heart,
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
    fn test_evaluate_texas_holdem_two_player_six_high_straight_beats_wheel() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Four,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Three,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Club,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Ace,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Six,
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
        };
        assert_eq!(
            evaluate(hand),
            Evaluation {
                id: String::from("hand-id"),
                players: vec![
                    PlayerEval {
                        id: String::from("player-1-id"),
                        hand: String::from("Six High Straight"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Wheel"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Six High Straight"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_high_card_board_chop_hole_cards_below_kicker() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Spade,
                },
                Card {
                    rank: Rank::Five,
                    suit: Suit::Heart,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Four,
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
                            rank: Rank::Four,
                            suit: Suit::Heart,
                        },
                        Card {
                            rank: Rank::Two,
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
                        hand: String::from("High Card Ace"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("High Card Ace"),
                    }
                ],
                winners: vec![String::from("player-1-id"), String::from("player-2-id")],
                winning_hand: String::from("High Card Ace"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_two_pair_board_chop_ace_kicker_hole_cards_irrelevant()
    {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::King,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Club,
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
                    rank: Rank::Ace,
                    suit: Suit::Heart,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Nine,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Heart,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Seven,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Two,
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
                        hand: String::from("Two Pair, Kings and Fours"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Two Pair, Kings and Fours"),
                    }
                ],
                winners: vec![String::from("player-1-id"), String::from("player-2-id")],
                winning_hand: String::from("Two Pair, Kings and Fours"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_trips_board_chop_unbeatable_kickers() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Nine,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Diamond,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Two,
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
                            rank: Rank::Four,
                            suit: Suit::Heart,
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
                        hand: String::from("Three of a Kind Nines"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Three of a Kind Nines"),
                    }
                ],
                winners: vec![String::from("player-1-id"), String::from("player-2-id")],
                winning_hand: String::from("Three of a Kind Nines"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_pair_board_chop_unbeatable_kickers() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Seven,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Heart,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Queen,
                    suit: Suit::Club,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Two,
                            suit: Suit::Club,
                        },
                        Card {
                            rank: Rank::Three,
                            suit: Suit::Heart,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Diamond,
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
                        hand: String::from("Pair of Sevens"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Pair of Sevens"),
                    }
                ],
                winners: vec![String::from("player-1-id"), String::from("player-2-id")],
                winning_hand: String::from("Pair of Sevens"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_flush_beats_straight_single_winner() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
                Card {
                    rank: Rank::Ace,
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
                    rank: Rank::Nine,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::Two,
                    suit: Suit::Heart,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::King,
                            suit: Suit::Diamond,
                        },
                        Card {
                            rank: Rank::Five,
                            suit: Suit::Diamond,
                        },
                    ],
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Four,
                            suit: Suit::Heart,
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
                        hand: String::from("Ace High Flush"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Six High Straight"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Ace High Flush"),
            }
        );
    }

    #[test]
    fn test_evaluate_texas_holdem_two_player_straight_flush_vs_straight_flush_no_chop_high_card() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            board: Some(vec![
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
                    rank: Rank::Two,
                    suit: Suit::Club,
                },
            ]),
            burn_cards: None,
            discarded_cards: None,
            remaining_deck: vec![],
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: vec![
                        Card {
                            rank: Rank::Nine,
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
                            rank: Rank::Four,
                            suit: Suit::Heart,
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
                        hand: String::from("Nine High Straight Flush"),
                    },
                    PlayerEval {
                        id: String::from("player-2-id"),
                        hand: String::from("Eight High Straight Flush"),
                    }
                ],
                winners: vec![String::from("player-1-id")],
                winning_hand: String::from("Nine High Straight Flush"),
            }
        );
    }
}
