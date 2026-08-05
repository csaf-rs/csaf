use crate::csaf_traits::{CsafTrait, DocumentTrait, ReferenceTrait, VulnerabilityTrait};
use crate::validation::ValidationError;
use crate::validations::utils::external_connections::{check_url_resolution, defang_url, UrlResolutionFailure};

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
pub fn test_6_3_7_use_of_self_referencing_urls_failing_to_resolve(
    doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    let resolve_success = |url: &str| check_url_resolution(url, |status_code| status_code < 400);

    // Check document references
    if let Some(self_refs) = doc.get_document().get_self_references() {
        for (r_i, reference) in self_refs {
            let url = reference.get_url();
            if let Err(failure) = resolve_success(url) {
                errors.get_or_insert_default().push(create_url_resolution_error(
                    url,
                    failure,
                    &format!("/document/references/{r_i}"),
                ));
            }
        }
    }

    // Check vulnerability references
    for (v_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(self_refs) = vulnerability.get_self_references() {
            for (r_i, reference) in self_refs {
                let url = reference.get_url();
                if let Err(failure) = resolve_success(url) {
                    errors.get_or_insert_default().push(create_url_resolution_error(
                        url,
                        failure,
                        &format!("/vulnerabilities/{v_i}/references/{r_i}"),
                    ));
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    ValidatorForTest6_3_7,
    test_6_3_7_use_of_self_referencing_urls_failing_to_resolve
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_7() {
        // Case 01: Failing example with https://example.invalid
        let case_01_invalid_url = Err(vec![create_url_resolution_error(
            "https://example.invalid",
            UrlResolutionFailure::FailedWithError(String::new()),
            "/document/references/0",
        )]);

        // Case 11: Valid example with resolvable URL (https://example.net)

        TESTS_2_0.test_6_3_7.expect(case_01_invalid_url.clone(), Ok(()));
        TESTS_2_1.test_6_3_7.expect(case_01_invalid_url, Ok(()));
    }
}
