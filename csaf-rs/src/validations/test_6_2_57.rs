use crate::csaf_traits::{CsafTrait};
use crate::validation::{TestFinding, TestFindingData};

fn create_unused_document_local_vuln_id_warning(id: &str, path: &str) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!("Document-local vulnerability ID `{id}` is not referenced in the document"),
        instance_path: path.to_owned(),
    })
}

/// 6.2.57 Unused Definition of Document-local Vuln ID
///
/// For each element of type `$['$defs'].dl_vuln_id_t` in vulnerabilities (`$.vulnerabilities[*].dl_vuln_id`)
/// it SHALL be tested that the `dl_vuln_id` is referenced somewhere within the same document.
pub fn test_6_2_57_unused_definition_of_document_local_vuln_id(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    // collect all document-local vulnerability IDs that are referenced in the document#
    let referenced_doc_local_vuln_ids = doc.get_all_document_local_vuln_id_references();

    let mut findings: Option<Vec<TestFinding>> = None;
    // for each document-local vulnerability ID defined
    for (defined_doc_local_vuln_id, defined_doc_local_vuln_id_path) in doc.get_all_document_local_vuln_ids_with_path() {
        // check if it is referenced in the document, if not, add a warning
        if !referenced_doc_local_vuln_ids.contains(&defined_doc_local_vuln_id) {
            findings
                .get_or_insert_default()
                .push(create_unused_document_local_vuln_id_warning(
                    &defined_doc_local_vuln_id,
                    &defined_doc_local_vuln_id_path,
                ));
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_57,
    test_6_2_57_unused_definition_of_document_local_vuln_id
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_57 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_57() {
        TESTS_2_1.test_6_2_57.expect(ExpectedResults {
            // TODO: Add testcase with no involvements present at all
            case_01: Err(vec![create_unused_document_local_vuln_id_warning(
                "VULN-1",
                "/vulnerabilities/0/dl_vuln_id",
            )]),
            case_02: Err(vec![
                create_unused_document_local_vuln_id_warning("VULN-1", "/vulnerabilities/0/dl_vuln_id"),
                create_unused_document_local_vuln_id_warning("VULN-2", "/vulnerabilities/1/dl_vuln_id"),
            ]),
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
