use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;

fn create_missing_involvement_info(category: CsafDocumentCategory) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: format!(
            "The document does not contain involvements, which is recommended for document category `{category}`",
        ),
        instance_path: "/document".to_string(),
    })
}

/// 6.3.23.1 Involvement
///
/// This test only applies to documents with `/document/category` with value
/// `csaf_vulnerability_report`.
///
/// It SHALL be tested that `$.document.involvement` exists.
pub fn test_6_3_23_1_involvement(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();
    let document_category = document.get_category();

    // check if document category matches the profile test
    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(document.get_csaf_version(), &document_category) {
        return Ok(());
    }

    // check if involvement exists
    if document.get_involvement().is_none() {
        Err(vec![create_missing_involvement_info(document_category)])
    } else {
        Ok(())
    }
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().csaf21(&[CsafDocumentCategory::CsafVulnerabilityReport]);

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_3_23_1, test_6_3_23_1_involvement);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_23_1 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_23_1() {
        // Case 11: category is `csaf_vulnerability_report` and involvement exists
        TESTS_2_1.test_6_3_23_1.expect(ExpectedResults {
            case_01: Err(vec![create_missing_involvement_info(
                CsafDocumentCategory::CsafVulnerabilityReport,
            )]),
            case_11: Ok(()),
        });
    }
}
