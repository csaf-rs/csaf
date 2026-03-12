use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::document_category_test_helper::DocumentCategoryTestConfig;
use crate::validation::ValidationError;

fn create_revision_history_only_one_entry_error(document_category: &CsafDocumentCategory) -> ValidationError {
    ValidationError {
        message: format!(
            "The revision history contains only one entry which is prohibited for documents with category {document_category}"
        ),
        instance_path: "/document/tracking/revision_history".to_string(),
    }
}

/// 6.1.27.16 Revision history
///
/// This test only applies to documents with `/document/category` with value `csaf_withdrawn` or `csaf_superseded`.
///
/// The revision history shall not contain only one entry.
pub fn test_6_1_27_16_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(()); // TODO This will be a wasSkipped later ([#409](https://github.com/csaf-rs/csaf/issues/409))
    }
    if doc.get_document().get_tracking().get_revision_history().len() == 1 {
        return Err(vec![create_revision_history_only_one_entry_error(&doc_category)]);
    }

    Ok(())
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig = DocumentCategoryTestConfig::new().csaf21(&[
    CsafDocumentCategory::CsafWithdrawn,
    CsafDocumentCategory::CsafSuperseded,
]);

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_27_16
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_16_revision_history(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_16() {
        let fail_withdrawn = Err(vec![create_revision_history_only_one_entry_error(
            &CsafDocumentCategory::CsafWithdrawn,
        )]);
        let fail_superseded = Err(vec![create_revision_history_only_one_entry_error(
            &CsafDocumentCategory::CsafSuperseded,
        )]);
        // Case 11: document status is draft, category is csaf_withdrawn, two revision history elements, one of which is a draft
        // Case 12: document status is final, category is csaf_withdrawn, two revision history elements
        // Case 13: document status is final, category is csaf_superseded, two revision history elements
        // Case 14: document status is draft, category is csaf_superseded, two revision history elements, one of which is a draft
        TESTS_2_1
            .test_6_1_27_16
            .expect(fail_withdrawn, fail_superseded, Ok(()), Ok(()), Ok(()), Ok(()));
    }
}
