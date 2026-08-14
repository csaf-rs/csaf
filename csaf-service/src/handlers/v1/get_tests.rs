use axum::extract::Query;
use axum::{Json, http::StatusCode};
use csaf::csaf_traits::CsafVersion;
use csaf::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework as Csaf2_0;
use csaf::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework as Csaf2_1;
use csaf::validation::Validatable;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::handlers::v1::errors::{ErrorResponse, error_response};

#[derive(Debug, Deserialize)]
pub(crate) struct LegacyTestsQuery {
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq)]
pub(crate) struct TestInPreset {
    pub name: String,
    pub preset: String,
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
pub(crate) async fn get_tests(
    Query(query): Query<LegacyTestsQuery>,
) -> Result<Json<Vec<TestInPreset>>, (StatusCode, Json<ErrorResponse>)> {
    let version = CsafVersion::try_from(query.version.clone().unwrap_or_else(|| "2.0".to_string()))
        .map_err(|e| error_response(StatusCode::NOT_FOUND, "INVALID_VERSION", e))?;
    Ok(Json(tests_for_version(&version)))
}

/// Returns the list of known preset names for a CSAF version.
fn tests_for_version(version: &CsafVersion) -> Vec<TestInPreset> {
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
