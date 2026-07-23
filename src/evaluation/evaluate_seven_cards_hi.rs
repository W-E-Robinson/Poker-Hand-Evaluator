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
    use crate::types::{Player, Rank, Suit};

    use super::*;
}
