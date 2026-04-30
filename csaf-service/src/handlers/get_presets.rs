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
        Err(err) => Err(error_response(StatusCode::BAD_REQUEST, err)),
        Ok(valid_version) => Ok(Json(PresetsResponse {
            version,
            presets: presets_for_version(&valid_version),
        })),
    }
}

/// Returns the list of known preset names for a CSAF version.
pub(crate) fn presets_for_version(version: &CsafVersion) -> Vec<String> {
    match version {
        CsafVersion::X20 => Csaf2_0::get_presets().into_iter().map(|p| p.to_string()).collect(),
        CsafVersion::X21 => Csaf2_1::get_presets().into_iter().map(|p| p.to_string()).collect(),
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
        Router::new().route(route, get(get_presets))
    }

    const GET_PRESET_ROUTE: &str = "/api/v1/csaf/{version}/presets";
    fn build_uri(version: &str) -> String {
        GET_PRESET_ROUTE.replace("{version}", version)
    }

    async fn get_json(uri: &str) -> (StatusCode, serde_json::Value) {
        let app = test_app(GET_PRESET_ROUTE);
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
    async fn returns_presets_for_csaf_2_0() {
        let (status, json) = get_json(build_uri("2.0").as_str()).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["version"], "2.0");

        let presets: Vec<String> = serde_json::from_value(json["presets"].clone()).unwrap();
        assert_eq!(presets, vec!["basic", "extended", "full"]);
    }

    #[tokio::test]
    async fn returns_presets_for_csaf_2_1() {
        let (status, json) = get_json(build_uri("2.1").as_str()).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["version"], "2.1");

        let presets: Vec<String> = serde_json::from_value(json["presets"].clone()).unwrap();
        assert_eq!(
            presets,
            vec![
                "mandatory",
                "recommended",
                "informative",
                "schema",
                "basic",
                "extended",
                "full",
                "external-request-free",
                "consistent-revision-history",
                "consistent-date-times",
                "ssvc",
            ]
        );
    }

    #[tokio::test]
    async fn returns_400_for_invalid_version() {
        let (status, json) = get_json(build_uri("3.0").as_str()).await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(json["error"].as_str().unwrap().contains("3.0"));
    }

    #[tokio::test]
    async fn returns_400_for_non_numeric_version() {
        let (status, json) = get_json(build_uri("abc").as_str()).await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(json["error"].is_string());
    }
}
