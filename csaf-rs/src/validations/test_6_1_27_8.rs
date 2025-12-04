use crate::csaf_traits::{CsafTrait, DocumentCategory, DocumentTrait, VulnerabilityTrait};
use crate::validation::ValidationError;

/// 6.1.27.8 Vulnerability ID
///
/// This test only applies to documents with `/document/category` with value `csaf_vex`.
///
/// In documents with this category each `/vulnerabilities[]` item must have at the `cve` or the `ids`
/// element.
pub fn test_6_1_27_8_vulnerability_id(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    if doc_category != DocumentCategory::CsafVex {
        return Ok(());
    }

    let mut errors: Option<Vec<ValidationError>> = None;
    // return error if there are vulnerabilities without either cve or ids
    for (v_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if vulnerability.get_cve().is_none() && vulnerability.get_ids().is_none() {
            errors
                .get_or_insert_with(Vec::new)
                .push(test_6_1_27_8_err_generator(&doc_category, &v_i));
        }
    }

    errors.map_or(Ok(()), Err)
}

fn test_6_1_27_8_err_generator(document_category: &DocumentCategory, vuln_path_index: &usize) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with category '{}' must provide at at least either cve or ids  in each vulnerability",
            document_category
        ),
        instance_path: format!("/vulnerabilities/{}/product_status", vuln_path_index),
    }
}

#[cfg(test)]
mod tests {
    use crate::csaf_traits::DocumentCategory;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validations::test_6_1_27_8::{test_6_1_27_8_err_generator, test_6_1_27_8_vulnerability_id};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_27_4() {
        let errors = HashMap::from([("01", vec![test_6_1_27_8_err_generator(&DocumentCategory::CsafVex, &0)])]);
        run_csaf20_tests("27-07", test_6_1_27_8_vulnerability_id, errors.clone());
        run_csaf21_tests("27-07", test_6_1_27_8_vulnerability_id, errors);
    }
}
