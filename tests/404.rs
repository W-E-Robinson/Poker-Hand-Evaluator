use reqwest;
use serde_json::{json, Value};
use tokio;

#[tokio::test]
async fn test_404_reponse() {
    let response = reqwest::get("http://localhost:8080/not-a-path")
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
        "error": "Endpoint call was not found.",
        "code": 404,
        "display": "NOT FOUND"
    });

    assert_eq!(response_text, expected_json)
}
