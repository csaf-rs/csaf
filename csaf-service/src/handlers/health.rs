use axum::{Json, response::IntoResponse};

/// Health check endpoint.
#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy")
    ),
    tag = "health"
)]
pub(crate) async fn health() -> impl IntoResponse {
    Json(serde_json::json!({"status": "ok"}))
}
