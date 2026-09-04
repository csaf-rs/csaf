use crate::csaf_traits::{CsafTrait, VulnerabilityIdTrait, VulnerabilityTrait};
use crate::validation::{TestFinding, TestFindingData};
use regex::Regex;
use std::sync::LazyLock;

static CVE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^CVE-[0-9]{4}-[0-9]{4,}$").unwrap());

fn create_cve_in_ids_error(id: &str, vuln_index: usize, id_index: usize) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!("Vulnerability ID text '{id}' matches CVE format"),
        instance_path: format!("/vulnerabilities/{vuln_index}/ids/{id_index}/text"),
    })
}

/// 6.2.17 CVE in field IDs
///
/// All `/vulnerabilities[]/ids[]` items must not match the CVE ID format in their `text` field.
pub fn test_6_2_17_cve_in_field_ids(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

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
    use crate::csaf2_0::testcases::ExpectedResults_6_2_17 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_17 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_17() {
        let id_with_cve_in_text_field = Err(vec![create_cve_in_ids_error("CVE-2021-44228", 0, 0)]);
        let multiple_ids_with_cve_in_text_field_fail_distinct = Err(vec![
            create_cve_in_ids_error("CVE-2021-44228", 0, 0),
            create_cve_in_ids_error("CVE-2021-44229", 0, 1),
        ]);
        let two_vulnerabilities_with_ids_with_cve_in_text_field = Err(vec![
            create_cve_in_ids_error("CVE-2021-44228", 0, 0),
            create_cve_in_ids_error("CVE-2021-44229", 1, 0),
        ]);
        let second_id_of_a_vulnerability_with_cve_in_text_field =
            Err(vec![create_cve_in_ids_error("CVE-2021-44228", 0, 1)]);
        let second_vulnerability_with_an_id_with_cve_in_text_field =
            Err(vec![create_cve_in_ids_error("CVE-2021-44228", 1, 0)]);

        TESTS_2_0.test_6_2_17.expect(ExpectedResults_2_0 {
            case_01: id_with_cve_in_text_field.clone(),
            // Valid test with one vulnerability containing an id with a correctly set text field
            case_11: Ok(()),

            case_s01: multiple_ids_with_cve_in_text_field_fail_distinct.clone(),
            case_s02: two_vulnerabilities_with_ids_with_cve_in_text_field.clone(),
            case_s03: second_id_of_a_vulnerability_with_cve_in_text_field.clone(),
            case_s04: second_vulnerability_with_an_id_with_cve_in_text_field.clone(),
            // Valid test for two vulnerabilities each with two ids with correctly set text fields
            case_s11: Ok(()),
        });
        TESTS_2_1.test_6_2_17.expect(ExpectedResults_2_1 {
            case_01: id_with_cve_in_text_field,
            // Valid test with one vulnerability containing an id with a correctly set text field
            case_11: Ok(()),

            case_s01: multiple_ids_with_cve_in_text_field_fail_distinct,
            case_s02: two_vulnerabilities_with_ids_with_cve_in_text_field,
            case_s03: second_id_of_a_vulnerability_with_cve_in_text_field,
            case_s04: second_vulnerability_with_an_id_with_cve_in_text_field,
            // Valid test for two vulnerabilities each with two ids with correctly set text fields
            case_s11: Ok(()),
        });
    }
}
