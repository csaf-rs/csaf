use std::collections::BTreeMap;

use crate::csaf_traits::CsafTrait;
use crate::validation::{TestFinding, TestFindingData};

fn generate_multiple_document_local_vuln_id_error(id: &str, path: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Multiple definition of document-local vulnerability ID `{id}`"),
        instance_path: path.to_owned(),
    })
}

/// 6.1.65.2 Multiple Definition of Document-local Vuln ID
///
/// For each element of type `$['$defs'].dl_vuln_id_t` in vulnerabilities
/// (`$.vulnerabilities[*].dl_vuln_id`) it SHALL be tested that the `dl_vuln_id` was not
/// already defined within the same document.
///
/// If a multiple definition of a document-local vulnerability ID is found, we generate
/// an error for each occurrence of the duplicate ID.
pub fn test_6_1_65_2_multiple_definition_of_document_local_vuln_id(
    doc: &impl CsafTrait,
) -> Result<(), Vec<TestFinding>> {
    // generate a map of document-local vulnerability ids to the paths where they are defined
    let mut definitions: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (id, path) in doc.get_all_document_local_vuln_ids_with_path() {
        definitions.entry(id).or_default().push(path);
    }

    // generate an error for every document-local vulnerability id that is defined more than once
    let mut errors: Option<Vec<TestFinding>> = None;
    for (id, paths) in definitions {
        if paths.len() > 1 {
            for path in paths {
                errors
                    .get_or_insert_default()
                    .push(generate_multiple_document_local_vuln_id_error(&id, &path));
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_65_2,
    test_6_1_65_2_multiple_definition_of_document_local_vuln_id
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_65_2 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_generate_multiple_document_local_vuln_id_error() {
        assert_eq!(
            generate_multiple_document_local_vuln_id_error("VULN-1", "/vulnerabilities/0/dl_vuln_id"),
            TestFinding::Error(TestFindingData {
                message: "Multiple definition of document-local vulnerability ID `VULN-1`".to_owned(),
                instance_path: "/vulnerabilities/0/dl_vuln_id".to_owned(),
            })
        );
    }

    #[test]
    fn test_test_6_1_65_2() {
        let two_vulns_same_dl_vuln_id = Err(vec![
            generate_multiple_document_local_vuln_id_error("VULN-1", "/vulnerabilities/0/dl_vuln_id"),
            generate_multiple_document_local_vuln_id_error("VULN-1", "/vulnerabilities/1/dl_vuln_id"),
        ]);

        // 5 vulns, 3 duplicates (VULN-3 x2, VULN-4 x3)
        // vulns 2 and 3 have ids with system name / text that looks like document local vuln ids, which are irrelevant
        let mutliple_vulns_multiple_duplicates = Err(vec![
            generate_multiple_document_local_vuln_id_error("VULN-3", "/vulnerabilities/0/dl_vuln_id"),
            generate_multiple_document_local_vuln_id_error("VULN-3", "/vulnerabilities/2/dl_vuln_id"),
            generate_multiple_document_local_vuln_id_error("VULN-4", "/vulnerabilities/1/dl_vuln_id"),
            generate_multiple_document_local_vuln_id_error("VULN-4", "/vulnerabilities/3/dl_vuln_id"),
            generate_multiple_document_local_vuln_id_error("VULN-4", "/vulnerabilities/4/dl_vuln_id"),
        ]);

        // Case 11: 2 vulns, different document local vuln ids
        // Case 12: 5 vulns, one without a document-local ID, the others with distinct `dl_vuln_id` values
        // Case S11: no vulnerabilities present
        // Case S12: 3 vulnerabilities, none with a document-local ID
        TESTS_2_1.test_6_1_65_2.expect(ExpectedResults {
            case_01: two_vulns_same_dl_vuln_id,
            case_02: mutliple_vulns_multiple_duplicates,
            case_11: Ok(()),
            case_12: Ok(()),
            case_s11: Ok(()),
            case_s12: Ok(()),
        });
    }
}
