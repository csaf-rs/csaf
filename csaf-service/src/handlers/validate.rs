use axum::{
    Json,
    body::Bytes,
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
};
use csaf::{
    csaf::loader::detect_version_from_json,
    csaf2_0::loader::load_document_from_str as load_2_0,
    csaf2_1::loader::load_document_from_str as load_2_1,
    validation::{Validatable, validate_by_tests},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::{ErrorResponse, error_response};
type CsafDoc20 = csaf::csaf::raw::RawDocument<csaf::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>;
type CsafDoc21 = csaf::csaf::raw::RawDocument<csaf::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>;

#[derive(Debug, Deserialize)]
pub struct ValidateQuery {
    pub preset: Option<String>,
    pub tests: Option<String>,
}

// OpenAPI schema types mirroring csaf::validation types.
// These exist because the upstream types don't derive utoipa::ToSchema.

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(as = ValidationResult)]
pub struct ValidationResultSchema {
    pub success: bool,
    pub version: String,
    pub test_results: Vec<TestResultSchema>,
    pub num_errors: usize,
    pub num_warnings: usize,
    pub num_infos: usize,
    pub num_not_found: usize,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(as = TestResult)]
pub struct TestResultSchema {
    pub test_id: String,
    pub status: TestResultStatusSchema,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(as = TestResultStatus)]
#[allow(dead_code)]
pub enum TestResultStatusSchema {
    Success,
    Failure {
        errors: Vec<ValidationErrorSchema>,
        warnings: Vec<ValidationErrorSchema>,
        infos: Vec<ValidationErrorSchema>,
    },
    NotFound,
    Skipped,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(as = ValidationError)]
pub struct ValidationErrorSchema {
    pub message: String,
    pub instance_path: String,
}

/// Validate a CSAF document.
#[utoipa::path(
    post,
    path = "/api/v1/csaf/{version}/validate",
    params(
        ("version" = String, Path, description = "CSAF version (2.0, 2.1, or auto)"),
        ("preset" = Option<String>, Query, description = "Validation preset (default: basic)"),
        ("tests" = Option<String>, Query, description = "Comma-separated test IDs (overrides preset)")
    ),
    request_body(content = serde_json::Value, description = "CSAF JSON document", content_type = "application/json"),
    responses(
        (status = 200, description = "Validation result", body = ValidationResultSchema),
        (status = 400, description = "Invalid request", body = ErrorResponse)
    ),
    tag = "validation"
)]
pub async fn validate(
    Path(path_version): Path<String>,
    Query(query): Query<ValidateQuery>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    let json_str = serde_json::to_string(&body).unwrap();
    run_validation(&path_version, &query, &json_str, &body)
}

/// Validate a CSAF document uploaded as a binary file.
#[utoipa::path(
    post,
    path = "/api/v1/csaf/{version}/validate/file",
    params(
        ("version" = String, Path, description = "CSAF version (2.0, 2.1, or auto)"),
        ("preset" = Option<String>, Query, description = "Validation preset (default: basic)"),
        ("tests" = Option<String>, Query, description = "Comma-separated test IDs (overrides preset)")
    ),
    request_body(content = String, description = "CSAF JSON file", content_type = "application/octet-stream"),
    responses(
        (status = 200, description = "Validation result", body = ValidationResultSchema),
        (status = 400, description = "Invalid request", body = ErrorResponse)
    ),
    tag = "validation"
)]
pub async fn validate_file(
    Path(path_version): Path<String>,
    Query(query): Query<ValidateQuery>,
    body: Bytes,
) -> impl IntoResponse {
    let json_str = match std::str::from_utf8(&body) {
        Ok(s) => s,
        Err(e) => {
            return Err(error_response(
                StatusCode::BAD_REQUEST,
                format!("Invalid UTF-8 in uploaded file: {e}"),
            ));
        },
    };

    let parsed: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            return Err(error_response(
                StatusCode::BAD_REQUEST,
                format!("Invalid JSON in uploaded file: {e}"),
            ));
        },
    };

    run_validation(&path_version, &query, json_str, &parsed)
}

/// Common validation logic shared by `validate` and `validate_file`.
fn run_validation(
    path_version: &str,
    query: &ValidateQuery,
    json_str: &str,
    parsed: &serde_json::Value,
) -> Result<Json<csaf::validation::ValidationResult>, (StatusCode, Json<ErrorResponse>)> {
    let version = resolve_version(path_version, parsed)
        .map_err(|e| error_response(StatusCode::BAD_REQUEST, e))?;

    let test_ids = resolve_test_ids(&version, query)
        .map_err(|e| error_response(StatusCode::BAD_REQUEST, e))?;

    let test_id_refs: Vec<&str> = test_ids.iter().map(|s| s.as_str()).collect();

    let result = match version.as_str() {
        "2.0" => {
            let doc = load_2_0(json_str).map_err(|e| {
                error_response(
                    StatusCode::BAD_REQUEST,
                    format!("Failed to load CSAF 2.0 document: {e}"),
                )
            })?;
            validate_by_tests(&doc, &version, &test_id_refs)
        },
        "2.1" => {
            let doc = load_2_1(json_str).map_err(|e| {
                error_response(
                    StatusCode::BAD_REQUEST,
                    format!("Failed to load CSAF 2.1 document: {e}"),
                )
            })?;
            validate_by_tests(&doc, &version, &test_id_refs)
        },
        _ => {
            return Err(error_response(
                StatusCode::BAD_REQUEST,
                format!("Unsupported CSAF version: {version}"),
            ));
        },
    };

    Ok(Json(result))
}

/// Resolve the effective CSAF version from the path parameter and optional JSON body.
fn resolve_version(path_version: &str, body: &serde_json::Value) -> Result<String, String> {
    match path_version {
        "auto" => detect_version_from_json(body).map_err(|e| format!("Failed to auto-detect CSAF version: {e}")),
        "2.0" | "2.1" => Ok(path_version.to_string()),
        other => Err(format!("Invalid CSAF version: {other}. Supported: 2.0, 2.1, auto")),
    }
}

/// Resolve test IDs from query parameters.
fn resolve_test_ids(version: &str, query: &ValidateQuery) -> Result<Vec<String>, String> {
    // If explicit test IDs are provided, use them
    if let Some(tests_str) = &query.tests {
        return Ok(tests_str.split(',').map(|s| s.trim().to_string()).collect());
    }

    // Otherwise use the preset
    let preset = query.preset.as_deref().unwrap_or("basic");

    let tests = match version {
        "2.0" => CsafDoc20::tests_in_preset(preset),
        "2.1" => CsafDoc21::tests_in_preset(preset),
        _ => None,
    };

    tests
        .map(|ids| ids.into_iter().map(|s| s.to_string()).collect())
        .ok_or_else(|| format!("Unknown preset '{preset}' for CSAF version {version}"))
}
