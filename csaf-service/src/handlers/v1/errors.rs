use axum::{Json, http::StatusCode};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    #[serde(rename = "statusCode")]
    pub status_code: u16,
    pub message: Option<String>,
    pub code: String,
}

pub(crate) fn error_response(
    status: StatusCode,
    code: impl Into<String>,
    message: impl Into<String>,
) -> (StatusCode, Json<ErrorResponse>) {
    (
        status,
        Json(ErrorResponse {
            status_code: status.as_u16(),
            error: status.to_string(),
            message: Some(message.into()),
            code: code.into(),
        }),
    )
}
