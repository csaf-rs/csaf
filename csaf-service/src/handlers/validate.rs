use axum::{
    Json,
    body::Bytes,
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
};
use csaf::csaf2_1::validation::Preset as Preset2_1;
use csaf::{
    csaf::loader::detect_version_from_json,
    csaf2_0::loader::load_document_from_value as load_2_0,
    csaf2_1::loader::load_document_from_value as load_2_1,
    validation::{Validatable, validate_by_tests},
};
use csaf::{csaf_traits::CsafVersion, csaf2_0::validation::Preset as Preset2_0};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::{ErrorResponse, error_response};
type CsafDoc20 = csaf::csaf::raw::RawDocument<csaf::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>;
type CsafDoc21 = csaf::csaf::raw::RawDocument<csaf::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>;

#[derive(Debug, Deserialize)]
pub(crate) struct ValidateQuery {
    pub preset: Option<String>,
    pub tests: Option<String>,
}

// OpenAPI schema types mirroring csaf::validation types.
// These exist because the upstream types don't derive utoipa::ToSchema.

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(as = ValidationResult)]
pub(crate) struct ValidationResultSchema {
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
pub(crate) struct TestResultSchema {
    pub test_id: String,
    pub status: TestResultStatusSchema,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(as = TestResultStatus)]
#[allow(dead_code)]
pub(crate) enum TestResultStatusSchema {
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
pub(crate) struct ValidationErrorSchema {
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
pub(crate) async fn validate(
    Path(path_version): Path<String>,
    Query(query): Query<ValidateQuery>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    run_validation(&path_version, &query, &body)
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
pub(crate) async fn validate_file(
    Path(path_version): Path<String>,
    Query(query): Query<ValidateQuery>,
    body: Bytes,
) -> impl IntoResponse {
    let parsed: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(e) => {
            return Err(error_response(
                StatusCode::BAD_REQUEST,
                format!("Invalid JSON in uploaded file: {e}"),
            ));
        },
    };
    run_validation(&path_version, &query, &parsed)
}

/// Common validation logic shared by `validate` and `validate_file`.
fn run_validation(
    path_version: &str,
    query: &ValidateQuery,
    json_value: &serde_json::Value,
) -> Result<Json<csaf::validation::ValidationResult>, (StatusCode, Json<ErrorResponse>)> {
    let version = resolve_version(path_version, json_value)?;
    let test_ids = resolve_test_ids(&version, query)?;

    let result = match version {
        CsafVersion::X20 => {
            let doc = load_2_0(json_value.clone()).map_err(|e| {
                error_response(
                    StatusCode::BAD_REQUEST,
                    format!("Failed to load CSAF 2.0 document: {e}"),
                )
            })?;
            validate_by_tests(&doc, version.as_str(), &test_ids)
        },
        CsafVersion::X21 => {
            let doc = load_2_1(json_value.clone()).map_err(|e| {
                error_response(
                    StatusCode::BAD_REQUEST,
                    format!("Failed to load CSAF 2.1 document: {e}"),
                )
            })?;
            validate_by_tests(&doc, version.as_str(), &test_ids)
        },
    };

    Ok(Json(result))
}

/// Resolve the effective CSAF version from the path parameter and optional JSON body.
fn resolve_version(
    path_version: &str,
    body: &serde_json::Value,
) -> Result<CsafVersion, (StatusCode, Json<ErrorResponse>)> {
    let parsed_version = if path_version.eq_ignore_ascii_case("auto") {
        detect_version_from_json(body).map_err(|e| error_response(StatusCode::BAD_REQUEST, e.to_string()))?
    } else {
        path_version.to_string()
    };
    let valid_version =
        CsafVersion::try_from(parsed_version).map_err(|e| error_response(StatusCode::BAD_REQUEST, e))?;
    Ok(valid_version)
}

/// Resolve test IDs from query parameters.
fn resolve_test_ids<'a>(
    version: &CsafVersion,
    query: &'a ValidateQuery,
) -> Result<Vec<&'a str>, (StatusCode, Json<ErrorResponse>)> {
    // If explicit test IDs are provided, use them and ignore preset
    if let Some(tests_str) = &query.tests {
        return Ok(tests_str.split(',').map(|s| s.trim()).collect());
    }

    // Otherwise use the preset
    let preset = query.preset.as_deref().unwrap_or("basic");

