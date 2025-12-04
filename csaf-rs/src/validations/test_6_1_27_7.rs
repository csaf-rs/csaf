use crate::csaf_traits::{CsafTrait, DocumentCategory, DocumentTrait, ProductStatusTrait, VulnerabilityTrait};
use crate::validation::ValidationError;

/// 6.1.27.7 VEX Product Status
///
/// This test only applies to documents with `/document/category` with value `csaf_vex`.
///
/// In documents with this category each `/vulnerabilities[]/product_status` must have at least one
/// of the elements: `fixed`, `known_affected`, `known_not_affected` or `under_investigation`
pub fn test_6_1_27_7_vex_product_status(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    if doc_category != DocumentCategory::CsafVex {
        return Ok(());
    }

    let mut errors: Option<Vec<ValidationError>> = None;
    // return error if there are vulnerabilities without fixed, known_affected, known_not_affected or under_investigation in product_status
    for (v_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(product_status) = vulnerability.get_product_status() {
            if !(product_status.get_fixed().is_some()
                || product_status.get_known_affected().is_some()
                || product_status.get_known_not_affected().is_some()
                || product_status.get_under_investigation().is_some())
            {
                errors
                    .get_or_insert_with(Vec::new)
                    .push(test_6_1_27_7_err_generator(&doc_category, &v_i));
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

fn test_6_1_27_7_err_generator(document_category: &DocumentCategory, vuln_path_index: &usize) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with category '{}' must provide at least one fixed, known_affected, known_unaffected or under_investigation product_status in each vulnerability",
            document_category
        ),
        instance_path: format!("/vulnerabilities/{}/product_status", vuln_path_index),
    }
}

#[cfg(test)]
mod tests {
    use crate::csaf_traits::DocumentCategory;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validations::test_6_1_27_7::{test_6_1_27_7_err_generator, test_6_1_27_7_vex_product_status};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_27_4() {
        let errors = HashMap::from([("01", vec![test_6_1_27_7_err_generator(&DocumentCategory::CsafVex, &0)])]);
        run_csaf20_tests("27-07", test_6_1_27_7_vex_product_status, errors.clone());
        run_csaf21_tests("27-07", test_6_1_27_7_vex_product_status, errors);
    }
}
