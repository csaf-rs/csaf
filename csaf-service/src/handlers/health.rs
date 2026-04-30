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

#[cfg(test)]
mod tests {
    use crate::routes;
    use crate::test_helpers::get_json;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn returns_ok_status() {
        let (status, json) = get_json(routes::HEALTH).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["status"], "ok");
    }
}
