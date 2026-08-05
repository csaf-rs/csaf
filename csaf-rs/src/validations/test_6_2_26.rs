use crate::csaf_traits::{CsafTrait, VulnerabilityTrait};
use crate::helpers::CWE_ENTRIES;
use crate::validation::ValidationError;

fn create_allowed_with_review_error(cwe: &str, version: &str, i_r: usize, i_cwe: usize) -> ValidationError {
    ValidationError {
        message: format!(
            "Weakness '{cwe}' has usage 'Allowed-with-Review' in version {version}, which requires a thorough review."
        ),
        instance_path: format!("/vulnerabilities/{i_r}/cwes/{i_cwe}"),
    }
}

/// 6.2.26 Usage of CWE Allowed with Review for Vulnerability Mapping
///
/// For each item in the CWE array it MUST be tested that the vulnerability mapping
/// is allowed without review.
///
/// Note: The property Usage within the MappingNotesType was introduced in version 7.0 of the
/// CWE schema definition. As a consequence, this information might not be available before
/// CWE version 4.12.
pub fn test_6_2_26_usage_of_cwe_allowed_with_review_for_vulnerability_mapping(
    doc: &impl CsafTrait,
) -> Result<(), Vec<ValidationError>> {
    let vulnerabilities = doc.get_vulnerabilities();
    let mut errors: Option<Vec<ValidationError>> = None;

    for (i_r, vulnerability) in vulnerabilities.iter().enumerate() {
        if let Some(cwes) = vulnerability.get_cwes() {
            for (i_cwe, cwe_item) in cwes.iter().enumerate() {
                // This check is semi-redundant: Version is guaranteed to be here on CSAF 2.1,
                // while this test does not exist on CSAF 2.0. The continue path will only
                // happen when running CSAF 2.0 docs against CSAF 2.1 tests.
                let Some(version) = cwe_item.version.as_deref() else {
                    continue;
                };

                // Skip this entry if the CWE version or CWE ID are unknown, this will be reported
                // by 6.1.11.
                let Some(entry) = CWE_ENTRIES.get(version).and_then(|v| v.entries.get(&cwe_item.id)) else {
                    continue;
                };

                // `None` means the CWE version predates the Usage field, so we can skip.
                let Some(usage) = entry.usage.as_deref() else {
                    // Revisit after #407
                    continue;
                };

                // Generate an error if the usage field is "Allowed-with-Review"
                if usage == "Allowed-with-Review" {
                    errors.get_or_insert_default().push(create_allowed_with_review_error(
                        &cwe_item.id,
                        version,
                        i_r,
                        i_cwe,
                    ));
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_26,
    test_6_2_26_usage_of_cwe_allowed_with_review_for_vulnerability_mapping
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_26 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_26() {
        let case_01_allowed_with_review = Err(vec![create_allowed_with_review_error("CWE-1023", "4.13", 0, 0)]);

        let case_02_multiple_cwes = Err(vec![create_allowed_with_review_error("CWE-1038", "4.13", 0, 1)]);

        let case_03_multiple_vulns = Err(vec![create_allowed_with_review_error("CWE-1039", "4.13", 3, 0)]);

        TESTS_2_1.test_6_2_26.expect(ExpectedResults {
            case_01: case_01_allowed_with_review,
            case_02: case_02_multiple_cwes,
            case_03: case_03_multiple_vulns,
            case_11: Ok(()), // CWE-184 (Allowed)
            case_12: Ok(()), // CWE-14 + CWE-733 (both Allowed)
            case_13: Ok(()), // all Allowed
        });
    }
}
