use crate::csaf::consts::chars::is_invisible_char;
use crate::csaf_traits::{CsafTrait, VulnerabilityIdTrait, VulnerabilityTrait};
use crate::schema::csaf2_1::schema::Cve;
use crate::validation::{TestFinding, TestFindingData};

fn create_cve_in_ids_warning(id_text_with_cve: &str, vuln_index: usize, id_index: usize) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!(
            "`{}` looks like a CVE ID; use the `cve` property instead of `ids`",
            id_text_with_cve
        ),
        instance_path: format!("/vulnerabilities/{vuln_index}/ids/{id_index}/text"),
    })
}

/// 6.2.17 CVE in field IDs
///
/// For each item in `$.vulnerabilities[*].ids` it SHALL be tested that it is not a CVE ID.
///
/// To implement this test it is deemed sufficient to check after invisible and white space characters
/// have been removed from the value of the property text whether it matches the regex `^CVE-[0-9]{4}-[0-9]{4,}$`.
///
/// The informative comment on how to implement this test was added with CSAF 2.1. As this is a) an informative comment and b) just covers
/// an edge case that was not considered in CSAF 2.0, we apply the same test logic to both CSAF 2.0 and CSAF 2.1 documents.
pub fn test_6_2_17_cve_in_field_ids(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;

    for (v_i, vuln) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(ids) = vuln.get_ids() {
            for (i_i, id) in ids.iter().enumerate() {
                let full_text = id.get_text();
                // filter whitespace and invisible chars
                let filtered_text: String = full_text
                    .chars()
                    .filter(|c| !c.is_whitespace() && !is_invisible_char(*c))
                    .collect();
                // try to parse the filtered text as CVE, using the same struct used for the "actual" CVE fields in CSAF 2.0 and 2.1
                if filtered_text.parse::<Cve>().is_ok() {
                    errors
                        .get_or_insert_default()
                        .push(create_cve_in_ids_warning(full_text, v_i, i_i));
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
        let id_with_cve_in_text_field = Err(vec![create_cve_in_ids_warning("CVE-2021-44228", 0, 0)]);
        let id_with_cve_in_text_field_with_filter_chars = Err(vec![create_cve_in_ids_warning("CVE-2021-44228", 0, 0)]);

        let multiple_ids_with_cve_in_text_field_fail_distinct = Err(vec![
            create_cve_in_ids_warning("CVE-2021-44228", 0, 0),
            create_cve_in_ids_warning("CVE-2021-44229", 0, 1),
        ]);
        let two_vulnerabilities_with_ids_with_cve_in_text_field = Err(vec![
            create_cve_in_ids_warning("CVE-2021-44228", 0, 0),
            create_cve_in_ids_warning("CVE-2021-44229", 1, 0),
        ]);
        let second_id_of_a_vulnerability_with_cve_in_text_field =
            Err(vec![create_cve_in_ids_warning("CVE-2021-44228", 0, 1)]);
        let second_vulnerability_with_an_id_with_cve_in_text_field =
            Err(vec![create_cve_in_ids_warning("CVE-2021-44228", 1, 0)]);

        // Case 11: Valid test with one vulnerability containing an id with a correctly set text field
        // Case S11: Valid test for two vulnerabilities each with two ids with correctly set text fields
        // Case 12: "cve" is lowercase, system_name in test case specifies this might be a company-specific registry

        TESTS_2_0.test_6_2_17.expect(ExpectedResults_2_0 {
            case_01: id_with_cve_in_text_field.clone(),
            case_s01: multiple_ids_with_cve_in_text_field_fail_distinct.clone(),
            case_s02: two_vulnerabilities_with_ids_with_cve_in_text_field.clone(),
            case_s03: second_id_of_a_vulnerability_with_cve_in_text_field.clone(),
            case_s04: second_vulnerability_with_an_id_with_cve_in_text_field.clone(),
            case_11: Ok(()),
            case_s11: Ok(()),
        });
        TESTS_2_1.test_6_2_17.expect(ExpectedResults_2_1 {
            case_01: id_with_cve_in_text_field,
            case_02: id_with_cve_in_text_field_with_filter_chars,

            case_s01: multiple_ids_with_cve_in_text_field_fail_distinct,
            case_s02: two_vulnerabilities_with_ids_with_cve_in_text_field,
            case_s03: second_id_of_a_vulnerability_with_cve_in_text_field,
            case_s04: second_vulnerability_with_an_id_with_cve_in_text_field,
            case_11: Ok(()),
            case_12: Ok(()),
            case_s11: Ok(()),
        });
    }
}
