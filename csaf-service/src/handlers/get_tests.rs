use axum::{Json, extract::Path, http::StatusCode};
use csaf::csaf_traits::CsafVersion;
use csaf::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework as Csaf2_1;
use csaf::{schema::csaf2_0::schema::CommonSecurityAdvisoryFramework as Csaf2_0, validation::Validatable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::errors::*;

#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct TestsResponse {
    pub version: String,
    pub tests: Vec<TestInPreset>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq)]
pub(crate) struct TestInPreset {
    pub name: String,
    pub preset: String,
}

/// List available tests for a CSAF version.
#[utoipa::path(
    get,
    path = "/api/v1/csaf/{version}/tests",
    params(
        ("version" = String, Path, description = "CSAF version (2.0 or 2.1)")
    ),
    responses(
        (status = 200, description = "List of available tests", body = TestsResponse),
        (status = 404, description = "Invalid version", body = ErrorResponse)
    ),
    tag = "meta"
)]
pub(crate) async fn get_tests(
    Path(version): Path<String>,
) -> Result<Json<TestsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let valid_version = CsafVersion::try_from(version.clone()).map_err(|e| error_response(StatusCode::NOT_FOUND, e))?;
    Ok(Json(TestsResponse {
        version,
        tests: tests_for_version(&valid_version),
    }))
}

/// Returns the list of known preset names for a CSAF version.
pub(crate) fn tests_for_version(version: &CsafVersion) -> Vec<TestInPreset> {
    match version {
        CsafVersion::X20 => Csaf2_0::get_tests()
            .iter()
            .map(|f| TestInPreset {
                name: f.0.to_string(),
                preset: f.1.to_string(),
            })
            .collect(),
        CsafVersion::X21 => Csaf2_1::get_tests()
            .iter()
            .map(|f| TestInPreset {
                name: f.0.to_string(),
                preset: f.1.to_string(),
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use csaf::csaf2_0::testcases::informative_tests as informative_tests_2_0;
    use csaf::csaf2_0::testcases::mandatory_tests as mandatory_tests_2_0;
    use csaf::csaf2_0::testcases::recommended_tests as recommended_tests_2_0;
    use csaf::csaf2_0::validation::Preset as Preset_2_0;
    use csaf::csaf2_1::testcases::informative_tests as informative_tests_2_1;
    use csaf::csaf2_1::testcases::mandatory_tests as mandatory_tests_2_1;
    use csaf::csaf2_1::testcases::recommended_tests as recommended_tests_2_1;
    use csaf::csaf2_1::validation::Preset as Preset_2_1;

    use super::*;
    use crate::routes;
    use crate::test_helpers::get_json;

    fn build_uri(version: &str) -> String {
        routes::TESTS.replace("{version}", version)
    }

    #[tokio::test]
    async fn returns_tests_for_csaf_2_0() {
        let (status, json) = get_json(&build_uri("2.0")).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["version"], "2.0");

        let tests: Vec<TestInPreset> = serde_json::from_value(json["tests"].clone()).unwrap();
        assert_eq!(
            tests.len(),
            mandatory_tests_2_0().len() + recommended_tests_2_0().len() + informative_tests_2_0().len() + 1
        );
        assert!(tests.contains(&TestInPreset {
            name: Preset_2_0::Schema.to_string(),
            preset: Preset_2_0::Schema.to_string()
        }));
    }

    #[tokio::test]
    async fn returns_tests_for_csaf_2_1() {
        let (status, json) = get_json(&build_uri("2.1")).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["version"], "2.1");

        let tests: Vec<TestInPreset> = serde_json::from_value(json["tests"].clone()).unwrap();
        assert_eq!(
            tests.len(),
            mandatory_tests_2_1().len() + recommended_tests_2_1().len() + informative_tests_2_1().len() + 1
        );
        assert!(tests.contains(&TestInPreset {
            name: Preset_2_1::Schema.to_string(),
            preset: Preset_2_1::Schema.to_string()
        }));
    }

    #[tokio::test]
    async fn returns_404_for_invalid_version() {
        let (status, json) = get_json(&build_uri("3.0")).await;

        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(json["error"].as_str().unwrap().contains("3.0"));
    }

    #[tokio::test]
    async fn returns_404_for_non_numeric_version() {
        let (status, json) = get_json(&build_uri("abc")).await;

        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(json["error"].is_string());
    }
}
