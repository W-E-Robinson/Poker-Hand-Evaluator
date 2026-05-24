use axum::{
    extract::Query,
    routing::{get, post},
    Router,
};
use serde::Serialize;
use std::collections::HashMap;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/variants", get(get_variants))
        .route("/evaluate", post(post_evaluate))
        .layer(TraceLayer::new_for_http())
}

#[derive(Serialize)]
struct VariantDisplayInfo {
    default: String,
    alternates: Vec<String>,
}
#[derive(Serialize)]
struct VariantInfo {
    http_request: String,
    display: VariantDisplayInfo,
}
async fn get_variants() -> String {
    let variants_response = vec![VariantInfo {
        http_request: String::from("POST /evaluate?variant=five-card-draw"),
        display: VariantDisplayInfo {
            default: String::from("Five-card draw"),
            alternates: vec![String::from("Cantredraw")],
        },
    }];

    serde_json::to_string(&variants_response).unwrap()
}

async fn post_evaluate(Query(params): Query<HashMap<String, String>>) -> String {
    let variants_response = vec![VariantInfo {
        http_request: String::from("POST /evaluate?variant=five-card-draw"),
        display: VariantDisplayInfo {
            default: String::from("Five-card draw"),
            alternates: vec![String::from("Cantredraw")],
        },
    }];

    serde_json::to_string(&variants_response).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_not_found() {
        let app = app();

        let response = app
            .oneshot(Request::get("/does-not-exist").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn test_get_variants() {
        let app = app();

        let response = app
            .oneshot(Request::get("/variants").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        let expected_response = vec![VariantInfo {
            http_request: String::from("POST /evaluate?variant=five-card-draw"),
            display: VariantDisplayInfo {
                default: String::from("Five-card draw"),
                alternates: vec![String::from("Cantredraw")],
            },
        }];
        assert_eq!(body, serde_json::to_value(&expected_response).unwrap());
    }

    #[tokio::test]
    async fn test_evaluation_error() {
        // NOTE: test above unsupported variant
        let app = app();

        let response = app
            .oneshot(
                Request::get("/evaluate?variant=five-card-draw")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        let expected_response = vec![VariantInfo {
            http_request: String::from("POST /evaluate?variant=five-card-draw"),
            display: VariantDisplayInfo {
                default: String::from("Five-card draw"),
                alternates: vec![String::from("Cantredraw")],
            },
        }];
        assert_eq!(body, serde_json::to_value(&expected_response).unwrap());
    }

    // one server test

    #[tokio::test]
    async fn json() {
        let app = app();

        let response = app
            .oneshot(
                Request::post("/json")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!([1, 2, 3, 4])).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "data": [1, 2, 3, 4] }));
    }
}
