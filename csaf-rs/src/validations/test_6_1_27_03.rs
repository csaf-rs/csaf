use crate::csaf_traits::{CsafTrait, DocumentCategory};
use crate::validation::ValidationError;
use csaf_macros::profile_test_applies_to_category;

fn create_vulnerabilities_error(doc_category: &DocumentCategory) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with category '{}' must not have a '/vulnerabilities' element",
            doc_category
        ),
        instance_path: "/vulnerabilities".to_string(),
    }
}

/// 6.1.27.3 Vulnerabilities
///
/// This test only applies to documents with `/document/category` with value `csaf_informational_advisory` for
/// `/document/csaf_version` `2.0` and additionally to documents with `/document/category` with
/// value `csaf_withdrawn` and `csaf_superseded` for `/document/csaf_version` `2.1`.
///
/// Documents with this category must not have a `/vulnerabilities` element.
#[profile_test_applies_to_category(
    all = [CsafInformationalAdvisory],
    csaf21 = [CsafWithdrawn, CsafSuperseded]
)]
pub fn test_6_1_27_03_vulnerability(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // return error if there are elements in /vulnerabilities
    if !doc.get_vulnerabilities().is_empty() {
        return Err(vec![create_vulnerabilities_error(&doc.get_document().get_category())]);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_27_03() {
        let errors = HashMap::from([
            (
                "01",
                vec![create_vulnerabilities_error(
                    &DocumentCategory::CsafInformationalAdvisory,
                )],
            ),
            (
                "02",
                vec![create_vulnerabilities_error(&DocumentCategory::CsafWithdrawn)],
            ),
            (
                "03",
                vec![create_vulnerabilities_error(&DocumentCategory::CsafSuperseded)],
            ),
        ]);
        run_csaf20_tests("27-03", test_6_1_27_03_vulnerability, errors.clone());
        run_csaf21_tests("27-03", test_6_1_27_03_vulnerability, errors);
    }
}
