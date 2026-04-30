#![cfg(test)]

use axum::Router;
use axum::body::Body;
use axum::http::StatusCode;
use axum::routing::{get, post};
use http::Request;
use http_body_util::BodyExt;
use tower::ServiceExt;

use crate::handlers::get_preset_tests::get_preset_tests;
use crate::handlers::get_presets::get_presets;
use crate::handlers::health::health;
use crate::handlers::validate::{validate, validate_file};
use crate::routes;

/// Creates a router with all endpoints registered for testing.
pub fn app() -> Router {
    Router::new()
        .route(routes::PRESETS, get(get_presets))
        .route(routes::PRESET_TESTS, get(get_preset_tests))
        .route(routes::VALIDATE, post(validate))
        .route(routes::VALIDATE_FILE, post(validate_file))
        .route(routes::HEALTH, get(health))
}

/// Sends a GET request to the given URI and returns the status and parsed JSON.
pub async fn get_json(uri: &str) -> (StatusCode, serde_json::Value) {
    let response = app()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
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
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    (status, json)
}

/// Sends a POST request with raw bytes and returns the status and parsed JSON.
pub async fn post_bytes(uri: &str, bytes: Vec<u8>) -> (StatusCode, serde_json::Value) {
    let response = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("content-type", "application/octet-stream")
                .body(Body::from(bytes))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    (status, json)
}
