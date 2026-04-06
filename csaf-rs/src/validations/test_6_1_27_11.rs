use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::document_category_test_helper::DocumentCategoryTestConfig;
use crate::validation::ValidationError;

fn create_missing_vulnerabilities_error(document_category: &CsafDocumentCategory) -> ValidationError {
    ValidationError {
        message: format!("Document with category '{document_category}' must have a '/vulnerabilities' element"),
        instance_path: "/vulnerabilities".to_string(),
    }
}

/// 6.1.27.11 Vulnerabilities
///
/// This test only applies to documents with `/document/category` with value `csaf_security_advisory` and `csaf_vex` for
/// `/document/csaf_version` `2.0` and additionally to documents with `/document/category` with
/// value `csaf_deprecated_security_advisory` for `/document/csaf_version` `2.1`.
///
/// In documents with this category a `/vulnerabilities[]` element must exist.
pub fn test_6_1_27_11_vulnerabilities(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(()); // ToDo generate skipped https://github.com/csaf-rs/csaf/issues/409
    }

    if doc.get_vulnerabilities().is_empty() {
        return Err(vec![create_missing_vulnerabilities_error(&doc_category)]);
    }

    Ok(())
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig = DocumentCategoryTestConfig::new()
    .shared(&[
        CsafDocumentCategory::CsafSecurityAdvisory,
        CsafDocumentCategory::CsafVex,
    ])
    .csaf21(&[CsafDocumentCategory::CsafDeprecatedSecurityAdvisory]);

crate::test_validation::impl_validator!(ValidatorForTest6_1_27_11, test_6_1_27_11_vulnerabilities);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_11() {
        let case_security_advisory = Err(vec![create_missing_vulnerabilities_error(
            &CsafDocumentCategory::CsafSecurityAdvisory,
        )]);
        let case_vex = Err(vec![create_missing_vulnerabilities_error(
            &CsafDocumentCategory::CsafVex,
        )]);
        let case_deprecated_security_advisory = Err(vec![create_missing_vulnerabilities_error(
            &CsafDocumentCategory::CsafDeprecatedSecurityAdvisory,
        )]);

        TESTS_2_0
            .test_6_1_27_11
            .expect(case_security_advisory.clone(), case_vex.clone(), Ok(()), Ok(()));

        TESTS_2_1.test_6_1_27_11.expect(
            case_security_advisory,
            case_vex,
            case_deprecated_security_advisory,
            Ok(()),
            Ok(()),
        );
    }
}
