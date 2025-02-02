pub mod five_card_draw;
use std::collections::HashSet;

use crate::{Card, PlayerRequest};

pub fn general_validate(players: &Vec<PlayerRequest>) -> Result<(), String> {
    if players.len() == 0 {
        return Err(String::from("must provide at least one player"));
    }

    let mut cards: HashSet<String> = HashSet::new();
    let mut displays: HashSet<String> = HashSet::new();

    for player in players.iter() {
        if displays.get(&player.display) == None {
            displays.insert(player.display.clone());
        } else {
            return Err(String::from(format!(
                "repeated player display: {}",
                player.display,
            )));
        }

        for card in player.cards.iter() {
            if let Err(_err) = Card::from_str(card) {
                return Err(String::from(format!("invalid card: {}", card)));
            }

            if cards.get(card) == None {
                cards.insert(card.to_owned());
            } else {
                return Err(String::from(format!("repeated card: {}", card)));
            }
        }
    }

    Ok(())
}

pub fn too_many_players(
    players: &Vec<PlayerRequest>,
    cards_per_player: usize,
    board_cards: usize,
    burn_cards: usize,
) -> Result<(), String> {
    let max_players = (52 - board_cards - burn_cards) / cards_per_player;
    if players.len() > max_players {
        return Err(String::from(format!(
            "maximum number of players is {}, {} provided",
            max_players,
            players.len()
        )));
    }

    Ok(())
}

pub fn wrong_number_cards(
    players: &Vec<PlayerRequest>,
    cards_per_player: usize,
) -> Result<(), String> {
    for player in players.iter() {
        if player.cards.len() != cards_per_player {
            return Err(String::from(format!(
                "expected number of cards per player is {}, {} provided for {}",
                cards_per_player,
                player.cards.len(),
                player.display,
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_errors_no_players() {
        let players_requests = vec![];
        let error = general_validate(&players_requests).unwrap_err();
        assert_eq!(error, "must provide at least one player");
    }

    #[test]
    fn test_errors_repeated_card() {
        let players_requests = vec![
            PlayerRequest {
                display: "player 1".to_owned(),
                cards: vec![
                    "2h".to_owned(),
                    "3h".to_owned(),
                    "4h".to_owned(),
                    "5h".to_owned(),
                    "6h".to_owned(),
                ],
            },
            PlayerRequest {
                display: "player 2".to_owned(),
                cards: vec![
                    "2h".to_owned(),
                    "3d".to_owned(),
                    "4d".to_owned(),
                    "5d".to_owned(),
                    "6d".to_owned(),
                ],
            },
        ];
        let error = general_validate(&players_requests).unwrap_err();
        assert_eq!(error, "repeated card: 2h");
    }

    #[test]
    fn test_errors_repeated_player_display() {
        let players_requests = vec![
            PlayerRequest {
                display: "player 1".to_owned(),
                cards: vec![
                    "2h".to_owned(),
                    "3h".to_owned(),
                    "4h".to_owned(),
                    "5h".to_owned(),
                    "6h".to_owned(),
                ],
            },
            PlayerRequest {
                display: "player 1".to_owned(),
                cards: vec![
                    "2d".to_owned(),
                    "3d".to_owned(),
                    "4d".to_owned(),
                    "5d".to_owned(),
                    "6d".to_owned(),
                ],
            },
        ];
        let error = general_validate(&players_requests).unwrap_err();
        assert_eq!(error, "repeated player display: player 1");
    }

    #[test]
    fn test_errors_unrecognised_card() {
        let players_requests = vec![
            PlayerRequest {
                display: "player 1".to_owned(),
                cards: vec![
                    "not a card".to_owned(),
                    "3h".to_owned(),
                    "4h".to_owned(),
                    "5h".to_owned(),
                    "6h".to_owned(),
                ],
            },
            PlayerRequest {
                display: "player 2".to_owned(),
                cards: vec![
                    "2d".to_owned(),
                    "3d".to_owned(),
                    "4d".to_owned(),
                    "5d".to_owned(),
                    "6d".to_owned(),
                ],
            },
        ];
        let error = general_validate(&players_requests).unwrap_err();
        assert_eq!(error, "invalid card: not a card");
    }

    #[test]
    fn test_errors_too_many_players() {
        let players_requests = vec![
            PlayerRequest {
                display: "player 1".to_owned(),
                cards: vec![
                    "2h".to_owned(),
                    "3h".to_owned(),
                    "4h".to_owned(),
                    "5h".to_owned(),
                    "6h".to_owned(),
                ],
            },
            PlayerRequest {
                display: "player 2".to_owned(),
                cards: vec![
                    "2d".to_owned(),
                    "3d".to_owned(),
                    "4d".to_owned(),
                    "5d".to_owned(),
                    "6d".to_owned(),
                ],
            },
        ];
        let error = too_many_players(&players_requests, 25, 5, 5).unwrap_err();
        assert_eq!(error, "maximum number of players is 1, 2 provided");
    }

    #[test]
    fn test_errors_wrong_number_cards() {
        let players_requests = vec![
            PlayerRequest {
                display: "player 1".to_owned(),
                cards: vec![
                    "2h".to_owned(),
                    "3h".to_owned(),
                    "4h".to_owned(),
                    "5h".to_owned(),
                ],
            }
        ];
        let error = wrong_number_cards(&players_requests, 5).unwrap_err();
        assert_eq!(error, "expected number of cards per player is 5, 4 provided for player 1");
    }

    #[test]
    fn test_ok() {
        let players_requests = vec![
            PlayerRequest {
                display: "player 1".to_owned(),
                cards: vec![
                    "2h".to_owned(),
                    "3h".to_owned(),
                    "4h".to_owned(),
                    "5h".to_owned(),
                    "6h".to_owned(),
                ],
            },
            PlayerRequest {
                display: "player 2".to_owned(),
                cards: vec![
                    "2d".to_owned(),
                    "3d".to_owned(),
                    "4d".to_owned(),
                    "5d".to_owned(),
                    "6d".to_owned(),
                ],
            },
        ];
        assert_eq!(general_validate(&players_requests), Ok(()));
    }
}
