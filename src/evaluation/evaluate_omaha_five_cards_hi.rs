use itertools::Itertools;

use crate::{
    evaluation::{evaluate_five_cards_hi::evaluate_five_cards_hi, SingleHandEval},
    types::Card,
};

pub fn evaluate_omaha_five_cards_hi(
    player_cards: &Vec<Card>,
    board_cards: &Vec<Card>,
) -> SingleHandEval {
    let mut evals = Vec::new();

    for hole in player_cards.iter().combinations(2) {
        for board in board_cards.iter().combinations(3) {
            let mut hand: Vec<Card> = hole.clone().into_iter().cloned().collect();
            hand.extend(board.into_iter().cloned());

            evals.push(evaluate_five_cards_hi(&hand));
        }
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
    fn test_evaluate_omaha_five_cards_hi_royal_flush() {
        let player_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Spade,
            },
        ];
        let board_cards = vec![
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
                rank: Rank::Three,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_omaha_five_cards_hi(&player_cards, &board_cards),
            SingleHandEval {
                hand_description: String::from("Royal Flush"),
                rank_value: 9_000_015_872
            }
        );
    }

    #[test]
    fn test_evaluate_omaha_five_cards_hi_straight_flush() {
        let player_cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Spade,
            },
        ];
        let board_cards = vec![
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
                rank: Rank::Three,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_omaha_five_cards_hi(&player_cards, &board_cards),
            SingleHandEval {
                hand_description: String::from("Nine High Straight Flush"),
                rank_value: 9_000_000_496
            }
        );
    }

    #[test]
    fn test_evaluate_omaha_five_cards_hi_four_of_a_kind_using_pair_from_hole_and_pair_from_board() {
        let player_cards = vec![
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
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Spade,
            },
        ];
        let board_cards = vec![
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_omaha_five_cards_hi(&player_cards, &board_cards),
            SingleHandEval {
                hand_description: String::from("Four of a Kind Kings"),
                rank_value: 8_134_217_984
            }
        );
    }

    #[test]
    fn test_evaluate_omaha_five_cards_hi_full_house() {
        let player_cards = vec![
            Card {
                rank: Rank::Queen,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Queen,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Spade,
            },
        ];
        let board_cards = vec![
            Card {
                rank: Rank::Queen,
                suit: Suit::Club,
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
                rank: Rank::Four,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_omaha_five_cards_hi(&player_cards, &board_cards),
            SingleHandEval {
                hand_description: String::from("Full House, Queens full of Jacks"),
                rank_value: 7_050_331_658
            }
        );
    }

    #[test]
    fn test_evaluate_omaha_five_cards_hi_flush() {
        let player_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Spade,
            },
        ];
        let board_cards = vec![
            Card {
                rank: Rank::King,
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
                rank: Rank::Jack,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_omaha_five_cards_hi(&player_cards, &board_cards),
            SingleHandEval {
                hand_description: String::from("Ace High Flush"),
                rank_value: 6_000_012_364
            }
        );
    }

    #[test]
    fn test_evaluate_omaha_five_cards_hi_straight() {
        let player_cards = vec![
            Card {
                rank: Rank::Ten,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Spade,
            },
        ];
        let board_cards = vec![
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
                rank: Rank::King,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_omaha_five_cards_hi(&player_cards, &board_cards),
            SingleHandEval {
                hand_description: String::from("Ten High Straight"),
                rank_value: 5_000_000_992
            }
        );
    }

    #[test]
    fn test_evaluate_omaha_five_cards_hi_three_of_a_kind_using_one_hole_card_and_a_board_pair() {
        let player_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Queen,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Spade,
            },
        ];
        let board_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamond,
            },
        ];
        assert_eq!(
            evaluate_omaha_five_cards_hi(&player_cards, &board_cards),
            SingleHandEval {
                hand_description: String::from("Three of a Kind Aces"),
                rank_value: 4_201_332_736
            }
        );
    }

    #[test]
    fn test_evaluate_omaha_five_cards_hi_two_pair_using_one_pair_from_hole_and_one_from_board() {
        let player_cards = vec![
            Card {
                rank: Rank::King,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Spade,
            },
        ];
        let board_cards = vec![
            Card {
                rank: Rank::Queen,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Queen,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_omaha_five_cards_hi(&player_cards, &board_cards),
            SingleHandEval {
                hand_description: String::from("Two Pair, Kings and Queens"),
                rank_value: 3_100_663_360
            }
        );
    }

    #[test]
    fn test_evaluate_omaha_five_cards_hi_pair_using_one_hole_ace_and_a_board_ace() {
        let player_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Spade,
            },
        ];
        let board_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamond,
            },
        ];
        assert_eq!(
            evaluate_omaha_five_cards_hi(&player_cards, &board_cards),
            SingleHandEval {
                hand_description: String::from("Pair of Aces"),
                rank_value: 2_134_222_976
            }
        );
    }

    #[test]
    fn test_evaluate_omaha_five_cards_hi_high_card() {
        let player_cards = vec![
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Spade,
            },
        ];
        let board_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamond,
            },
        ];
        assert_eq!(
            evaluate_omaha_five_cards_hi(&player_cards, &board_cards),
            SingleHandEval {
                hand_description: String::from("High Card Ace"),
                rank_value: 1_000_013_696
            }
        );
    }

    #[test]
    fn test_evaluate_omaha_five_cards_hi_trip_in_hole_only_plays_as_a_pair() {
        let player_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Heart,
            },
        ];
        let board_cards = vec![
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
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamond,
            },
        ];
        assert_eq!(
            evaluate_omaha_five_cards_hi(&player_cards, &board_cards),
            SingleHandEval {
                hand_description: String::from("Pair of Aces"),
                rank_value: 2_134_224_896
            }
        );
    }

    #[test]
    fn test_evaluate_omaha_five_cards_hi_two_pair_in_hole_only_plays_as_a_pair() {
        let player_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Diamond,
            },
        ];
        let board_cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Heart,
            },
        ];
        assert_eq!(
            evaluate_omaha_five_cards_hi(&player_cards, &board_cards),
            SingleHandEval {
                hand_description: String::from("Pair of Aces"),
                rank_value: 2_134_218_056
            }
        );
    }

    #[test]
    fn test_evaluate_omaha_five_cards_hi_four_flush_in_hole_is_not_a_flush_without_three_on_board()
    {
        let player_cards = vec![
            Card {
                rank: Rank::Two,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Queen,
                suit: Suit::Heart,
            },
        ];
        let board_cards = vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Spade,
            },
        ];
        assert_eq!(
            evaluate_omaha_five_cards_hi(&player_cards, &board_cards),
            SingleHandEval {
                hand_description: String::from("High Card Ace"),
                rank_value: 1_000_015_616
            }
        );
    }

    #[test]
    fn test_evaluate_omaha_five_cards_hi_one_suited_hole_card_is_not_a_flush_despite_four_suited_on_board(
    ) {
        let player_cards = vec![
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Club,
            },
        ];
        let board_cards = vec![
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
                rank: Rank::Four,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Club,
            },
        ];
        assert_eq!(
            evaluate_omaha_five_cards_hi(&player_cards, &board_cards),
            SingleHandEval {
                hand_description: String::from("High Card Ace"),
                rank_value: 1_000_014_080
            }
        );
    }

    #[test]
    fn test_evaluate_omaha_five_cards_hi_straight_needing_three_hole_cards_is_not_a_straight() {
        let player_cards = vec![
            Card {
                rank: Rank::Five,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Spade,
            },
        ];
        let board_cards = vec![
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
                rank: Rank::Three,
                suit: Suit::Diamond,
            },
        ];
        assert_eq!(
            evaluate_omaha_five_cards_hi(&player_cards, &board_cards),
            SingleHandEval {
                hand_description: String::from("High Card King"),
                rank_value: 1_000_005_568
            }
        );
    }

    #[test]
    fn test_evaluate_omaha_five_cards_hi_trip_on_board_uses_hole_cards_only_as_kickers() {
        let player_cards = vec![
            Card {
                rank: Rank::Two,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            },
        ];
        let board_cards = vec![
            Card {
                rank: Rank::Queen,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Queen,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Queen,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Diamond,
            },
        ];
        assert_eq!(
            evaluate_omaha_five_cards_hi(&player_cards, &board_cards),
            SingleHandEval {
                hand_description: String::from("Three of a Kind Queens"),
                rank_value: 4_050_336_000
            }
        );
    }
}
