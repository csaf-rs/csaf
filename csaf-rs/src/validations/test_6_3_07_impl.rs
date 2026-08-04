use crate::csaf_traits::{CsafTrait, DocumentTrait, ReferenceTrait, VulnerabilityTrait};
use crate::helpers::{defang_url, get_status_code};
use crate::validation::ValidationError;

fn create_url_resolution_error(url: &str, status_code: Option<u16>, instance_path: &str) -> ValidationError {
    let message = match status_code {
        Some(code) => format!(
            "The URL '{}' does not resolve with HTTP status code of less than 400. Got status code: {}",
            defang_url(url),
            code
        ),
        None => format!(
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
/// It will attempt to make HEAD calls to all self reference urls.
pub fn test_6_3_7_use_of_self_referencing_urls_failing_to_resolve(
    doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    // Helper closure to check if a URL resolves with HTTP status code < 400
    // Returns Ok(()) if status is < 400, Err with status code otherwise
    let check_url_resolution = |url: &str| -> Result<(), Option<u16>> {
        let status_code = get_status_code(url);
        if status_code == 0 {
            Err(None)
        } else if status_code < 400 {
            Ok(())
        } else {
            Err(Some(status_code))
        }
    };

    // Check document references
    if let Some(self_refs) = doc.get_document().get_self_references() {
        for (r_i, reference) in self_refs {
            let url = reference.get_url();
            if let Err(status) = check_url_resolution(url) {
                errors.get_or_insert_default().push(create_url_resolution_error(
                    url,
                    status,
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
                if let Err(status) = check_url_resolution(url) {
                    errors.get_or_insert_default().push(create_url_resolution_error(
                        url,
                        status,
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
            None,
            "/document/references/0",
        )]);

        // Case 11: Valid example with resolvable URL (https://example.net)

        TESTS_2_0.test_6_3_7.expect(case_01_invalid_url.clone(), Ok(()));
        TESTS_2_1.test_6_3_7.expect(case_01_invalid_url, Ok(()));
    }
}
