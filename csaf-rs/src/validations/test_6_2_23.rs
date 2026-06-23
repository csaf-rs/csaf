use crate::csaf_traits::{CsafTrait, VulnerabilityTrait};
use crate::helpers::CWE_ENTRIES;
use crate::validation::ValidationError;

fn create_deprecated_cwe_error(cwe: &str, version: &str, path: &str) -> ValidationError {
    ValidationError {
        message: format!("Weakness '{cwe}' is deprecated in version {version}."),
        instance_path: format!("{path}/name"),
    }
}

/// 6.2.23 Usage of Deprecated CWE
///
/// For each item in the CWE array it MUST be tested that the CWE is not deprecated in the given version.
pub fn test_6_2_23_usage_of_deprecated_cwe(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let vulnerabilities = doc.get_vulnerabilities();
    let mut errors: Vec<ValidationError> = Vec::new();

    for (i_r, vulnerability) in vulnerabilities.iter().enumerate() {
        if let Some(cwes) = vulnerability.get_cwe() {
            for (i_cwe, cwe_item) in cwes.iter().enumerate() {
                // Determine which CWE CSV version to check: use the version
                // declared on the CWE item
                let Some(version) = cwe_item.version.as_deref() else {
                    // version missing — Document should already have been rejected
                    continue;
                };

                // First check whether the CWE entry provided in the vulnerability
                // itself marks the weakness as deprecated (name starts with
                // "DEPRECATED:"). If not, fall back to checking the CWE name
                // found in the CSV assets for the specified CWE version.
                let mut is_deprecated = cwe_item.name.starts_with("DEPRECATED:");

                if !is_deprecated {
                    // We can also just run this part, and only look into the cwe file
                    if let Some((_date, map)) = CWE_ENTRIES.get(version)
                        && let Some(name) = map.get(&cwe_item.id)
                            && name.starts_with("DEPRECATED:") {
                                is_deprecated = true;


                    }
                }

                if is_deprecated {
                    errors.push(create_deprecated_cwe_error(
                        &cwe_item.id,
                        version,
                        format!("/vulnerabilities/{i_r}/cwes/{i_cwe}").as_str(),
                    ));
                }
            }
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_2_23, test_6_2_23_usage_of_deprecated_cwe);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_23() {
        // Expect failing cases (01,02,03) and passing cases (11,12,13) as defined in testcases.json
        TESTS_2_1.test_6_2_23.expect(
            // case 01: CWE-596 deprecated in 4.13
            Err(vec![create_deprecated_cwe_error(
                "CWE-596",
                "4.13",
                "/vulnerabilities/0/cwes/0",
            )]),
            // case 02: first CWE is CWE-1324 deprecated in 4.10
            Err(vec![create_deprecated_cwe_error(
                "CWE-1324",
                "4.10",
                "/vulnerabilities/0/cwes/0",
            )]),
            // case 03: the third vulnerability contains CWE-365 deprecated in 4.13
            Err(vec![create_deprecated_cwe_error(
                "CWE-365",
                "4.13",
                "/vulnerabilities/2/cwes/0",
            )]),
            // case 11: valid
            Ok(()),
            // case 12: valid
            Ok(()),
            // case 13: valid
            Ok(()),
        );
    }
}
