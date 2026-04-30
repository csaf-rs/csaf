use axum::Json;
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use csaf::csaf_traits::CsafVersion;
use serde::Serialize;
use utoipa::ToSchema;

use crate::handlers::get_presets::presets_for_version;
use crate::models::{ErrorResponse, error_response};

#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct PresetTestsResponse {
    pub preset: String,
    pub version: String,
    pub tests: Vec<String>,
}

/// Get the test IDs belonging to a preset.
#[utoipa::path(
    get,
    path = "/api/v1/csaf/{version}/presets/{preset}/tests",
    params(
        ("version" = String, Path, description = "CSAF version (2.0 or 2.1)"),
        ("preset" = String, Path, description = "Preset name")
    ),
    responses(
        (status = 200, description = "List of test IDs in the preset", body = PresetTestsResponse),
        (status = 400, description = "Invalid version or preset", body = ErrorResponse)
    ),
    tag = "presets"
)]
pub(crate) async fn get_preset_tests(Path((version, preset)): Path<(String, String)>) -> impl IntoResponse {
    match CsafVersion::try_from(version.clone()) {
        Err(err) => Err(error_response(StatusCode::BAD_REQUEST, err)),
        Ok(valid_version) => {
            let presets = presets_for_version(&valid_version);
            if !presets.contains(&preset) {
                return Err(error_response(
                    StatusCode::BAD_REQUEST,
                    format!("Preset '{preset}' not found for version {version}"),
                ));
            }
            Ok(Json(PresetTestsResponse {
                preset,
                version,
                tests: presets_for_version(&valid_version), // ToDo
            }))
        },
    }
}
