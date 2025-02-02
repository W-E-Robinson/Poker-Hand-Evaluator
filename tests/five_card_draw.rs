use reqwest::Client;
use serde_json::{json, Value};
use tokio;

// NOTE: not enough players = 0 = actually should be in general

#[tokio::test]
async fn test_bad_request_too_many_players() {
    let body = json!({
      "players": [
        { "display": "player 1", "cards": ["3h", "7s", "Ts", "4h", "Kd"] },
        { "display": "player 2", "cards": ["5c", "Jd", "6h", "Qs", "3s"] },
        { "display": "player 3", "cards": ["9d", "As", "2h", "5h", "Jh"] },
        { "display": "player 4", "cards": ["Kh", "Td", "Qd", "Ah", "8h"] },
        { "display": "player 5", "cards": ["6s", "7h", "4s", "2s", "9c"] },
        { "display": "player 6", "cards": ["3d", "Jc", "9h", "8c", "Kc"] },
        { "display": "player 7", "cards": ["7d", "5d", "Tc", "6c", "Ad"] },
        { "display": "player 8", "cards": ["8d", "Qc", "4d", "2d", "7c"] },
        { "display": "player 9", "cards": ["Js", "Ks", "9s", "4c", "Th"] },
        { "display": "player 10", "cards": ["5s", "2c", "Qh", "6d", "Ac"] },
        { "display": "player 11", "cards": ["3c", "Jc", "9h", "8c", "Kc"] }
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
            "message": "Maximum number of players in a Five-card draw hand is 10, 11 are provided"
        }
    });

    assert_eq!(response_text, expected_json)
}

#[tokio::test]
async fn test_bad_request_wrong_number_cards_player_hand() {
    let body = json!({
      "players": [
        { "display": "player 1", "cards": ["2h", "3h", "4h", "5h", "6h"] },
        { "display": "player 2", "cards": ["2h", "3h", "4h", "5h", "6h", "7h"] },
        { "display": "player 3", "cards": ["2h", "3h", "4h", "5h"] }
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
             "message": "Players 2 and 3 provided the incorrect number of cards, each player provides 5 cards"
         }
    });

    assert_eq!(response_text, expected_json)
}

