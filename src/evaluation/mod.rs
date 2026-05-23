use crate::{
    constants::{
        ACE_VALUE, BROADWAY_STRAIGHT_INDICATOR, NUMBER_RANKS, NUMBER_SUITS, RANK_BASE_VALUE,
        WHEEL_STRAIGHT_INDICATOR,
    },
    types::{Card, Evaluation, PlayerEval},
};

// NOTE: within eval detect type, then send to 5 card, which in turn just does 5 hi as below, fine
// for now
// while do this how structure this? have a 5 card hi file for exmaple? a five card draw file for
// exmaple? but keep structs/enums here for now? decide pre further testing me thinks? or at least
// think on, nah make tests then lift and shift lollll
// pub fn evaluate(hand: &Hand) -> Evaluation {} // NOTE: big testing needed here

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
    high_card: bool,
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

fn evaluate_five_cards_hi(cards: Vec<Card>) -> SingleHandEval {
    let mut suits = [0; NUMBER_SUITS];
    let mut values = [0; NUMBER_RANKS];

    for card in cards {
        let suits_idx = (card.matrix_value() as f64 / NUMBER_RANKS as f64).floor() as usize;
        suits[suits_idx] += 1;

        let values_idx = (card.matrix_value() as f64 % NUMBER_RANKS as f64) as usize;
        values[values_idx] += 1;
    }

    let pre_adjustment_rank_value = values
        .iter()
        .enumerate()
        .fold(0usize, |acc, (idx, &value)| {
            let mut acc = acc;
            let base = 2usize.pow((idx + 1) as u32);

            if value == 1 {
                acc += base;
            }
            if value > 1 {
                acc += base * ACE_VALUE * value;
            }
            acc
        });

    let first_card_idx = values.iter().position(|&value| value == 1);

    let mut hand_ranks = HandRanks {
        straight_flush: false,
        four_of_a_kind: values.iter().position(|&value| value == 4).is_some(),
        full_house: values.iter().filter(|&&value| value != 0).count() == 2,
        flush: suits.iter().position(|&suit| suit == 5).is_some(),
        straight: pre_adjustment_rank_value == WHEEL_STRAIGHT_INDICATOR
            || first_card_idx.is_some_and(|idx| {
                values
                    .get(idx..idx + 5)
                    .is_some_and(|slice| slice.iter().all(|&value| value == 1))
            }),
        three_of_a_kind: values.iter().position(|&value| value == 3).is_some(),
        two_pair: values.iter().filter(|&&value| value == 2).count() == 2,
        pair: values.iter().filter(|&&value| value == 2).count() == 1,
        high_card: true,
    };
    hand_ranks.straight_flush = hand_ranks.flush && hand_ranks.straight;

    let hand_rank_multiplier = hand_ranks.obtain_hand_multiplier();

    let mut post_adjusment_rank_value =
        pre_adjustment_rank_value + hand_rank_multiplier * RANK_BASE_VALUE;
    if post_adjusment_rank_value == WHEEL_STRAIGHT_INDICATOR {
        post_adjusment_rank_value -= ACE_VALUE - 1;
    };
    if hand_rank_multiplier == HandRankMultipliers::FullHouse as usize {
        let full_house_pair_index = values.iter().position(|&value| value == 2).unwrap();
        post_adjusment_rank_value -= 2usize.pow(full_house_pair_index as u32 + 1)
            * ACE_VALUE
            * 2
            * (full_house_pair_index + 1);
    };

    SingleHandEval {
        hand_description: hand_ranks.generate_hand_description(pre_adjustment_rank_value),
        rank_value: post_adjusment_rank_value,
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{Rank, Suit};

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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
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
            high_card: true,
        };
        assert_eq!(
            hand_rank.generate_hand_description(1),
            String::from("High Card")
        );
    }

    #[test]
    fn test_evaluate_five_cards_hi_royal_flush() {
        let cards = vec![
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
        ];
        assert_eq!(
            evaluate_five_cards_hi(cards),
            SingleHandEval {
                hand_description: String::from("Royal Flush"),
                rank_value: 9_000_015_872
            }
        );
    }

    #[test]
    fn test_evaluate_five_cards_hi_wheel_flush() {
        let cards = vec![
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
                rank: Rank::Five,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Diamond,
            },
        ];
        assert_eq!(
            evaluate_five_cards_hi(cards),
            SingleHandEval {
                hand_description: String::from("Wheel Flush"),
                rank_value: 9_000_008_222
            }
        );
    }

    #[test]
    fn test_evaluate_five_cards_hi_straight_flush() {
        let cards = vec![
            Card {
                rank: Rank::Six,
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
                rank: Rank::Five,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Diamond,
            },
        ];
        assert_eq!(
            evaluate_five_cards_hi(cards),
            SingleHandEval {
                hand_description: String::from("Straight Flush"),
                rank_value: 9_000_000_062
            }
        );
    }

    #[test]
    fn test_evaluate_five_cards_hi_four_of_a_kind() {
        let cards = vec![
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
                rank: Rank::Six,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Diamond,
            },
        ];
        assert_eq!(
            evaluate_five_cards_hi(cards),
            SingleHandEval {
                hand_description: String::from("Four of a Kind"),
                rank_value: 8_001_048_584
            }
        );
    }

    #[test]
    fn test_evaluate_five_cards_hi_full_house() {
        let cards = vec![
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
                rank: Rank::Four,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Diamond,
            },
        ];
        assert_eq!(
            evaluate_five_cards_hi(cards),
            SingleHandEval {
                hand_description: String::from("Full House"),
                rank_value: 7_000_524_288
            }
        );
    }

    #[test]
    fn test_evaluate_five_cards_hi_flush() {
        let cards = vec![
            Card {
                rank: Rank::Six,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            },
        ];
        assert_eq!(
            evaluate_five_cards_hi(cards),
            SingleHandEval {
                hand_description: String::from("Flush"),
                rank_value: 6_000_009_320
            }
        );
    }

    #[test]
    fn test_evaluate_five_cards_hi_broadway() {
        let cards = vec![
            Card {
                rank: Rank::Ten,
                suit: Suit::Club,
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
        ];
        assert_eq!(
            evaluate_five_cards_hi(cards),
            SingleHandEval {
                hand_description: String::from("Broadway"),
                rank_value: 5_000_015_872
            }
        );
    }

    #[test]
    fn test_evaluate_five_cards_hi_wheel() {
        let cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Club,
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
        ];
        assert_eq!(
            evaluate_five_cards_hi(cards),
            SingleHandEval {
                hand_description: String::from("Wheel"),
                rank_value: 5_000_008_222
            }
        );
    }

    #[test]
    fn test_evaluate_five_cards_hi_straight() {
        let cards = vec![
            Card {
                rank: Rank::King,
                suit: Suit::Club,
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
        ];
        assert_eq!(
            evaluate_five_cards_hi(cards),
            SingleHandEval {
                hand_description: String::from("Straight"),
                rank_value: 5_000_007_936
            }
        );
    }

    #[test]
    fn test_evaluate_five_cards_hi_three_of_a_kind() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Diamond,
            },
        ];
        assert_eq!(
            evaluate_five_cards_hi(cards),
            SingleHandEval {
                hand_description: String::from("Three of a Kind"),
                rank_value: 4_006_292_992
            }
        );
    }

    #[test]
    fn test_evaluate_five_cards_hi_two_pair() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Diamond,
            },
        ];
        assert_eq!(
            evaluate_five_cards_hi(cards),
            SingleHandEval {
                hand_description: String::from("Two Pair"),
                rank_value: 3_012_583_936
            }
        );
    }

    #[test]
    fn test_evaluate_five_cards_hi_pair() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Diamond,
            },
        ];
        assert_eq!(
            evaluate_five_cards_hi(cards),
            SingleHandEval {
                hand_description: String::from("Pair"),
                rank_value: 2_008_398_080
            }
        );
    }

    #[test]
    fn test_evaluate_five_cards_hi_high_card() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Diamond,
            },
        ];
        assert_eq!(
            evaluate_five_cards_hi(cards),
            SingleHandEval {
                hand_description: String::from("High Card"),
                rank_value: 1_000_009_986
            }
        );
    }
}
