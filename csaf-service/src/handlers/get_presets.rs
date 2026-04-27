use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use csaf::csaf_traits::CsafVersion;
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
pub(crate) async fn get_presets(Path(version): Path<String>) -> impl IntoResponse {
    match CsafVersion::try_from(version.clone()) {
        Err(err) => return Err(error_response(StatusCode::BAD_REQUEST, err)),
        Ok(valid_version) => Ok(Json(PresetsResponse {
            version,
            presets: presets_for_version(valid_version),
        })),
    }
}

/// Returns the list of known preset names for a CSAF version.
pub(crate) fn presets_for_version(version: CsafVersion) -> Vec<String> {
    match version {
        CsafVersion::X20 => Csaf2_0::get_presets().iter().map(|p| p.to_string()).collect(),
        CsafVersion::X21 => Csaf2_1::get_presets().iter().map(|p| p.to_string()).collect(),
    }
}
