use crate::csaf_traits::{CsafDocumentCategory, CsafTrait, DocumentTrait};
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
        return Ok(());
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

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_27_11
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_11_vulnerabilities(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_27_11
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_11_vulnerabilities(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_11() {
        let case_01 = Err(vec![create_missing_vulnerabilities_error(
            &CsafDocumentCategory::CsafSecurityAdvisory,
        )]);

        // CSAF 2.0 has 1 test case
        TESTS_2_0.test_6_1_27_11.expect(case_01.clone());

        // CSAF 2.1 has 3 test cases
        TESTS_2_1.test_6_1_27_11.expect(
            case_01,
            Err(vec![create_missing_vulnerabilities_error(
                &CsafDocumentCategory::CsafVex,
            )]),
            Err(vec![create_missing_vulnerabilities_error(
                &CsafDocumentCategory::CsafDeprecatedSecurityAdvisory,
            )]),
        );
    }
}
