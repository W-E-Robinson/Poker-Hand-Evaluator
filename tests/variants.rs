use reqwest;
use serde_json::{json, Value};
use tokio;

#[tokio::test]
async fn test_variants_response() {
    let response = reqwest::get("http://localhost:8080/variants")
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_text = response.text().await.unwrap();
    // let response_json: Value = serde_json::from_str(&response_text).unwrap();

    let expected_json = json!({
         "message": "List of supported poker variants to evaluate.",
         "variants": [
             {
                 "path_parameter": "five-card-draw",
                 "display": {
                     "default": "Five-card draw",
                     "alternates": ["Cantredraw"]
                 }
             },
             {
                 "path_parameter": "texas-holdem",
                 "display": {
                     "default": "Texas hold'em",
                     "alternates": ["Texas holdem", "hold'em", "holdem"]
                 }
             }
         ]
    });

    assert_eq!(response_text, expected_json)
}
