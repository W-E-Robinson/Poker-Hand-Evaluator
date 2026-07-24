use crate::{
    constants::{ACE_VALUE, NUMBER_RANKS, NUMBER_SUITS, RANK_BASE_VALUE, WHEEL_STRAIGHT_INDICATOR},
    evaluation::{HandRankMultipliers, HandRanks, SingleHandEval},
    types::Card,
};

pub fn evaluate_seven_cards_hi(cards: &Vec<Card>) -> SingleHandEval {
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
    };
    hand_ranks.straight_flush = hand_ranks.flush && hand_ranks.straight;

    let hand_rank_multiplier = hand_ranks.obtain_hand_multiplier();

    let mut post_adjustment_rank_value =
        pre_adjustment_rank_value + hand_rank_multiplier * RANK_BASE_VALUE;
    if pre_adjustment_rank_value == WHEEL_STRAIGHT_INDICATOR {
        post_adjustment_rank_value -= ACE_VALUE - 1;
    };
    if hand_rank_multiplier == HandRankMultipliers::FullHouse as usize {
        let full_house_pair_index = values.iter().position(|&value| value == 2).unwrap();
        post_adjustment_rank_value -= 2usize.pow(full_house_pair_index as u32 + 1)
            * ACE_VALUE
            * 2
            * (full_house_pair_index + 1);
    };

    SingleHandEval {
        hand_description: hand_ranks.generate_hand_description(pre_adjustment_rank_value),
        rank_value: post_adjustment_rank_value,
    }
}

mod tests {
    use crate::types::{Rank, Suit};

    use super::*;

    #[test]
    fn test_evaluate_seven_cards_hi_royal_flush_with_unrelated_kickers() {
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
            Card {
                rank: Rank::Two,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Spade,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Royal Flush"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_royal_flush_with_extra_suited_card() {
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
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Royal Flush"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_royal_flush_with_paired_kickers() {
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
            Card {
                rank: Rank::Nine,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Royal Flush"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_royal_flush_from_seven_card_flush() {
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
            Card {
                rank: Rank::Eight,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Diamond,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Royal Flush"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_straight_flush_with_unrelated_kickers() {
        let cards = vec![
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
                rank: Rank::Two,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Spade,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Straight Flush"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_straight_flush_with_extra_suited_card() {
        let cards = vec![
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
                rank: Rank::Two,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Straight Flush"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_straight_flush_with_overlapping_flush_run() {
        let cards = vec![
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
                rank: Rank::King,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Straight Flush"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_straight_flush_with_trips_among_kickers() {
        let cards = vec![
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
                rank: Rank::Nine,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Straight Flush"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_wheel_flush_with_unrelated_kickers() {
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
                rank: Rank::Four,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Spade,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Wheel Flush"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_wheel_flush_with_paired_kickers() {
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
                rank: Rank::Four,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Five,
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
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Wheel Flush"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_four_of_a_kind_with_unrelated_kickers() {
        let cards = vec![
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
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Spade,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Four of a Kind"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_four_of_a_kind_with_paired_kickers() {
        let cards = vec![
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
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Spade,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Four of a Kind"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_four_of_a_kind_with_trips_among_kickers() {
        let cards = vec![
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
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Heart,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Four of a Kind"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_full_house_with_unrelated_kickers() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
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
                rank: Rank::Five,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Spade,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Full House"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_full_house_with_two_trips() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
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
                rank: Rank::Five,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Spade,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Full House"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_full_house_with_trips_and_two_pairs() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
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
                rank: Rank::King,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Diamond,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Full House"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_three_pair_is_not_a_full_house() {
        let cards = vec![
            Card {
                rank: Rank::King,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Queen,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Queen,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Two Pair"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_flush_with_unrelated_kickers() {
        let cards = vec![
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
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Spade,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Flush"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_flush_with_extra_suited_card() {
        let cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Nine,
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
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Flush"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_flush_with_paired_kicker() {
        let cards = vec![
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
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Spade,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Flush"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_flush_with_trips_among_ranks() {
        let cards = vec![
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
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Heart,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Flush"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_flush_with_coincidental_straight() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Five,
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
                rank: Rank::Eight,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Spade,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Flush"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_straight_with_unrelated_kickers() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
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
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Heart,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Straight"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_straight_with_duplicate_rank_kicker() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
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
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Heart,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Straight"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_straight_with_overlapping_run() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
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
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Heart,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Straight"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_straight_with_trips_among_kickers() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
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
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Straight"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_broadway_with_unrelated_kickers() {
        let cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
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
                rank: Rank::Jack,
                suit: Suit::Spade,
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
                rank: Rank::Nine,
                suit: Suit::Heart,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Broadway"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_wheel_with_unrelated_kickers() {
        let cards = vec![
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
            Card {
                rank: Rank::Nine,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Heart,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Wheel"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_three_of_a_kind_with_unrelated_kickers() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
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
                rank: Rank::Two,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Heart,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Three of a Kind"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_three_of_a_kind_with_near_straight_kickers() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
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
                rank: Rank::Eight,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Heart,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Three of a Kind"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_trips_with_coincidental_straight_is_not_three_of_a_kind() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
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
                rank: Rank::Eight,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Heart,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Straight"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_two_sets_of_three_of_a_kind_is_a_full_house() {
        let cards = vec![
            Card {
                rank: Rank::Jack,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Four,
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
                rank: Rank::Two,
                suit: Suit::Spade,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Full House"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_two_pair_with_unrelated_kickers() {
        let cards = vec![
            Card {
                rank: Rank::King,
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
                rank: Rank::Queen,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Two Pair"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_two_pair_with_three_pairs_present() {
        let cards = vec![
            Card {
                rank: Rank::Ten,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Ten,
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
                rank: Rank::Six,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Two Pair"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_two_pair_with_coincidental_straight_is_not_two_pair() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Nine,
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
                rank: Rank::Seven,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Straight"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_pair_with_unrelated_kickers() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Pair"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_pair_with_near_straight_kickers() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Pair"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_pair_with_coincidental_straight_is_not_a_pair() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Straight"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_pair_with_coincidental_flush_is_not_a_pair() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Club,
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
            Card {
                rank: Rank::King,
                suit: Suit::Spade,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("Flush"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_high_card_basic() {
        let cards = vec![
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
                rank: Rank::Seven,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("High Card"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_high_card_with_near_flush() {
        let cards = vec![
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
                rank: Rank::King,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Heart,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("High Card"),
                rank_value: 0
            }
        );
    }

    #[test]
    fn test_evaluate_seven_cards_hi_high_card_with_near_straight() {
        let cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Diamond,
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
                rank: Rank::King,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_seven_cards_hi(&cards),
            SingleHandEval {
                hand_description: String::from("High Card"),
                rank_value: 0
            }
        );
    }
}
