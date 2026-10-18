use crate::csaf_traits::CsafTrait;
use crate::validation::{TestFinding, TestFindingData};

fn generate_undefined_document_local_vuln_id_error(id: &str, path: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("Missing definition of a document-local vulnerability id (`dl_vuln_id`): {id}"),
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
    let defined_ids = doc.get_all_document_local_vuln_ids();

    let mut errors: Option<Vec<TestFinding>> = None;
    for (id, path) in doc.get_all_document_local_vuln_id_references() {
        if !defined_ids.contains(&id) {
            errors
                .get_or_insert_default()
                .push(generate_undefined_document_local_vuln_id_error(&id, &path));
        }
    }

    errors.map_or(Ok(()), Err)
}