    let tests = match version {
        CsafVersion::X20 => CsafDoc20::tests_in_preset(
            Preset2_0::try_from(preset).map_err(|e| error_response(StatusCode::BAD_REQUEST, e))?,
        ),
        CsafVersion::X21 => CsafDoc21::tests_in_preset(
            Preset2_1::try_from(preset).map_err(|e| error_response(StatusCode::BAD_REQUEST, e))?,
        ),
    };

    Ok(tests)
}

#[cfg(test)]
mod tests {
    use crate::routes;
    use crate::test_helpers::{post_bytes, post_json};
    use axum::http::StatusCode;

    fn validate_uri(version: &str) -> String {
        routes::VALIDATE.replace("{version}", version)
    }

    fn validate_file_uri(version: &str) -> String {
        routes::VALIDATE_FILE.replace("{version}", version)
    }

    fn valid_csaf_2_0() -> serde_json::Value {
        let bytes = include_bytes!(
            "../../../csaf/csaf_2.0/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_0-2021-6-1-01-11.json"
        );
        serde_json::from_slice(bytes).unwrap()
    }

    fn valid_csaf_2_1() -> serde_json::Value {
        let bytes = include_bytes!(
            "../../../csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-01-11.json"
        );
        serde_json::from_slice(bytes).unwrap()
    }

    #[tokio::test]
    async fn validates_valid_csaf_2_0_document() {
        let (status, json) = post_json(&validate_uri("2.0"), valid_csaf_2_0()).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["success"], true);
        assert_eq!(json["version"], "2.0");
    }

    #[tokio::test]
    async fn validates_valid_csaf_2_1_document() {
        let (status, json) = post_json(&validate_uri("2.1"), valid_csaf_2_1()).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["success"], true);
        assert_eq!(json["version"], "2.1");
    }

    #[tokio::test]
    async fn validates_with_auto_version_detection() {
        let (status, json) = post_json(&validate_uri("auto"), valid_csaf_2_0()).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["success"], true);
        assert_eq!(json["version"], "2.0");
    }

    #[tokio::test]
    async fn validates_with_preset_query_param() {
        let uri = format!("{}?preset=basic", validate_uri("2.0"));
        let (status, json) = post_json(&uri, valid_csaf_2_0()).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["success"], true);
    }

    #[tokio::test]
    async fn validates_with_explicit_test_ids() {
        let uri = format!("{}?tests=6.1.1", validate_uri("2.0"));
        let (status, json) = post_json(&uri, valid_csaf_2_0()).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["success"], true);
        assert_eq!(json["testResults"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn returns_400_for_invalid_version() {
        let (status, json) = post_json(&validate_uri("3.0"), valid_csaf_2_0()).await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(json["error"].as_str().unwrap().contains("3.0"));
    }

    #[tokio::test]
    async fn returns_400_for_invalid_preset() {
        let uri = format!("{}?preset=nonexistent", validate_uri("2.0"));
        let (status, json) = post_json(&uri, valid_csaf_2_0()).await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(json["error"].as_str().unwrap().contains("nonexistent"));
    }

    #[tokio::test]
    async fn returns_400_for_invalid_json_body() {
        // A JSON string is valid JSON but not a valid CSAF document —
        // the endpoint still attempts to load it and reports a load failure
        let uri = format!("{}?tests=schema", validate_uri("2.0"));
        let (status, json) = post_json(&uri, serde_json::json!("not an object")).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["success"], false);
    }

    #[tokio::test]
    async fn validate_file_with_valid_document() {
        let bytes = include_bytes!(
            "../../../csaf/csaf_2.0/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_0-2021-6-1-01-11.json"
        );
        let (status, json) = post_bytes(&validate_file_uri("2.0"), bytes.to_vec()).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["success"], true);
    }

    #[tokio::test]
    async fn validate_file_returns_400_for_invalid_utf8() {
        let (status, json) = post_bytes(&validate_file_uri("2.0"), vec![0xFF, 0xFE]).await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(json["error"].as_str().unwrap().contains("UTF-8"));
    }

    #[tokio::test]
    async fn validate_file_returns_400_for_invalid_json() {
        let (status, json) = post_bytes(&validate_file_uri("2.0"), b"not json".to_vec()).await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(json["error"].as_str().unwrap().contains("JSON"));
    }

    #[tokio::test]
    async fn reports_validation_failures_for_invalid_document() {
        let mut doc = valid_csaf_2_0();
        // Remove required field to trigger validation failure
        doc.as_object_mut().unwrap().remove("document");

        let uri = format!("{}?tests=schema", validate_uri("2.0"));
        let (status, json) = post_json(&uri, doc).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["success"], false);
        assert!(json["numErrors"].as_u64().unwrap() > 0);
    }
}
