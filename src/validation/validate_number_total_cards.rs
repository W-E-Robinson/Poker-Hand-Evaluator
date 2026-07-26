use crate::{constants::NUMBER_CARDS_DECK, types::Hand};

pub fn validate_number_total_cards(hand: &Hand) -> Result<(), String> {
    let mut total_num_cards = 0;

    for player in hand.players.iter() {
        total_num_cards += player.cards.len();
    }

    if let Some(board) = &hand.board {
        total_num_cards += board.len();
    }

    if let Some(burn_cards) = &hand.burn_cards {
        total_num_cards += burn_cards.len();
    }

    if let Some(discarded_cards) = &hand.discarded_cards {
        total_num_cards += discarded_cards.len();
    }

    total_num_cards += hand.remaining_deck.len();

    if total_num_cards != NUMBER_CARDS_DECK {
        return Err(format!(
            "Exactly {} cards must be provided in total.",
            NUMBER_CARDS_DECK
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::types::{Card, Player, Rank, Suit, Variant};

    use super::*;

    fn full_deck() -> Vec<Card> {
        let ranks = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];
        let suits = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];

        let mut deck = Vec::new();
        for suit in suits.iter() {
            for rank in ranks.iter() {
                deck.push(Card {
                    rank: rank.clone(),
                    suit: suit.clone(),
                });
            }
        }
        deck
    }

    #[test]
    fn test_invalid_number_cards_five_card_draw() {
        let mut deck = full_deck();
        let player_1_cards = deck.drain(0..5).collect();
        let discarded_cards = deck.drain(0..1).collect();
        deck.drain(0..deck.len() - 1);

        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            players: vec![Player {
                id: String::from("player-1-id"),
                cards: player_1_cards,
            }],
            board: None,
            burn_cards: None,
            discarded_cards: Some(discarded_cards),
            remaining_deck: deck,
        };

        let result = validate_number_total_cards(&hand).unwrap_err();
        assert_eq!(result, "Exactly 52 cards must be provided in total.",);
    }

    #[test]
    fn test_valid_number_cards_five_card_draw() {
        let mut deck = full_deck();
        let player_1_cards = deck.drain(0..5).collect();
        let discarded_cards = deck.drain(0..13).collect();

        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FiveCardDraw,
            players: vec![Player {
                id: String::from("player-1-id"),
                cards: player_1_cards,
            }],
            board: None,
            burn_cards: None,
            discarded_cards: Some(discarded_cards),
            remaining_deck: deck,
        };

        let result = validate_number_total_cards(&hand);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_number_cards_texas_holdem() {
        let mut deck = full_deck();
        let player_1_cards = deck.drain(0..2).collect();
        let player_2_cards = deck.drain(0..2).collect();
        let board = deck.drain(0..5).collect();
        let burn_cards = deck.drain(0..3).collect();
        deck.pop();

        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: player_1_cards,
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: player_2_cards,
                },
            ],
            board: Some(board),
            burn_cards: Some(burn_cards),
            discarded_cards: None,
            remaining_deck: deck,
        };

        let result = validate_number_total_cards(&hand).unwrap_err();
        assert_eq!(result, "Exactly 52 cards must be provided in total.",);
    }

    #[test]
    fn test_valid_number_cards_texas_holdem() {
        let mut deck = full_deck();
        let player_1_cards = deck.drain(0..2).collect();
        let player_2_cards = deck.drain(0..2).collect();
        let board = deck.drain(0..5).collect();
        let burn_cards = deck.drain(0..3).collect();

        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::TexasHoldem,
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: player_1_cards,
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: player_2_cards,
                },
            ],
            board: Some(board),
            burn_cards: Some(burn_cards),
            discarded_cards: None,
            remaining_deck: deck,
        };

        let result = validate_number_total_cards(&hand);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_number_cards_four_card_omaha_hi() {
        let mut deck = full_deck();
        let player_1_cards = deck.drain(0..4).collect();
        let player_2_cards = deck.drain(0..4).collect();
        let board = deck.drain(0..5).collect();
        let burn_cards = deck.drain(0..3).collect();
        deck.pop();

        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FourCardOmahaHi,
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: player_1_cards,
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: player_2_cards,
                },
            ],
            board: Some(board),
            burn_cards: Some(burn_cards),
            discarded_cards: None,
            remaining_deck: deck,
        };

        let result = validate_number_total_cards(&hand).unwrap_err();
        assert_eq!(result, "Exactly 52 cards must be provided in total.",);
    }

    #[test]
    fn test_valid_number_cards_four_card_omaha_hi() {
        let mut deck = full_deck();
        let player_1_cards = deck.drain(0..4).collect();
        let player_2_cards = deck.drain(0..4).collect();
        let board = deck.drain(0..5).collect();
        let burn_cards = deck.drain(0..3).collect();

        let hand = Hand {
            id: String::from("hand-id"),
            variant: Variant::FourCardOmahaHi,
            players: vec![
                Player {
                    id: String::from("player-1-id"),
                    cards: player_1_cards,
                },
                Player {
                    id: String::from("player-2-id"),
                    cards: player_2_cards,
                },
            ],
            board: Some(board),
            burn_cards: Some(burn_cards),
            discarded_cards: None,
            remaining_deck: deck,
        };

        let result = validate_number_total_cards(&hand);
        assert!(result.is_ok());
    }
}
