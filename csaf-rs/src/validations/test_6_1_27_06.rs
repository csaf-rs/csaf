use crate::csaf_traits::{CsafTrait, DocumentCategory, VulnerabilityTrait};
use crate::validation::ValidationError;
use csaf_macros::profile_test_applies_to_category;

/// 6.1.27.6 Product Status
///
/// This test only applies to documents with `/document/category` with value `csaf_security_advisory` for
/// `/document/csaf_version` `2.0` and additionally to documents with `/document/category` with
/// value `csaf_deprecated_security_advisory` for `/document/csaf_version` `2.1`.
///
/// Documents with these categories must have a `/vulnerabilities[]/product_status` element.
#[profile_test_applies_to_category(
    all = [CsafSecurityAdvisory],
    csaf21 = [CsafDeprecatedSecurityAdvisory]
)]
pub fn test_6_1_27_06_product_status(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;
    // return error if there are vulnerabilities without product_status
    for (v_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if vulnerability.get_product_status().is_none() {
            errors
                .get_or_insert_with(Vec::new)
                .push(test_6_1_27_06_err_generator(&doc.get_document().get_category(), &v_i));
        }
    }

    errors.map_or(Ok(()), Err)
}

fn test_6_1_27_06_err_generator(document_category: &DocumentCategory, vuln_path_index: &usize) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with category '{}' must have a product_status element in each vulnerability",
            document_category
        ),
        instance_path: format!("/vulnerabilities/{}/product_status", vuln_path_index),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_27_06() {
        let errors = HashMap::from([
            (
                "01",
                vec![test_6_1_27_06_err_generator(
                    &DocumentCategory::CsafSecurityAdvisory,
                    &0,
                )],
            ),
            (
                "02",
                vec![test_6_1_27_06_err_generator(
                    &DocumentCategory::CsafSecurityAdvisory,
                    &0,
                )],
            ),
            (
                "03",
                vec![test_6_1_27_06_err_generator(
                    &DocumentCategory::CsafDeprecatedSecurityAdvisory,
                    &0,
                )],
            ),
        ]);
        run_csaf20_tests("27-06", test_6_1_27_06_product_status, errors.clone());
        run_csaf21_tests("27-06", test_6_1_27_06_product_status, errors);
    }
}
