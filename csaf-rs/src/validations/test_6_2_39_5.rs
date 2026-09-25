use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;

fn create_extension_warning(doc_category: &CsafDocumentCategory, instance_path: &str) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!(
            "The document uses a CSAF Extension, which is not recommended for category `{doc_category}`."
        ),
        instance_path: instance_path.to_string(),
    })
}

/// 6.2.39.5 Extension in Superseded or Withdrawn Document
///
/// This test only applies to documents with `/document/category` with value `csaf_withdrawn` or `csaf_superseded`.
///
/// It SHALL be tested that the document does not contain an extension.
/// The relevant paths for this test are:
/// - `/document/x_extensions`
/// - `/x_extensions`
pub fn test_6_2_39_5_extension_in_superseded_or_withdrawn_document(
    doc: &impl CsafTrait,
) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();
    let doc_category = document.get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(document.get_csaf_version(), &doc_category) {
        return Ok(());
    }

    let mut warnings: Option<Vec<TestFinding>> = None;
    if document.get_extensions().is_some() {
        warnings
            .get_or_insert_default()
            .push(create_extension_warning(&doc_category, "/document/x_extensions"));
    }
    if doc.get_extensions().is_some() {
        warnings
            .get_or_insert_default()
            .push(create_extension_warning(&doc_category, "/x_extensions"));
    }

    warnings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_39_5,
    test_6_2_39_5_extension_in_superseded_or_withdrawn_document
);

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig = DocumentCategoryTestConfig::new().csaf21(&[
    CsafDocumentCategory::CsafWithdrawn,
    CsafDocumentCategory::CsafSuperseded,
]);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_39_5 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_39_5() {
        let root_extension_withdrawn = Err(vec![create_extension_warning(
            &CsafDocumentCategory::CsafWithdrawn,
            "/x_extensions",
        )]);

        let root_extension_superseded = Err(vec![create_extension_warning(
            &CsafDocumentCategory::CsafSuperseded,
            "/x_extensions",
        )]);

        // Case 11: withdrawn document without extension
        // Case 12: superseded document without extension

        TESTS_2_1.test_6_2_39_5.expect(ExpectedResults {
            case_01: root_extension_superseded,
            case_02: root_extension_withdrawn,
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
