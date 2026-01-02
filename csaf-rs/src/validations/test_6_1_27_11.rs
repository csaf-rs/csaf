use crate::csaf_traits::{CsafTrait, DocumentCategory};
use crate::profile_test_applies_to_category;
use crate::validation::ValidationError;

/// 6.1.27.11 Vulnerabilities
///
/// This test only applies to documents with `/document/category` with value `csaf_security_advisory` and `csaf_vex` for
/// `/document/csaf_version` `2.0` and additionally to documents with `/document/category` with
/// value `csaf_deprecated_security_advisory` for `/document/csaf_version` `2.1`.
///
/// In documents with this category a `/vulnerabilities[]` element must exist.
#[profile_test_applies_to_category(
    all = [CsafSecurityAdvisory, CsafVex],
    csaf21 = [CsafDeprecatedSecurityAdvisory]
)]
pub fn test_6_1_27_11_vulnerabilities(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    if doc.get_vulnerabilities().is_empty() {
        return Err(vec![test_6_1_27_11_err_generator(&doc.get_document().get_category())]);
    }

    Ok(())
}

fn test_6_1_27_11_err_generator(document_category: &DocumentCategory) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with category '{}' must not have a '/vulnerabilities' element",
            document_category
        ),
        instance_path: "/vulnerabilities".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_27_11() {
        let errors = HashMap::from([
            (
                "01",
                vec![test_6_1_27_11_err_generator(&DocumentCategory::CsafSecurityAdvisory)],
            ),
            ("02", vec![test_6_1_27_11_err_generator(&DocumentCategory::CsafVex)]),
            (
                "03",
                vec![test_6_1_27_11_err_generator(
                    &DocumentCategory::CsafDeprecatedSecurityAdvisory,
                )],
            ),
        ]);
        run_csaf20_tests("27-11", test_6_1_27_11_vulnerabilities, errors.clone());
        run_csaf21_tests("27-11", test_6_1_27_11_vulnerabilities, errors);
    }
}
