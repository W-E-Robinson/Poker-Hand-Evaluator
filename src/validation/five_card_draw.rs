use crate::PlayerRequest;

use super::{too_many_players, wrong_number_cards};

pub fn validate(players: &Vec<PlayerRequest>) -> Result<(), String> {
    too_many_players(players, 5, 0, 0)?;

    wrong_number_cards(players, 5)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_errors_too_many_players() {
        let players_requests = vec![
            PlayerRequest {
                display: "player 1".to_owned(),
                cards: vec![
                    "3h".to_owned(),
                    "7s".to_owned(),
                    "Ts".to_owned(),
                    "4h".to_owned(),
                    "Kd".to_owned(),
                ],
            },
            PlayerRequest {
                display: "player 2".to_owned(),
                cards: vec![
                    "5c".to_owned(),
                    "Jd".to_owned(),
                    "6h".to_owned(),
                    "Qs".to_owned(),
                    "3s".to_owned(),
                ],
            },
            PlayerRequest {
                display: "player 3".to_owned(),
                cards: vec![
                    "9d".to_owned(),
                    "As".to_owned(),
                    "2h".to_owned(),
                    "5h".to_owned(),
                    "Jh".to_owned(),
                ],
            },
            PlayerRequest {
                display: "player 4".to_owned(),
                cards: vec![
                    "Kh".to_owned(),
                    "Td".to_owned(),
                    "Qd".to_owned(),
                    "Ah".to_owned(),
                    "8h".to_owned(),
                ],
            },
            PlayerRequest {
                display: "player 5".to_owned(),
                cards: vec![
                    "6s".to_owned(),
                    "7h".to_owned(),
                    "4s".to_owned(),
                    "2s".to_owned(),
                    "9c".to_owned(),
                ],
            },
            PlayerRequest {
                display: "player 6".to_owned(),
                cards: vec![
                    "3d".to_owned(),
                    "Jc".to_owned(),
                    "9h".to_owned(),
                    "8c".to_owned(),
                    "Kc".to_owned(),
                ],
            },
            PlayerRequest {
                display: "player 7".to_owned(),
                cards: vec![
                    "7d".to_owned(),
                    "5d".to_owned(),
                    "Tc".to_owned(),
                    "6c".to_owned(),
                    "Ad".to_owned(),
                ],
            },
            PlayerRequest {
                display: "player 8".to_owned(),
                cards: vec![
                    "8d".to_owned(),
                    "Qc".to_owned(),
                    "4d".to_owned(),
                    "2d".to_owned(),
                    "7c".to_owned(),
                ],
            },
            PlayerRequest {
                display: "player 9".to_owned(),
                cards: vec![
                    "Js".to_owned(),
                    "Ks".to_owned(),
                    "9s".to_owned(),
                    "4c".to_owned(),
                    "Th".to_owned(),
                ],
            },
            PlayerRequest {
                display: "player 10".to_owned(),
                cards: vec![
                    "5s".to_owned(),
                    "2c".to_owned(),
                    "Qh".to_owned(),
                    "6d".to_owned(),
                    "Ac".to_owned(),
                ],
            },
            PlayerRequest {
                display: "player 11".to_owned(),
                cards: vec![
                    "3c".to_owned(),
                    "Jc".to_owned(),
                    "9h".to_owned(),
                    "8c".to_owned(),
                    "Kc".to_owned(),
                ],
            },
        ];
        let error = validate(&players_requests).unwrap_err();
        assert_eq!(error, "maximum number of players is 10, 11 provided");
    }

    #[test]
    fn test_errors_wrong_number_cards() {
        let players_requests = vec![PlayerRequest {
            display: "player 1".to_owned(),
            cards: vec![
                "2h".to_owned(),
                "3h".to_owned(),
                "4h".to_owned(),
                "5h".to_owned(),
            ],
        }];
        let error = validate(&players_requests).unwrap_err();
        assert_eq!(
            error,
            "expected number of cards per player is 5, 4 provided for player 1"
        );
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
        assert_eq!(validate(&players_requests), Ok(()));
    }
}
