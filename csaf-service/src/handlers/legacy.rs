use axum::{Json, extract::Query, http::StatusCode};
use csaf::csaf_traits::CsafVersion;
use csaf::validation::{TestResultStatus, Validatable, ValidationResult, validate_by_tests};
use csaf::{
    csaf::loader::detect_version, csaf2_0::loader::load_document as load_2_0,
    csaf2_1::loader::load_document as load_2_1,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::errors::*;
use crate::handlers::get_tests::{TestInPreset, tests_for_version};

type CsafDoc20 = csaf::csaf::raw::RawDocument<csaf::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>;
type CsafDoc21 = csaf::csaf::raw::RawDocument<csaf::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>;

#[derive(Debug, Deserialize)]
pub(crate) struct LegacyTestsQuery {
    pub version: Option<String>,
}

/// Retrieve all tests.
#[utoipa::path(
    get,
    path = "/api/v1/tests",
    description = "Retrieve all tests for the requested CSAF version (2.0 by default). For each test, return the test number as well as the primary preset it belongs to.",
    params(
        ("version" = Option<String>, Query, description = "CSAF version (2.0 or 2.1). Defaults to 2.0."),
    ),
    responses(
        (status = 200, description = "List of available tests", body = Vec<TestInPreset>),
        (status = 404, description = "Invalid version", body = ErrorResponse),
    ),
    tag = "meta"
)]
pub(crate) async fn get_tests_legacy(
    Query(query): Query<LegacyTestsQuery>,
) -> Result<Json<Vec<TestInPreset>>, (StatusCode, Json<ErrorResponse>)> {
    let version = CsafVersion::try_from(query.version.clone().unwrap_or_else(|| "2.0".to_string()))
        .map_err(|e| error_response(StatusCode::NOT_FOUND, e))?;
    Ok(Json(tests_for_version(&version)))
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type")]
pub(crate) enum TestOrPreset {
    #[serde(rename = "test")]
    Test { name: String },
    #[serde(rename = "preset")]
    Preset { name: String },
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub(crate) struct LegacyValidateBody {
    pub tests: Vec<TestOrPreset>,
    pub document: serde_json::Value,
}

/// Legacy validation response matching the secvisogram csaf-validator-service format.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LegacyValidateResponse {
    pub is_valid: bool,
    pub tests: Vec<LegacyTestResult>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LegacyTestResult {
    pub name: String,
    pub is_valid: bool,
    pub errors: Vec<LegacyFinding>,
    pub warnings: Vec<LegacyFinding>,
    pub infos: Vec<LegacyFinding>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LegacyFinding {
    pub instance_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

fn to_legacy_response(result: ValidationResult) -> LegacyValidateResponse {
    let tests: Vec<LegacyTestResult> = result
        .test_results
        .into_iter()
        .map(|tr| {
            let (is_valid, errors, warnings, infos) = match tr.status {
                TestResultStatus::Success | TestResultStatus::Skipped | TestResultStatus::NotFound => {
                    (true, vec![], vec![], vec![])
                },
                TestResultStatus::Failure {
                    errors: errs,
                    warnings: warns,
                    infos: info_items,
                } => {
                    let errors = errs
                        .into_iter()
                        .map(|e| LegacyFinding {
                            instance_path: e.instance_path,
                            message: Some(e.message),
                        })
                        .collect();
                    let warnings = warns
                        .into_iter()
                        .map(|w| LegacyFinding {
                            instance_path: w.instance_path,
                            message: Some(w.message),
                        })
                        .collect();
                    let infos = info_items
                        .into_iter()
                        .map(|i| LegacyFinding {
                            instance_path: i.instance_path,
                            message: Some(i.message),
                        })
                        .collect();
                    (false, errors, warnings, infos)
                },
            };
            LegacyTestResult {
                name: tr.test_id,
                is_valid,
                errors,
                warnings,
                infos,
            }
        })
        .collect();

    let is_valid = tests.iter().all(|t| t.is_valid);
    LegacyValidateResponse { is_valid, tests }
}

/// Validate a CSAF document.
#[utoipa::path(
    post,
    path = "/api/v1/validate",
    description = "Evaluates a CSAF document against a selected set of tests.<br/>
    At least one entry has to be provided in the **tests** array.
    Each entry provided runs either a single named test or a named preset (which expands to a fixed set of tests).
    Duplicate tests from overlapping entries are automatically removed.<br/>
    <b>Available presets:</b><br/>
    - schema (JSON schema validation)<br/>
    - mandatory (all mandatory tests from section 6.1)<br/>
    - optional (all optional tests from section 6.2)<br/>
    - informative (all informative tests from section 6.3)<br/>
    - basic (schema & mandatory)<br/>
    - extended (basic & optional)<br/>
    - full (extended & informative)<br/>
    ",
    request_body(
        content = LegacyValidateBody,
        description = "Validation request with document and tests/presets",
        examples(
            ("Validate with a single test" = (
             //   summary = "Validate with a single test",
                value = json!({"tests": [{"type": "test", "name": "6.1.15"}], "document": {"document": {"category": "csaf_base", "csaf_version": "2.0", "publisher": {"category": "vendor", "name": "Example", "namespace": "https://example.com"}, "title": "Example", "tracking": {"current_release_date": "2024-01-01T00:00:00Z", "id": "Example-001", "initial_release_date": "2024-01-01T00:00:00Z", "revision_history": [{"date": "2024-01-01T00:00:00Z", "number": "1", "summary": "Initial"}], "status": "final", "version": "1"}}}})
            )),
            ("Validate with the basic preset" = (
                summary = "Validate with the basic preset",
                value = json!({"tests": [{"type": "preset", "name": "basic"}], "document": {"document": {"category": "csaf_base", "csaf_version": "2.0"}}})
            )),
            ("Combine individual tests with presets" = (
                summary = "Combine individual tests with presets",
                value = json!({"tests": [{"type": "preset", "name": "basic"}, {"type": "test", "name": "6.2.1"}], "document": {"document": {"category": "csaf_base", "csaf_version": "2.0"}}})
            ))
        )
    ),
    responses(
        (status = 200, description = "Validation result", body = LegacyValidateResponse,
            examples(
                ("Valid document" = (
                    summary = "All tests passed",
                    value = json!({"isValid": true, "tests": [{"name": "schema", "isValid": true, "errors": [], "warnings": [], "infos": []}]})
                )),
                ("Invalid document" = (
                    summary = "Document has validation errors",
                    value = json!({"isValid": false, "tests": [{"name": "schema", "isValid": false, "errors": [{"instancePath": "/document", "message": "required property 'publisher' is missing"}], "warnings": [], "infos": []}]})
                ))
            )
        ),
        (status = 400, description = "Invalid request", body = ErrorResponse)
    ),
    tag = "validation"
)]
pub(crate) async fn validate_legacy(
    Json(body): Json<LegacyValidateBody>,
) -> Result<Json<LegacyValidateResponse>, (StatusCode, Json<ErrorResponse>)> {
    let json_value = body.document;

    let version = {
        let detected =
            detect_version(json_value.clone()).map_err(|e| error_response(StatusCode::BAD_REQUEST, e.to_string()))?;
        CsafVersion::try_from(detected).map_err(|e| error_response(StatusCode::BAD_REQUEST, e))?
    };

    let mut test_ids: Vec<String> = Vec::new();
    for entry in &body.tests {
        match entry {
            TestOrPreset::Test { name } => test_ids.push(name.clone()),
            TestOrPreset::Preset { name } => {
                let preset_tests = match version {
                    CsafVersion::X20 => CsafDoc20::tests_in_preset(name)
                        .map_err(|e| error_response(StatusCode::BAD_REQUEST, e.to_string()))?,
                    CsafVersion::X21 => CsafDoc21::tests_in_preset(name)
                        .map_err(|e| error_response(StatusCode::BAD_REQUEST, e.to_string()))?,
                };
                test_ids.extend(preset_tests.iter().map(|s| s.to_string()));
            },
        }
    }

    test_ids.sort();
    test_ids.dedup();

    if test_ids.is_empty() {
        test_ids.push("schema".to_string());
    }

    let test_id_refs: Vec<&str> = test_ids.iter().map(|s| s.as_str()).collect();

    let result = match version {
        CsafVersion::X20 => {
            let doc = load_2_0(json_value).map_err(|e| {
                error_response(
                    StatusCode::BAD_REQUEST,
                    format!("Failed to load CSAF 2.0 document: {e}"),
                )
            })?;
            validate_by_tests(&doc, version.as_str(), &test_id_refs)
        },
        CsafVersion::X21 => {
            let doc = load_2_1(json_value).map_err(|e| {
                error_response(
                    StatusCode::BAD_REQUEST,
                    format!("Failed to load CSAF 2.1 document: {e}"),
                )
            })?;
            validate_by_tests(&doc, version.as_str(), &test_id_refs)
        },
    };

    Ok(Json(to_legacy_response(result)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes;
    use crate::test_helpers::post_json;
    use axum::http::StatusCode;

    fn valid_csaf_2_0() -> serde_json::Value {
        let bytes = include_bytes!(
            "../../../csaf/csaf_2.0/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_0-2021-6-1-01-11.json"
        );
        serde_json::from_slice(bytes).unwrap()
    }

    #[tokio::test]
    async fn validate_legacy_with_test() {
        let body = serde_json::json!({
            "tests": [{"type": "test", "name": "schema"}],
            "document": valid_csaf_2_0()
        });
        let (status, json) = post_json(routes::VALIDATE_LEGACY, body).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["isValid"], true);
        assert!(!json["tests"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn validate_legacy_with_preset() {
        let body = serde_json::json!({
            "tests": [{"type": "preset", "name": "basic"}],
            "document": valid_csaf_2_0()
        });
        let (status, json) = post_json(routes::VALIDATE_LEGACY, body).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["isValid"], true);
    }

    #[tokio::test]
    async fn validate_legacy_response_structure() {
        let body = serde_json::json!({
            "tests": [{"type": "test", "name": "schema"}],
            "document": valid_csaf_2_0()
        });
        let (status, json) = post_json(routes::VALIDATE_LEGACY, body).await;

        assert_eq!(status, StatusCode::OK);
        // Check legacy response shape
        assert!(json["isValid"].is_boolean());
        let tests = json["tests"].as_array().unwrap();
        let test_result = &tests[0];
        assert!(test_result["name"].is_string());
        assert!(test_result["isValid"].is_boolean());
        assert!(test_result["errors"].is_array());
        assert!(test_result["warnings"].is_array());
        assert!(test_result["infos"].is_array());
    }

    #[tokio::test]
    async fn validate_legacy_invalid_document() {
        let body = serde_json::json!({
            "tests": [{"type": "test", "name": "schema"}],
            "document": "not an object"
        });
        let (status, json) = post_json(routes::VALIDATE_LEGACY, body).await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(!json["error"].as_str().unwrap().is_empty());
    }

    #[test]
    fn test_or_preset_serializes_with_type_tag() {
        let test = TestOrPreset::Test {
            name: "schema".to_string(),
        };
        let json = serde_json::to_value(&test).unwrap();
        assert_eq!(json["type"], "test");
        assert_eq!(json["name"], "schema");

        let preset = TestOrPreset::Preset {
            name: "basic".to_string(),
        };
        let json = serde_json::to_value(&preset).unwrap();
        assert_eq!(json["type"], "preset");
        assert_eq!(json["name"], "basic");
    }

    #[test]
    fn test_or_preset_deserializes_from_type_tag() {
        let json = serde_json::json!({"type": "test", "name": "schema"});
        let parsed: TestOrPreset = serde_json::from_value(json).unwrap();
        assert!(matches!(parsed, TestOrPreset::Test { name } if name == "schema"));

        let json = serde_json::json!({"type": "preset", "name": "basic"});
        let parsed: TestOrPreset = serde_json::from_value(json).unwrap();
        assert!(matches!(parsed, TestOrPreset::Preset { name } if name == "basic"));
    }
}
