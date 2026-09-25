use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait, VulnerabilityTrait};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;

fn create_missing_metrics_warning(vulnerability_index: usize, category: CsafDocumentCategory) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!(
            "The vulnerability does not contain metrics, which is recommended for document category `{category}`",
        ),
        instance_path: format!("/vulnerabilities/{vulnerability_index}"),
    })
}

/// 6.2.39.10 Vulnerability Metrics
///
/// This test only applies to documents with `/document/category` with value
/// `csaf_vulnerability_report`.
///
/// It SHALL be tested that `$.vulnerabilities[*].metrics` exists.
pub fn test_6_2_39_10_vulnerability_metrics(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();
    let document_category = document.get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(document.get_csaf_version(), &document_category) {
        return Ok(());
    }

    let mut errors: Option<Vec<TestFinding>> = None;

    for (vuln_index, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if vulnerability.get_metrics().is_none() {
            errors
                .get_or_insert_default()
                .push(create_missing_metrics_warning(vuln_index, document_category.clone()));
        }
    }

    errors.map_or(Ok(()), Err)
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().csaf21(&[CsafDocumentCategory::CsafVulnerabilityReport]);

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_2_39_10, test_6_2_39_10_vulnerability_metrics);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_39_10 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_39_10() {
        let missing_metrics_on_vuln_report = Err(vec![create_missing_metrics_warning(
            0,
            CsafDocumentCategory::CsafVulnerabilityReport,
        )]);

        // Case 01: simple doc without metrics
        // Case 02: more complex doc without metrics
        // Case 11: Case 11 with metrics
        TESTS_2_1.test_6_2_39_10.expect(ExpectedResults {
            case_01: missing_metrics_on_vuln_report.clone(),
            case_02: missing_metrics_on_vuln_report,
            case_11: Ok(()),
        });
    }
}
