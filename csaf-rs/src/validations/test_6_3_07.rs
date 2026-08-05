#[cfg(feature = "external-connections")]
use std::collections::HashMap;

use crate::csaf_traits::CsafTrait;
#[cfg(feature = "external-connections")]
use crate::csaf_traits::{DocumentTrait, ReferenceTrait, VulnerabilityTrait};
use crate::validation::ValidationError;
#[cfg(feature = "external-connections")]
use crate::validations::utils::external_connections::{UrlResolutionFailure, check_url_resolution, defang_url};

/// 6.3.7 Use of Self Referencing URLs Failing to Resolve
///
/// When the `external-connections` feature is not enabled, this validation is skipped and returns Ok(()).
#[cfg(not(feature = "external-connections"))]
pub fn test_6_3_7_use_of_self_referencing_urls_failing_to_resolve(
    _doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {
    // TODO: #407 this would be another use-case?
    Ok(())
}

#[cfg(feature = "external-connections")]
fn create_url_resolution_error(url: &str, failure: UrlResolutionFailure, instance_path: &str) -> ValidationError {
    let message = match failure {
        UrlResolutionFailure::FailedWithStatusCode(code) => format!(
            "The URL '{}' does not resolve with HTTP status code of less than 400. Got status code: {}",
            defang_url(url),
            code
        ),
        UrlResolutionFailure::FailedWithError(_) => format!(
            "The URL '{}' does not resolve. It may be invalid or the server may be unreachable.",
            defang_url(url)
        ),
    };
    ValidationError {
        message,
        instance_path: instance_path.to_string(),
    }
}

/// 6.3.7 Use of Self Referencing URLs Failing to Resolve
///
/// For each item in an array of type references_t with the category self it MUST be tested that
/// the URL referenced resolves with an HTTP status code less than 400.
///
/// This function is only available when the `external-connections` feature is enabled.
/// It will attempt to make HEAD calls to all self-referencing URLs.
#[cfg(feature = "external-connections")]
pub fn test_6_3_7_use_of_self_referencing_urls_failing_to_resolve(
    doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {
    // Collect all self-referencing URLs first, keyed by URL, so that each distinct URL is only
    // resolved once.
    let mut urls_to_instance_paths: Option<HashMap<&str, Vec<String>>> = None;

    // document self-reference urls
    if let Some(self_refs) = doc.get_document().get_self_references() {
        for (r_i, reference) in self_refs {
            urls_to_instance_paths
                .get_or_insert_default()
                .entry(reference.get_url())
                .or_default()
                .push(format!("/document/references/{r_i}"));
        }
    }

    // vulnerabilities self-reference urls
    for (v_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(self_refs) = vulnerability.get_self_references() {
            for (r_i, reference) in self_refs {
                urls_to_instance_paths
                    .get_or_insert_default()
                    .entry(reference.get_url())
                    .or_default()
                    .push(format!("/vulnerabilities/{v_i}/references/{r_i}"));
            }
        }
    }

    // no self-reference urls
    let Some(urls_to_instance_paths) = urls_to_instance_paths else {
        return Ok(());
    };

    let mut errors: Option<Vec<ValidationError>> = None;

    for (url, instance_paths) in urls_to_instance_paths {
        if let Err(failure) = check_url_resolution(url, |status_code| status_code < 400) {
            for instance_path in instance_paths {
                errors
                    .get_or_insert_default()
                    .push(create_url_resolution_error(url, failure.clone(), &instance_path));
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    ValidatorForTest6_3_7,
    test_6_3_7_use_of_self_referencing_urls_failing_to_resolve
);

#[cfg(all(test, feature = "external-connections"))]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use crate::validations::utils::external_connections::url_mock::{
        MockResponse, clear_mock_responses, set_mock_response,
    };

    #[test]
    fn test_test_6_3_7() {
        // mock the urls contained in the test case jsn files
        set_mock_response("https://example.invalid", MockResponse::ConnectionFailure);
        set_mock_response("https://example.net", MockResponse::StatusCode(200));

        // Case 01: Failing example with https://example.invalid
        let case_01_invalid_url = Err(vec![create_url_resolution_error(
            "https://example.invalid",
            UrlResolutionFailure::FailedWithError("mocked connection failure".to_string()),
            "/document/references/0",
        )]);

        // Case 11: Valid example with resolvable URL (https://example.net)
        TESTS_2_0.test_6_3_7.expect(case_01_invalid_url.clone(), Ok(()));
        TESTS_2_1.test_6_3_7.expect(case_01_invalid_url, Ok(()));

        clear_mock_responses();
    }
}
