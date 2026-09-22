use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;

/// 6.2.39.8 Aggregate Severity
///
/// This test only applies to documents with `/document/category` with value
/// `csaf_vulnerability_report`.
///
/// It SHALL be tested that `/document/aggregate_severity` exists.
pub fn test_6_2_39_8_aggregate_severity(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();
    let document_category = document.get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(document.get_csaf_version(), &document_category) {
        return Ok(());
    }

    if document.get_aggregate_severity().is_none() {
        return Err(vec![create_missing_aggregate_severity_warning()]);
    }

    Ok(())
}

fn create_missing_aggregate_severity_warning() -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: "The document does not contain the aggregate severity element.".to_string(),
        instance_path: "/document/aggregate_severity".to_string(),
    })
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().csaf21(&[CsafDocumentCategory::CsafVulnerabilityReport]);

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_2_39_8, test_6_2_39_8_aggregate_severity);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_39_8 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_39_8() {
        let missing_aggregate_severity = Err(vec![create_missing_aggregate_severity_warning()]);

        // Case 01: document aggregate severity is missing
        TESTS_2_1.test_6_2_39_8.expect(ExpectedResults {
            case_01: missing_aggregate_severity,
            // Case 11: document aggregate severity is present
            case_11: Ok(()),
        });
    }
}
