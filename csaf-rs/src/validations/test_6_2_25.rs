use crate::csaf_traits::{CsafTrait, VulnerabilityTrait};
use crate::helpers::CWE_ENTRIES;
use crate::validation::ValidationError;

fn create_disallowed_cwe_usage_error(
    cwe: &str,
    usage: &str,
    version: &str,
    i_r: usize,
    i_cwe: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Weakness '{cwe}' has usage '{usage}' in version '{version}', which is not allowed for vulnerability mapping."
        ),
        instance_path: format!("/vulnerabilities/{i_r}/cwes/{i_cwe}"),
    }
}

/// 6.2.25 Usage of CWE Not Allowed for Vulnerability Mapping
///
/// For each item in the CWE array it MUST be tested that the vulnerability mapping
/// is allowed. Currently, this includes the two usage states Allowed and Allowed-with-Review.
///
/// Note: The property Usage within the MappingNotesType was introduced in version 7.0 of the
/// CWE schema definition. As a consequence, this information might not be available before
/// CWE version 4.12.
pub fn test_6_2_25_usage_of_cwe_not_allowed_for_vulnerability_mapping(
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
                    // Revisit after #407
                    continue;
                };

                // `None` means the CWE version predates the Usage field, so we can skip.
                let Some(usage) = entry.usage.as_deref() else {
                    // Revisit after #407
                    continue;
                };

                // Generate an error if the usage field is not one of the two specified values.
                if usage != "Allowed" && usage != "Allowed-with-Review" {
                    errors.get_or_insert_default().push(create_disallowed_cwe_usage_error(
                        &cwe_item.id,
                        usage,
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
    ValidatorForTest6_2_25,
    test_6_2_25_usage_of_cwe_not_allowed_for_vulnerability_mapping
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_25() {
        let case_01_discouraged = Err(vec![create_disallowed_cwe_usage_error(
            "CWE-20",
            "Discouraged",
            "4.13",
            0,
            0,
        )]);

        let case_02_prohibited = Err(vec![create_disallowed_cwe_usage_error(
            "CWE-1187",
            "Prohibited",
            "4.13",
            0,
            0,
        )]);

        let case_03_multiple_cwes_with_discouraged = Err(vec![create_disallowed_cwe_usage_error(
            "CWE-20",
            "Discouraged",
            "4.13",
            0,
            1,
        )]);

        let case_04_multiple_vulns_cwe_with_discouraged = Err(vec![create_disallowed_cwe_usage_error(
            "CWE-74",
            "Discouraged",
            "4.13",
            2,
            1,
        )]);

        // Case 11: CWE-112 (Allowed)
        // Case 12: CWE-908 (Allowed)
        // Case 13: CWE-1287 (Allowed)
        // Case 14: multiple vulns, multiple cwes, all allowed

        TESTS_2_1.test_6_2_25.expect(
            case_01_discouraged,
            case_02_prohibited,
            case_03_multiple_cwes_with_discouraged,
            case_04_multiple_vulns_cwe_with_discouraged,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
