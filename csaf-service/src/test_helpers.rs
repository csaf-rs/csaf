#![cfg(test)]

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::StatusCode;
use axum::routing::{get, post};
use http::Request;
use tower::ServiceExt;

use crate::handlers::health::health;
use crate::handlers::v1::{get_tests::get_tests, validate::validate};
use crate::routes;

/// Creates a router with all endpoints registered for testing.
pub fn app() -> Router {
    Router::new()
        .route(routes::HEALTH, get(health))
        .route(routes::V1_TESTS, get(get_tests))
        .route(routes::V1_VALIDATE, post(validate))
}

/// Sends a GET request to the given URI and returns the status and parsed JSON.
pub async fn get_json(uri: &str) -> (StatusCode, serde_json::Value) {
    let response = app()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    (status, json)
}

/// Sends a POST request with a JSON body and returns the status and parsed JSON.
pub async fn post_json(uri: &str, body: serde_json::Value) -> (StatusCode, serde_json::Value) {
    let response = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    (status, json)
}
