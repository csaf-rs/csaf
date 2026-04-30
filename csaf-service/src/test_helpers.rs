#![cfg(test)]

use axum::Router;
use axum::body::Body;
use axum::http::StatusCode;
use axum::routing::get;
use http::Request;
use http_body_util::BodyExt;
use tower::ServiceExt;

use crate::handlers::get_preset_tests::get_preset_tests;
use crate::handlers::get_presets::get_presets;
use crate::routes;

/// Creates a router with all GET endpoints registered for testing.
pub fn app() -> Router {
    Router::new()
        .route(routes::PRESETS, get(get_presets))
        .route(routes::PRESET_TESTS, get(get_preset_tests))
}

/// Sends a GET request to the given URI on the test app and returns the status and parsed JSON.
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
