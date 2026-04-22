use axum::Json;
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use utoipa::ToSchema;

use csaf::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework as Csaf2_1;
use csaf::{schema::csaf2_0::schema::CommonSecurityAdvisoryFramework as Csaf2_0, validation::Validatable};

use crate::handlers::get_presets::presets_for_version;
use crate::models::{ErrorResponse, error_response};

#[derive(Debug, Serialize, ToSchema)]
pub struct PresetTestsResponse {
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
pub async fn get_preset_tests(Path((version, preset)): Path<(String, String)>) -> impl IntoResponse {
    if presets_for_version(&version).is_none() {
        return Err(error_response(
            StatusCode::BAD_REQUEST,
            format!("Invalid CSAF version: {version}. Supported versions: 2.0, 2.1"),
        ));
    }

    let tests = match version.as_str() {
        "2.0" => Csaf2_0::tests_in_preset(&preset),
        "2.1" => Csaf2_1::tests_in_preset(&preset),
        _ => None,
    };

    match tests {
        Some(test_ids) => Ok(Json(PresetTestsResponse {
            preset,
            version,
            tests: test_ids.into_iter().map(|s| s.to_string()).collect(),
        })),
        None => Err(error_response(
            StatusCode::BAD_REQUEST,
            format!("Unknown preset '{preset}' for CSAF version {version}"),
        )),
    }
}