#[tokio::test]
async fn test_evaluation_flush_chopped_pot() {
    let body = json!({
        "players": [
            { "display": "player 1", "cards": [ "Ac", "3c", "4c", "5c", "6c" ] },
            { "display": "player 2", "cards": [ "Ad", "3d", "4d", "5d", "6d" ] },
            { "display": "player 3", "cards": [ "9c", "9d", "9s", "8h", "7h" ] }
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

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "players": [
            {
                "display": "player 1",
                "cards": [ "Ac", "3c", "4c", "5c", "6c" ],
                "hand": "Ace, six, five, four, three flush",
                "winner": true
            },
            {
                "display": "player 2",
                "cards": [ "Ad", "3d", "4d", "5d", "6d" ],
                "hand": "Ace, six, five, four, three flush",
                "winner": true
            },
            {
                "display": "player 3",
                "cards": [ "9c", "9d", "9s", "8h", "7h" ],
                "hand": "Three of a kind nines with eight and seven kickers",
                "winner": false
            }
        ]
    });

    assert_eq!(response_text, expected_json)
}

#[tokio::test]
async fn test_evaluation_flush_over_straight() {
    let body = json!({
        "players": [
            { "display": "player 1", "cards": [ "Ah", "Kd", "Th", "Jh", "Qh" ] },
            { "display": "player 2", "cards": [ "Ad", "3d", "4d", "5d", "6d" ] },
            { "display": "player 3", "cards": [ "9c", "9d", "9s", "8h", "7h" ] }
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

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "players": [
            {
                "display": "player 1",
                "cards": [ "Ah", "Kh", "Th", "Jh", "Qh" ],
                "hand": "Ace high straight",
                "winner": false
            },
            {
                "display": "player 2",
                "cards": [ "Ad", "3d", "4d", "5d", "6d" ],
                "hand": "Ace, six, five, four, three flush",
                "winner": true
            },
            {
                "display": "player 3",
                "cards": [ "9c", "9d", "9s", "8h", "7h" ],
                "hand": "Three of a kind nines with eight and seven kickers",
                "winner": false
            }
        ]
    });

    assert_eq!(response_text, expected_json)
}

#[tokio::test]
async fn test_evaluation_flush_over_three_of_a_kind() {
    let body = json!({
        "players": [
            { "display": "player 1", "cards": [ "2h", "Kh", "Th", "Jh", "Qh" ] },
            { "display": "player 2", "cards": [ "2d", "2d", "4d", "Kd", "6d" ] },
            { "display": "player 3", "cards": [ "9c", "9d", "9s", "8h", "7h" ] }
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

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "players": [
            {
                "display": "player 1",
                "cards": [ "2h", "Kh", "Th", "Jh", "Qh" ],
                "hand": "King high flush with queen, jack, ten and two kickers",
                "winner": true
            },
            {
                "display": "player 2",
                "cards": [ "2d", "2d", "4d", "Kd", "6d" ],
                "hand": "Pair of twos with king, six and four kickers",
                "winner": false
            },
            {
                "display": "player 3",
                "cards": [ "9c", "9d", "9s", "8h", "7h" ],
                "hand": "Three of a kind nines with eight and seven kickers",
                "winner": false
            }
        ]
    });

    assert_eq!(response_text, expected_json)
}

#[tokio::test]
async fn test_evaluation_four_of_a_kind_winner() {
    let body = json!({
        "players": [
            { "display": "player 1", "cards": [ "Ah", "Kh", "Td", "Jh", "Qh" ] },
            { "display": "player 2", "cards": [ "2d", "3d", "4h", "5d", "6d" ] },
            { "display": "player 3", "cards": [ "9c", "9d", "9s", "9h", "7h" ] }
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

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "players": [
            {
                "display": "player 1",
                "cards": [ "Ah", "Kh", "Td", "Jh", "Qh" ],
                "hand": "Ace high straight",
                "winner": false
            },
            {
                "display": "player 2",
                "cards": [ "2d", "3d", "4h", "5d", "6d" ],
                "hand": "Six high straight",
                "winner": false
            },
            {
                "display": "player 3",
                "cards": [ "9c", "9d", "9s", "9h", "7h" ],
                "hand": "Four of a kind nines with seven kicker",
                "winner": true
            }
        ]
    });

    assert_eq!(response_text, expected_json)
}

#[tokio::test]
async fn test_evaluation_full_house_winner() {
    let body = json!({
        "players": [
            { "display": "player 1", "cards": [ "Ah", "Kh", "Qc", "Qd", "Qh" ] },
            { "display": "player 2", "cards": [ "2d", "3d", "4d", "5d", "9d" ] },
            { "display": "player 3", "cards": [ "9c", "9d", "9s", "8h", "8d" ] }
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

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "players": [
            {
                "display": "player 1",
                "cards": [ "Ah", "Kh", "Qc", "Qd", "Qh" ],
                "hand": "Three of a kind queens with ace and king kickers",
                "winner": false
            },
            {
                "display": "player 2",
                "cards": [ "2d", "3d", "4d", "5d", "9d" ],
                "hand": "Nine, five, four, three, two flush",
                "winner": false
            },
            {
                "display": "player 3",
                "cards": [ "9c", "9d", "9s", "8h", "8d" ],
                "hand": "Full house, nines full of eights",
                "winner": true
            }
        ]
    });

    assert_eq!(response_text, expected_json)
}

#[tokio::test]
async fn test_evaluation_high_card_winner() {
    let body = json!({
        "players": [
            { "display": "player 1", "cards": [ "Ah", "Kh", "9c", "Jh", "Qh" ] },
            { "display": "player 2", "cards": [ "2d", "3d", "4d", "5d", "7h" ] },
            { "display": "player 3", "cards": [ "9c", "3d", "4s", "8h", "7h" ] }
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

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "players": [
            {
                "display": "player 1",
                "cards": [ "Ah", "Kh", "9c", "Jh", "Qh" ],
                "hand": "High card ace with king, queen, jack and nine kickers",
                "winner": true
            },
            {
                "display": "player 2",
                "cards": [ "2d", "3d", "4d", "5d", "7h" ],
                "hand": "High card seven with five, four, three and two kickers",
                "winner": false
            },
            {
                "display": "player 3",
                "cards": [ "9c", "3d", "4s", "8h", "7h" ],
                "hand": "High card nine with eight, seven, four and three kickers",
                "winner": false
            }
        ]
    });

    assert_eq!(response_text, expected_json)
}

#[tokio::test]
async fn test_evaluation_higher_pair() {
    let body = json!({
        "players": [
            { "display": "player 1", "cards": [ "Kd", "Kh", "Th", "Jh", "Qh" ] },
            { "display": "player 2", "cards": [ "3c", "3d", "4d", "5d", "6d" ] },
            { "display": "player 3", "cards": [ "Tc", "4d", "9s", "8h", "7h" ] }
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

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "players": [
            {
                "display": "player 1",
                "cards": [ "Kd", "Kh", "Th", "Jh", "Qh" ],
                "hand": "Pair of kings with queen, jack and ten kickers",
                "winner": true
            },
            {
                "display": "player 2",
                "cards": [ "3c", "3d", "4d", "5d", "6d" ],
                "hand": "Pair of threes with six, five and four kickers",
                "winner": false
            },
            {
                "display": "player 3",
                "cards": [ "Tc", "4d", "9s", "8h", "7h" ],
                "hand": "High card ten with nine, eight, seven and four kickers",
                "winner": false
            }
        ]
    });

    assert_eq!(response_text, expected_json)
}

#[tokio::test]
async fn test_evaluation_same_pair_higher_second_kicker() {
    let body = json!({
        "players": [
            {
                "display": "player 1",
                "cards": [ "Jh", "Kd", "Th", "Jh", "Qh" ]
            },
            {
                "display": "player 2",
                "cards": [ "Jd", "Jc", "4d", "5d", "6d" ]
            },
            {
                "display": "player 3",
                "cards": [ "9c", "2d", "9s", "8h", "7h" ]
            }
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

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "players": [
            {
                "display": "player 1",
                "cards": [ "Jh", "Kd", "Th", "Jh", "Qh" ],
                "hand": "Pair of jacks with king, queen and ten kickers",
                "winner": true
            },
            {
                "display": "player 2",
                "cards": [ "Jd", "Jc", "4d", "5d", "6d" ],
                "hand": "Pair of jacks with six, five and four kickers",
                "winner": false
            },
            {
                "display": "player 3",
                "cards": [ "9c", "2d", "9s", "8h", "7h" ],
                "hand": "Pair of nines with eight, seven and two kickers",
                "winner": false
            }
        ]
    });

    assert_eq!(response_text, expected_json)
}

#[tokio::test]
async fn test_evaluation_single_player() {
    let body = json!({
        "players": [
            { "display": "player 1", "cards": [ "Ah", "Kh", "Th", "Jh", "Qh" ] }
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

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "players": [
            {
                "display": "player 1",
                "cards": [ "Ah", "Kh", "Th", "Jh", "Qh" ],
                "hand": "Ace high straight flush",
                "winner": true
            }
        ]
    });

    assert_eq!(response_text, expected_json)
}

#[tokio::test]
async fn test_evaluation_straight_flush() {
    let body = json!({
        "players": [
            { "display": "player 1", "cards": [ "Ah", "Kh", "Th", "Jh", "Qh" ] },
            { "display": "player 2", "cards": [ "2c", "3d", "4d", "5d", "6d" ] },
            { "display": "player 3", "cards": [ "9c", "9d", "9s", "8h", "7h" ] }
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

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "players": [
            {
                "display": "player 1",
                "cards": [ "Ah", "Kh", "Th", "Jh", "Qh" ],
                "hand": "Ace high straight flush",
                "winner": true
            },
            {
                "display": "player 2",
                "cards": [ "2c", "3d", "4d", "5d", "6d" ],
                "hand": "Six high straight",
                "winner": false
            },
            {
                "display": "player 3",
                "cards": [ "9c", "9d", "9s", "8h", "7h" ],
                "hand": "Three of a kind nines with eight and seven kickers",
                "winner": false
            }
        ]
    });

    assert_eq!(response_text, expected_json)
}

#[tokio::test]
async fn test_evaluation_straight_over_three_of_a_kind() {
    let body = json!({
        "players": [
            { "display": "player 1", "cards": [ "Ad", "Kh", "Th", "Jh", "Qh" ] },
            { "display": "player 2", "cards": [ "Ac", "3d", "4d", "5d", "6d" ] },
            { "display": "player 3", "cards": [ "9c", "9d", "9s", "8h", "7h" ] }
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

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "players": [
            {
                "display": "player 1",
                "cards": [ "Ad", "Kh", "Th", "Jh", "Qh" ],
                "hand": "Ace high straight",
                "winner": true
            },
            {
                "display": "player 2",
                "cards": [ "Ac", "3d", "4d", "5d", "6d" ],
                "hand": "High card Ace with six, five, four and three kickers",
                "winner": false
            },
            {
                "display": "player 3",
                "cards": [ "9c", "9d", "9s", "8h", "7h" ],
                "hand": "Three of a kind nines with eight and seven kickers",
                "winner": false
            }
        ]
    });

    assert_eq!(response_text, expected_json)
}

#[tokio::test]
async fn test_evaluation_three_of_a_kind_over_pair() {
    let body = json!({
        "players": [
            { "display": "player 1", "cards": [ "Kd", "Kh", "Th", "Jh", "Qh" ] },
            { "display": "player 2", "cards": [ "Ac", "3d", "4d", "5d", "6d" ] },
            { "display": "player 3", "cards": [ "9c", "9d", "9s", "8h", "7h" ] }
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

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "players": [
            {
                "display": "player 1",
                "cards": [ "Kd", "Kh", "Th", "Jh", "Qh" ],
                "hand": "Pair of kings with queen, jack and ten kickers",
                "winner": false
            },
            {
                "display": "player 2",
                "cards": [ "Ac", "3d", "4d", "5d", "6d" ],
                "hand": "High card ace with six, five, four and three kickers",
                "winner": false
            },
            {
                "display": "player 3",
                "cards": [ "9c", "9d", "9s", "8h", "7h" ],
                "hand": "Three of a kind nines with eight and seven kickers",
                "winner": true
            }
        ]
    });

    assert_eq!(response_text, expected_json)
}

#[tokio::test]
async fn test_evaluation_wheel_flush() {
    let body = json!({
        "players": [
            { "display": "player 1", "cards": [ "Ah", "Kh", "Th", "Jh", "Kd" ] },
            { "display": "player 2", "cards": [ "2d", "3d", "4d", "5d", "Ad" ] },
            { "display": "player 3", "cards": [ "9c", "9d", "9s", "8h", "7h" ] }
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

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "players": [
            {
                "display": "player 1",
                "cards": [ "Ah", "Kh", "Th", "Jh", "Kd" ],
                "hand": "Pair of kings with ace, jack and ten kickers",
                "winner": false
            },
            {
                "display": "player 2",
                "cards": [ "2d", "3d", "4d", "5d", "Ad" ],
                "hand": "Wheel straight flush",
                "winner": true
            },
            {
                "display": "player 3",
                "cards": [ "9c", "9d", "9s", "8h", "7h" ],
                "hand": "Three of a kind nines with eight and seven kickers",
                "winner": false
            }
        ]
    });

    assert_eq!(response_text, expected_json)
}

#[tokio::test]
async fn test_evaluation_wheel_winner() {
    let body = json!({
        "players": [
            { "display": "player 1", "cards": [ "Ah", "Kh", "Th", "Jh", "Kd" ] },
            { "display": "player 2", "cards": [ "2d", "3d", "4d", "5d", "Ac" ] },
            { "display": "player 3", "cards": [ "9c", "9d", "9s", "8h", "7h" ] }
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

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "players": [
            {
                "display": "player 1",
                "cards": [ "Ah", "Kh", "Th", "Jh", "Kd" ],
                "hand": "Pair of kings with ace, jack and ten kickers",
                "winner": false
            },
            {
                "display": "player 2",
                "cards": [ "2d", "3d", "4d", "5d", "Ac" ],
                "hand": "Wheel straight",
                "winner": true
            },
            {
                "display": "player 3",
                "cards": [ "9c", "9d", "9s", "8h", "7h" ],
                "hand": "Three of a kind nines with eight and seven kickers",
                "winner": false
            }
        ]
    });

    assert_eq!(response_text, expected_json)
}
