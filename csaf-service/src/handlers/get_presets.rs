use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use csaf::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework as Csaf2_1;
use csaf::{schema::csaf2_0::schema::CommonSecurityAdvisoryFramework as Csaf2_0, validation::Validatable};
use serde::Serialize;
use utoipa::ToSchema;

use crate::models::*;

#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct PresetsResponse {
    pub version: String,
    pub presets: Vec<String>,
}

/// List available presets for a CSAF version.
#[utoipa::path(
    get,
    path = "/api/v1/csaf/{version}/presets",
    params(
        ("version" = String, Path, description = "CSAF version (2.0 or 2.1)")
    ),
    responses(
        (status = 200, description = "List of available presets", body = PresetsResponse),
        (status = 400, description = "Invalid version", body = ErrorResponse)
    ),
    tag = "presets"
)]
pub(crate) async fn list_presets(Path(version): Path<String>) -> impl IntoResponse {
    match presets_for_version(&version) {
        Some(presets) => Ok(Json(PresetsResponse {
            version,
            presets: presets.iter().map(|s| s.to_string()).collect(),
        })),
        None => Err(error_response(
            StatusCode::BAD_REQUEST,
            format!("Invalid CSAF version: {version}. Supported versions: 2.0, 2.1"),
        )),
    }
}

/// Returns the list of known preset names for a CSAF version.
pub(crate) fn presets_for_version(version: &str) -> Option<Vec<String>> {
    match version {
        "2.0" => Some(Csaf2_0::get_presets().iter().map(|p| p.to_string()).collect()),
        "2.1" => Some(Csaf2_1::get_presets().iter().map(|p| p.to_string()).collect()),
        _ => None,
    }
}
