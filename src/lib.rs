use crate::{
    types::{Error, ErrorType, Evaluation, Hand, PlayerEval},
    validation::validate,
};

mod constants;
mod evaluation;
mod types;
mod validation;

pub fn evaluate_hand(hand: Hand) -> Result<Evaluation, Error> {
    if let Err(err) = validate(&hand) {
        return Err(Error {
            id: hand.id,
            error_type: ErrorType::Validation,
            message: err,
        });
    }

    Ok(Evaluation {
        id: String::from("id"),
        players: vec![PlayerEval {
            id: String::from("id"),
            hand: String::from("a hand"),
        }],
        winners: vec![String::from("id")],
        winning_hand: String::from("a hand"),
    })
}

#[cfg(test)]
mod tests {
    use crate::types::{Card, Rank, Suit, Variant};

    use super::*;

    #[test]
    fn test_validation_error() {
        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            players: vec![],
            board: Some(vec![Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            }]),
            burn_cards: None,
            discarded_cards: Some(vec![Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            }]),
            remaining_deck: vec![Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            }],
        };

        let result = evaluate_hand(hand).unwrap_err();
        assert_eq!(
            result,
            Error {
                id: String::from("hand-id"),
                error_type: ErrorType::Validation,
                message: String::from("At least one player must be provided."),
            }
        );
    }

    // NOTE: have one simple evaluation test
}
