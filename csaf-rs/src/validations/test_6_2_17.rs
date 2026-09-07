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
        let case_01 = Err(vec![create_cve_in_ids_error("CVE-2021-44228", 0, 0)]);
        let case_s01 = Err(vec![
            create_cve_in_ids_error("CVE-2021-44228", 0, 0),
            create_cve_in_ids_error("CVE-2021-44229", 0, 1),
        ]);
        let case_s02 = Err(vec![
            create_cve_in_ids_error("CVE-2021-44228", 0, 0),
            create_cve_in_ids_error("CVE-2021-44229", 1, 0),
        ]);
        let case_s03 = Err(vec![create_cve_in_ids_error("CVE-2021-44228", 0, 1)]);
        let case_s04 = Err(vec![create_cve_in_ids_error("CVE-2021-44228", 1, 0)]);

        TESTS_2_0.test_6_2_17.expect(ExpectedResults_2_0 {
            case_01: case_01.clone(),
            case_11: Ok(()),
        });
        TESTS_2_1.test_6_2_17.expect(ExpectedResults_2_1 {
            case_01,
            case_11: Ok(()),

            // Failing test for two ids for the same vulnerability with CVEs in the text field
            case_s01,
            // Failing test for two vulnerability each containing one id with a CVE in the text field
            case_s02,
            // Failing test for two ids for the same vulnerability with the second having a CVE in the text field
            case_s03,
            // Failing test for two vulnerabilities with the second having a CVE in its id's text field
            case_s04,
            // Valid test for two vulnerabilities each with two ids with correctly set text fields
            case_s11: Ok(()),
        });
    }
}
