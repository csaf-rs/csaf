use axum::Json;
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use csaf::csaf2_0::validation::Preset as Preset2_0;
use csaf::csaf2_1::validation::Preset as Preset2_1;
use csaf::csaf_traits::CsafVersion;
use csaf::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework as Csaf2_0;
use csaf::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework as Csaf2_1;
use csaf::validation::Validatable;
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
                tests: tests_for_preset(&valid_version, &preset),
                preset,
                version,
            }))
        }
    }
}

/// Returns the test IDs belonging to a preset for a given CSAF version.
pub(crate) fn tests_for_preset(version: &CsafVersion, preset: &str) -> Vec<String> {
    match version {
        CsafVersion::X20 => {
            let p = Preset2_0::try_from(preset).expect("preset already validated");
            Csaf2_0::tests_in_preset(p).into_iter().map(String::from).collect()
        }
        CsafVersion::X21 => {
            let p = Preset2_1::try_from(preset).expect("preset already validated");
            Csaf2_1::tests_in_preset(p).into_iter().map(String::from).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, body::Body, routing::get};
    use http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    fn test_app(route: &str) -> Router {
        Router::new().route(route, get(get_preset_tests))
    }

    const ROUTE: &str = "/api/v1/csaf/{version}/presets/{preset}/tests";
    fn build_uri(version: &str, preset: &str) -> String {
        ROUTE.replace("{version}", version).replace("{preset}", preset)
    }

    async fn get_json(uri: &str) -> (StatusCode, serde_json::Value) {
        let app = test_app(ROUTE);
        let response = app
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        (status, json)
    }

    #[tokio::test]
    async fn returns_tests_for_csaf_2_0_basic() {
        let (status, json) = get_json(&build_uri("2.0", "basic")).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["version"], "2.0");
        assert_eq!(json["preset"], "basic");

        let tests: Vec<String> = serde_json::from_value(json["tests"].clone()).unwrap();
        let expected: Vec<String> = Csaf2_0::tests_in_preset(Preset2_0::Basic)
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(tests, expected);
        assert!(tests.contains(&"schema".to_string()));
    }

    #[tokio::test]
    async fn returns_tests_for_csaf_2_0_extended() {
        let (status, json) = get_json(&build_uri("2.0", "extended")).await;

        assert_eq!(status, StatusCode::OK);

        let tests: Vec<String> = serde_json::from_value(json["tests"].clone()).unwrap();
        let expected: Vec<String> = Csaf2_0::tests_in_preset(Preset2_0::Extended)
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(tests, expected);
    }

    #[tokio::test]
    async fn returns_tests_for_csaf_2_0_full() {
        let (status, json) = get_json(&build_uri("2.0", "full")).await;

        assert_eq!(status, StatusCode::OK);

        let tests: Vec<String> = serde_json::from_value(json["tests"].clone()).unwrap();
        let expected: Vec<String> = Csaf2_0::tests_in_preset(Preset2_0::Full)
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(tests, expected);
    }

    #[tokio::test]
    async fn returns_tests_for_csaf_2_1_mandatory() {
        let (status, json) = get_json(&build_uri("2.1", "mandatory")).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["version"], "2.1");
        assert_eq!(json["preset"], "mandatory");

        let tests: Vec<String> = serde_json::from_value(json["tests"].clone()).unwrap();
        let expected: Vec<String> = Csaf2_1::tests_in_preset(Preset2_1::Mandatory)
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(tests, expected);
    }

    #[tokio::test]
    async fn returns_tests_for_csaf_2_1_full() {
        let (status, json) = get_json(&build_uri("2.1", "full")).await;

        assert_eq!(status, StatusCode::OK);

        let tests: Vec<String> = serde_json::from_value(json["tests"].clone()).unwrap();
        let expected: Vec<String> = Csaf2_1::tests_in_preset(Preset2_1::Full)
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(tests, expected);
    }

    #[tokio::test]
    async fn returns_tests_for_csaf_2_1_external_request_free() {
        let (status, json) = get_json(&build_uri("2.1", "external-request-free")).await;

        assert_eq!(status, StatusCode::OK);

        let tests: Vec<String> = serde_json::from_value(json["tests"].clone()).unwrap();
        let expected: Vec<String> = Csaf2_1::tests_in_preset(Preset2_1::ExternalRequestFree)
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(tests, expected);
    }

    #[tokio::test]
    async fn returns_400_for_invalid_version() {
        let (status, json) = get_json(&build_uri("3.0", "basic")).await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(json["error"].as_str().unwrap().contains("3.0"));
    }

    #[tokio::test]
    async fn returns_400_for_unknown_preset() {
        let (status, json) = get_json(&build_uri("2.0", "nonexistent")).await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(json["error"].as_str().unwrap().contains("nonexistent"));
    }

    #[tokio::test]
    async fn returns_400_for_preset_not_in_version() {
        // "mandatory" is a 2.1 preset, not available in 2.0
        let (status, json) = get_json(&build_uri("2.0", "mandatory")).await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(json["error"].as_str().unwrap().contains("mandatory"));
    }
}
