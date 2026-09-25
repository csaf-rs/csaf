use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;
use std::sync::LazyLock;

/// 6.2.39.7 Document Acknowledgments
///
/// This test only applies to documents with `/document/category` with value
/// `csaf_vulnerability_report`.
///
/// It SHALL be tested that `/document/acknowledgments` exists.
pub fn test_6_2_39_7_document_acknowledgments(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();
    let document_category = document.get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(document.get_csaf_version(), &document_category) {
        return Ok(());
    }

    if document.get_acknowledgments().is_none() {
        return Err(vec![MISSING_ACKNOWLEDGMENTS_WARNING.clone()]);
    }

    Ok(())
}

static MISSING_ACKNOWLEDGMENTS_WARNING: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Warning(TestFindingData {
        message: "The document does not contain the acknowledgments element.".to_string(),
        instance_path: "/document/acknowledgments".to_string(),
    })
});

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().csaf21(&[CsafDocumentCategory::CsafVulnerabilityReport]);

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_39_7,
    test_6_2_39_7_document_acknowledgments
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_39_7 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_39_7() {
        let missing_acknowledgments = Err(vec![MISSING_ACKNOWLEDGMENTS_WARNING.clone()]);

        // Case 11: present acknowledgments
        TESTS_2_1.test_6_2_39_7.expect(ExpectedResults {
            case_01: missing_acknowledgments,
            case_11: Ok(()),
        });
    }
}
