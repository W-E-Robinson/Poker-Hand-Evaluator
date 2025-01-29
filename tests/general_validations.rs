use reqwest::Client;
use serde_json::{json, Value};
use tokio;

// General validations tested against Five-card draw endpoint, but will be ubiquitous across all
// poker variants

#[tokio::test]
async fn test_bad_request_repeated_cards() {
    let body = json!({
        "players": [
            { "display": "player 1", "cards": ["2h", "3h", "4h", "5h", "6h"] },
            { "display": "player 2", "cards": ["2h", "3h", "4d", "5d", "6d"] }
        ]
    })
    .to_string();

    let client = Client::new();
    let response = client
        .post("http://localhost:8080/evaluate/five-card-draw")
        .body(body)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "error": {
            "code": "400",
            "type": "Bad request",
            "message": "Duplicate cards: '2h', '3h'"
        }
    });

    assert_eq!(response_text, expected_json)
}

#[tokio::test]
async fn test_bad_request_repeated_player_display() {
    let body = json!({
        "players": [
            { "display": "player 1", "cards": ["2h", "3h", "4h", "5h", "6h"] },
            { "display": "player 1", "cards": ["2d", "3d", "4d", "5d", "6d"] }
        ]
    })
    .to_string();

    let client = Client::new();
    let response = client
        .post("http://localhost:8080/evaluate/five-card-draw")
        .body(body)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "error": {
            "code": "400",
            "type": "Bad request",
            "message": "Each player must have a unique display, offending players: player 1"
        }
    });

    assert_eq!(response_text, expected_json)
}

#[tokio::test]
async fn test_bad_request_unrecognised_cards() {
    let body = json!({
        "players": [
            { "display": "player 1", "cards": ["not a card 1", "3h", "4h", "5h", "6h"] },
            { "display": "player 2", "cards": ["not a card 2", "8h", "9h", "Th", "Jh"] }
        ]
    })
    .to_string();

    let client = Client::new();
    let response = client
        .post("http://localhost:8080/evaluate/five-card-draw")
        .body(body)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "error": {
            "code": "400",
            "type": "Bad request",
            "message": "Invalid cards provided: 'not a card 1', 'not a card 2'"
        }
    });

    assert_eq!(response_text, expected_json)
}
