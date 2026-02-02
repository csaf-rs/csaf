use crate::csaf_traits::{CsafDocumentCategory, CsafTrait, DocumentTrait, VulnerabilityTrait};
use crate::document_category_test_helper::DocumentCategoryTestConfig;
use crate::validation::ValidationError;

/// 6.1.27.5 Vulnerability Notes
///
/// This test only applies to documents with `/document/category` with value `csaf_security_advisory` and `csaf_vex` for
/// `/document/csaf_version` `2.0` and additionally to documents with `/document/category` with
/// value `csaf_deprecated_security_advisory` for `/document/csaf_version` `2.1`.
///
/// Documents with these categories must have a `/vulnerabilities[]/notes` element.
pub fn test_6_1_27_05_vulnerability_notes(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(());
    }

    let mut errors: Option<Vec<ValidationError>> = None;
    // return error if there are vulnerabilities without notes
    for (v_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if vulnerability.get_notes().is_none() {
            errors
                .get_or_insert_with(Vec::new)
                .push(test_6_1_27_05_err_generator(&doc_category, &v_i));
        }
    }

    errors.map_or(Ok(()), Err)
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig = DocumentCategoryTestConfig::new()
    .shared(&[
        CsafDocumentCategory::CsafSecurityAdvisory,
        CsafDocumentCategory::CsafVex,
    ])
    .csaf21(&[CsafDocumentCategory::CsafDeprecatedSecurityAdvisory]);

fn test_6_1_27_05_err_generator(document_category: &CsafDocumentCategory, vuln_path_index: &usize) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with category '{document_category}' must have a notes element in each vulnerability"
        ),
        instance_path: format!("/vulnerabilities/{vuln_path_index}/notes"),
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_27_5
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_05_vulnerability_notes(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_27_5
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_05_vulnerability_notes(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_05() {
        let case_01 = Err(vec![test_6_1_27_05_err_generator(
            &CsafDocumentCategory::CsafSecurityAdvisory,
            &0,
        )]);

        TESTS_2_0.test_6_1_27_5.expect(case_01.clone());
        TESTS_2_1.test_6_1_27_5.expect(
            case_01,
            Err(vec![test_6_1_27_05_err_generator(&CsafDocumentCategory::CsafVex, &0)]),
            Err(vec![test_6_1_27_05_err_generator(&CsafDocumentCategory::CsafVex, &0)]),
        );
    }
}
