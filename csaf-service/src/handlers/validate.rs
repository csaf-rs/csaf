use axum::{
    Json,
    body::Bytes,
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
};
use csaf::csaf_traits::CsafVersion;
use csaf::{
    csaf::loader::detect_version,
    csaf2_0::loader::load_document as load_2_0,
    csaf2_1::loader::load_document as load_2_1,
    validation::{Validatable, validate_by_tests},
};
use serde::Deserialize;

use crate::errors::{ErrorResponse, error_response};
type CsafDoc20 = csaf::csaf::raw::RawDocument<csaf::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>;
type CsafDoc21 = csaf::csaf::raw::RawDocument<csaf::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>;

#[derive(Debug, Deserialize)]
pub(crate) struct ValidateQuery {
    pub preset: Option<String>,
    pub tests: Option<String>,
    pub exclude_tests: Option<String>,
}

/// Validate a CSAF document.
///
/// Accepts either `application/json` (parsed CSAF document) or
/// `application/octet-stream` (raw file upload) via the Content-Type header.
#[utoipa::path(
    post,
    path = "/api/v1/csaf/{version}/validate",
    description = "Validate a CSAF document. Send the document as JSON (Content-Type: application/json) or as a raw file upload (Content-Type: application/octet-stream). If neither preset nor explicit tests are specified, the schema test will be run by default.",
    params(
        ("version" = String, Path, description = "CSAF version (2.0, 2.1, or auto)"),
        ("preset" = Option<String>, Query, description = "Validation preset"),
        ("tests" = Option<String>, Query, description = "Comma-separated test IDs (additional to preset)"),
        ("exclude_tests" = Option<String>, Query, description = "Comma-separated test IDs (excluded from preset or explicit list)"),
    ),
    request_body(
        description = "CSAF JSON document",
        content(
            (serde_json::Value = "application/json"),
            (String = "application/octet-stream"),
        )
    ),
    responses(
        (status = 200, description = "Validation result", body = csaf::validation::ValidationResult),
        (status = 400, description = "Invalid request", body = ErrorResponse)
    ),
    tag = "validation"
)]
pub(crate) async fn validate(
    Path(path_version): Path<String>,
    Query(query): Query<ValidateQuery>,
    body: Bytes,
) -> impl IntoResponse {
    let json_value: serde_json::Value = serde_json::from_slice(&body).map_err(|e| {
        error_response(StatusCode::BAD_REQUEST, format!("Invalid JSON: {e}"))
    })?;
    run_validation(&path_version, &query, &json_value)
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
        detect_version(body.to_owned()).map_err(|e| error_response(StatusCode::BAD_REQUEST, e.to_string()))?
    } else {
        path_version.to_string()
    };
    let valid_version = CsafVersion::try_from(parsed_version).map_err(|e| error_response(StatusCode::NOT_FOUND, e))?;
    Ok(valid_version)
}

/// Resolve test IDs from query parameters.
fn resolve_test_ids<'a>(
    version: &CsafVersion,
    query: &'a ValidateQuery,
) -> Result<Vec<&'a str>, (StatusCode, Json<ErrorResponse>)> {
    let mut tests: Vec<&str> = Vec::new();
    // resolve preset tests
    if let Some(preset) = &query.preset {
        let preset_tests =
            match version {
                CsafVersion::X20 => CsafDoc20::tests_in_preset(preset)
                    .map_err(|e| error_response(StatusCode::NOT_FOUND, e.to_string()))?,
                CsafVersion::X21 => CsafDoc21::tests_in_preset(preset)
                    .map_err(|e| error_response(StatusCode::NOT_FOUND, e.to_string()))?,
            };
        tests.extend(preset_tests);
    }

    // add additional requested tests
    if let Some(tests_str) = &query.tests {
        let only_tests: Vec<&str> = tests_str
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        tests.extend(only_tests);
    }

    // remove excluded tests
    if let Some(exclude_str) = &query.exclude_tests {
        let exclude_tests: Vec<&str> = exclude_str
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        tests.retain(|t| !exclude_tests.contains(t));
    }

    tests.sort();
    tests.dedup();

    if tests.is_empty() {
        tests.push("schema");
    }

    Ok(tests)
}

#[cfg(test)]
mod tests {
    use crate::routes;
    use crate::test_helpers::{post_bytes, post_json};
    use axum::http::StatusCode;
    use csaf::csaf2_0::testcases::mandatory_tests;

    fn validate_uri(version: &str) -> String {
        routes::VALIDATE.replace("{version}", version)
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
        assert_eq!(
            json["testResults"].as_array().unwrap().len(),
            mandatory_tests().len() + 1 // +1 for the schema test that's always included in presets
        );
    }

    #[tokio::test]
    async fn validates_with_preset_and_exclude_query_param() {
        let uri = format!("{}?preset=basic&exclude_tests=6.1.1", validate_uri("2.0"));
        let (status, json) = post_json(&uri, valid_csaf_2_0()).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["success"], true);
        assert_eq!(
            json["testResults"].as_array().unwrap().len(),
            mandatory_tests().len() // +1 for schema, -1 for the excluded test
        );
    }

    #[tokio::test]
    async fn validates_with_explicit_test_ids() {
        let uri = format!("{}?tests=6.1.1,6.1.2", validate_uri("2.0"));
        let (status, json) = post_json(&uri, valid_csaf_2_0()).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["success"], true);
        assert_eq!(json["testResults"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn returns_404_for_invalid_version() {
        let (status, json) = post_json(&validate_uri("3.0"), valid_csaf_2_0()).await;

        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(json["error"].as_str().unwrap().contains("3.0"));
    }

    #[tokio::test]
    async fn returns_404_for_invalid_preset() {
        let uri = format!("{}?preset=nonexistent", validate_uri("2.0"));
        let (status, json) = post_json(&uri, valid_csaf_2_0()).await;

        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(json["error"].as_str().unwrap().contains("nonexistent"));
    }

    #[tokio::test]
    async fn returns_400_for_invalid_json_body() {
        let uri = format!("{}?tests=schema", validate_uri("2.0"));
        let (status, json) = post_json(&uri, serde_json::json!("not an object")).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["success"], false);
    }

    #[tokio::test]
    async fn validates_file_upload_with_valid_document() {
        let bytes = include_bytes!(
            "../../../csaf/csaf_2.0/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_0-2021-6-1-01-11.json"
        );
        let (status, json) = post_bytes(&validate_uri("2.0"), bytes.to_vec()).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["success"], true);
    }

    #[tokio::test]
    async fn returns_400_for_invalid_utf8_upload() {
        let (status, json) = post_bytes(&validate_uri("2.0"), vec![0xFF, 0xFE]).await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(json["error"].as_str().unwrap().contains("JSON"));
    }

    #[tokio::test]
    async fn returns_400_for_invalid_json_upload() {
        let (status, json) = post_bytes(&validate_uri("2.0"), b"not json".to_vec()).await;

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
