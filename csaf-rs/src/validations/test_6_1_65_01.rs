use crate::csaf_traits::CsafTrait;
use crate::validation::{TestFinding, TestFindingData};

fn generate_undefined_document_local_vuln_id_error(id: &str, path: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Missing definition of document-local vulnerability ID `{id}`"),
        instance_path: path.to_owned(),
    })
}

/// 6.1.65.1 Missing Definition of Document-local Vuln ID
///
/// For each element of type `$['$defs'].dl_vuln_id_t` which is not inside a vulnerability
/// (`$.vulnerabilities[*].dl_vuln_id`) and therefore references a vulnerability it
/// SHALL be tested that the vulnerability element with the matching `dl_vuln_id` exists.
pub fn test_6_1_65_1_missing_definition_of_document_local_vuln_id(
    doc: &impl CsafTrait,
) -> Result<(), Vec<TestFinding>> {
    let defined_doc_local_vuln_ids = doc.get_all_document_local_vuln_ids();

    let mut errors: Option<Vec<TestFinding>> = None;
    for (doc_local_vuln_id, doc_local_vuln_id_path) in doc.get_all_document_local_vuln_id_references_with_path() {
        if !defined_doc_local_vuln_ids.contains(&doc_local_vuln_id) {
            errors
                .get_or_insert_default()
                .push(generate_undefined_document_local_vuln_id_error(
                    &doc_local_vuln_id,
                    &doc_local_vuln_id_path,
                ));
        }
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_65_1,
    test_6_1_65_1_missing_definition_of_document_local_vuln_id
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_65_1 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_65_1() {
        // TODO: Evaluate test coverage
        // TODO: Document test cases properly

        let case_01 = Err(vec![
            generate_undefined_document_local_vuln_id_error("VULN-1", "/document/involvement/actions/0/dl_vuln_ids/0"),
            generate_undefined_document_local_vuln_id_error("VULN-1", "/document/involvement/actions/1/dl_vuln_ids/0"),
        ]);

        let case_02 = Err(vec![
            generate_undefined_document_local_vuln_id_error("VULN-1", "/document/involvement/actions/0/dl_vuln_ids/0"),
            generate_undefined_document_local_vuln_id_error("VULN-2", "/document/involvement/actions/0/dl_vuln_ids/1"),
            generate_undefined_document_local_vuln_id_error("VULN-3", "/document/involvement/actions/0/dl_vuln_ids/2"),
            generate_undefined_document_local_vuln_id_error("VULN-4", "/document/involvement/actions/0/dl_vuln_ids/3"),
            generate_undefined_document_local_vuln_id_error("VULN-11", "/document/involvement/actions/1/dl_vuln_ids/0"),
            generate_undefined_document_local_vuln_id_error("VULN-12", "/document/involvement/actions/1/dl_vuln_ids/1"),
            generate_undefined_document_local_vuln_id_error("VULN-13", "/document/involvement/actions/1/dl_vuln_ids/2"),
            generate_undefined_document_local_vuln_id_error("VULN-14", "/document/involvement/actions/1/dl_vuln_ids/3"),
        ]);

        TESTS_2_1.test_6_1_65_1.expect(ExpectedResults {
            case_01,
            case_02,
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
