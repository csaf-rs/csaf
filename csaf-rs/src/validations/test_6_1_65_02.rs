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
