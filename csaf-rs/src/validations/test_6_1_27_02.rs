use crate::csaf_traits::{CsafDocumentCategory, CsafTrait, DocumentReferenceTrait, DocumentTrait};
use crate::document_category_test_helper::DocumentCategoryTestConfig;
use crate::schema::csaf2_1::schema::CategoryOfReference;
use crate::validation::ValidationError;

fn create_missing_external_reference_error(doc_category: &CsafDocumentCategory) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with category '{doc_category}' must have at least one reference with category 'external'"
        ),
        instance_path: "/document/references".to_string(),
    }
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig = DocumentCategoryTestConfig::new().shared(&[
    CsafDocumentCategory::CsafInformationalAdvisory,
    CsafDocumentCategory::CsafSecurityIncidentResponse,
]);

/// 6.1.27.2 Document References
///
/// This test only applies to documents with `/document/category` with value `csaf_informational_advisory`
/// or `csaf_security_incident_response`.
///
/// Documents with these categories must have at least one entry in `/document/notes` with `category` values
/// of `description`, `details`, `general` or `summary`.
pub fn test_6_1_27_02_document_references(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(());
    }

    // check if there is a document reference with category 'external'
    let mut found_external_reference = false;
    if let Some(references) = doc.get_document().get_references() {
        for reference in references.iter() {
            if CategoryOfReference::External == *reference.get_category() {
                found_external_reference = true;
                break;
            };
        }
    }

    // if there isn't a reference with category 'external', return an error
    if !found_external_reference {
        return Err(vec![create_missing_external_reference_error(&doc_category)]);
    }

    Ok(())
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_27_2
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_02_document_references(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_27_2
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_02_document_references(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_02() {
        let case_01 = Err(vec![create_missing_external_reference_error(
            &CsafDocumentCategory::CsafInformationalAdvisory,
        )]);

        TESTS_2_0.test_6_1_27_2.expect(case_01.clone());
        TESTS_2_1.test_6_1_27_2.expect(case_01);
    }
}
