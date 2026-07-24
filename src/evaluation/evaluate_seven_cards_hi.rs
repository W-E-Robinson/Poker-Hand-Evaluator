use itertools::Itertools;

use crate::{
    evaluation::{evaluate_five_cards_hi::evaluate_five_cards_hi, SingleHandEval},
    types::Card,
};

pub fn evaluate_seven_cards_hi(cards: &Vec<Card>) -> SingleHandEval {
    let combinations = cards.iter().combinations(5);
    let mut evals = Vec::new();

    for comb in combinations.into_iter() {
        let comb: Vec<Card> = comb.into_iter().cloned().collect();
        evals.push(evaluate_five_cards_hi(&comb));
    }

    evals
        .into_iter()
        .max_by_key(|eval| eval.rank_value)
        .unwrap()
}

#[cfg(test)]
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
                rank_value: 9_000_015_872
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
                rank_value: 9_000_015_872
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
                rank_value: 9_000_015_872
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
                rank_value: 9_000_015_872
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
                rank_value: 9_000_000_496
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
                rank_value: 9_000_000_496
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
                rank_value: 9_000_000_496
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
                rank_value: 9_000_000_496
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
                rank_value: 9_000_000_031
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
                rank_value: 9_000_000_031
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
                rank_value: 8_008_392_704
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
                rank_value: 8_008_392_704
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
                rank_value: 8_008_388_624
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
                rank_value: 7_006_291_460
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
                rank_value: 7_006_291_460
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
                rank_value: 7_006_291_468
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
                rank_value: 3_100_664_320
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
                rank_value: 6_000_009_362
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
                rank_value: 6_000_009_616
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
                rank_value: 6_000_009_362
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
                rank_value: 6_000_009_362
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
                rank_value: 6_000_000_342
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
                rank_value: 5_000_000_496
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
                rank_value: 5_000_000_496
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
                rank_value: 5_000_000_496
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
                rank_value: 5_000_000_496
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
                rank_value: 5_000_015_872
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
                rank_value: 5_000_000_031
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
                rank_value: 4_006_296_576
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
                rank_value: 4_006_295_680
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
                rank_value: 5_000_000_496
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
                rank_value: 7_025_165_827
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
                rank_value: 3_100_663_552
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
                rank_value: 3_010_485_792
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
                rank_value: 5_000_000_496
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
                rank_value: 2_004_199_488
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
                rank_value: 2_004_198_592
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
                rank_value: 5_000_000_496
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
                rank_value: 6_000_001_426
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
                rank_value: 1_000_012_624
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
                rank_value: 1_000_013_696
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
                rank_value: 1_000_004_576
            }
        );
    }
}
