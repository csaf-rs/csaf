use crate::csaf_traits::{CsafTrait, VulnerabilityIdTrait, VulnerabilityTrait};
use crate::validation::ValidationError;
use regex::Regex;
use std::sync::LazyLock;

static CVE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^CVE-[0-9]{4}-[0-9]{4,}$").unwrap());

fn create_cve_in_ids_error(id: &str, vuln_index: usize, id_index: usize) -> ValidationError {
    ValidationError {
        message: format!("Vulnerability ID text '{id}' matches CVE format"),
        instance_path: format!("/vulnerabilities/{vuln_index}/ids/{id_index}/text"),
    }
}

/// 6.2.17 CVE in field IDs
///
/// All `/vulnerabilities[]/ids[]` items must not match the CVE ID format in their `text` field.
pub fn test_6_2_17_cve_in_field_ids(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    for (v_i, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(ids) = vuln.get_ids() {
            for (i_i, id) in ids.iter().enumerate() {
                if CVE_REGEX.is_match(id.get_text()) {
                    errors
                        .get_or_insert_default()
                        .push(create_cve_in_ids_error(id.get_text(), v_i, i_i));
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(ValidatorForTest6_2_17, test_6_2_17_cve_in_field_ids);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_17() {
        let case_01 = Err(vec![create_cve_in_ids_error("CVE-2021-44228", 0, 0)]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_2_17.expect(case_01.clone(), Ok(()));
        TESTS_2_1.test_6_2_17.expect(case_01, Ok(()));
    }
}
