pub mod five_card_draw;

use crate::PlayerRequest;

pub fn general_validate(players: &Vec<PlayerRequest>) -> Result<(), String> {
    if players.len() == 0 {
        return Err(String::from("must provide at least one player"));
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
        assert_eq!(error, "Must have at least one player");
    }
}
// fn test_errors_request_repeated_cards() {
// "players": [
// { "display": "player 1", "cards": ["2h", "3h", "4h", "5h", "6h"] },
// { "display": "player 2", "cards": ["2h", "3h", "4d", "5d", "6d"] }
// ]
// "message": "Duplicate cards: '2h', '3h'"

// async fn test_errors_request_repeated_player_display() {
// "players": [
// { "display": "player 1", "cards": ["2h", "3h", "4h", "5h", "6h"] },
// { "display": "player 1", "cards": ["2d", "3d", "4d", "5d", "6d"] }
// ]
// "message": "Each player must have a unique display, offending players: player 1"

// async fn test_errors_request_unrecognised_cards() {
// "players": [
//   { "display": "player 1", "cards": ["not a card 1", "3h", "4h", "5h", "6h"] },
// { "display": "player 2", "cards": ["not a card 2", "8h", "9h", "Th", "Jh"] }
// ]
// "message": "Invalid cards provided: 'not a card 1', 'not a card 2'"
