use std::collections::HashMap;

use crate::csaf_traits::CsafTrait;
use crate::validation::{TestFinding, TestFindingData};

fn generate_multiple_document_local_vuln_id_error(id: &str, path: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Duplicate definition for document-local vuln ID `{id}"),
        instance_path: path.to_owned(),
    })
}

/// 6.1.65.2 Multiple Definition of Document-local Vuln ID
///
/// For each element of type `$['$defs'].dl_vuln_id_t` in vulnerabilities
/// (`$.vulnerabilities[*].dl_vuln_id`) it SHALL be tested that the `dl_vuln_id` was not
/// already defined within the same document.
pub fn test_6_1_65_2_multiple_definition_of_document_local_vuln_id(
    doc: &impl CsafTrait,
) -> Result<(), Vec<TestFinding>> {
    let mut definitions: HashMap<String, Vec<String>> = HashMap::new();
    for (id, path) in doc.get_all_document_local_vuln_id_definitions() {
        definitions.entry(id).or_default().push(path);
    }

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
    fn test_test_6_1_65_2() {
        // TODO: Evaluate test coverage
        // TODO: Document test cases properly

        let case_01 = Err(vec![
            generate_multiple_document_local_vuln_id_error("VULN-1", "/vulnerabilities/0/dl_vuln_id"),
            generate_multiple_document_local_vuln_id_error("VULN-1", "/vulnerabilities/1/dl_vuln_id"),
        ]);

        let case_02 = Err(vec![
            generate_multiple_document_local_vuln_id_error("VULN-3", "/vulnerabilities/0/dl_vuln_id"),
            generate_multiple_document_local_vuln_id_error("VULN-3", "/vulnerabilities/2/dl_vuln_id"),
            generate_multiple_document_local_vuln_id_error("VULN-4", "/vulnerabilities/1/dl_vuln_id"),
            generate_multiple_document_local_vuln_id_error("VULN-4", "/vulnerabilities/3/dl_vuln_id"),
            generate_multiple_document_local_vuln_id_error("VULN-4", "/vulnerabilities/4/dl_vuln_id"),
        ]);

        TESTS_2_1.test_6_1_65_2.expect(ExpectedResults {
            case_01,
            case_02,
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
