use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait, VulnerabilityTrait};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;

fn create_vulnerability_without_title_error(vuln_index: usize) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: String::from("Vulnerability in vulnerability report document has no title"),
        instance_path: format!("/vulnerabilities/{vuln_index}"),
    })
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().csaf21(&[CsafDocumentCategory::CsafVulnerabilityReport]);

/// 6.2.39.13 Vulnerability Title
///
/// This test checks if every vulnerability has a title if the document category is a Vulnerability
/// Report
pub fn test_6_2_39_13_vulnerability_title(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(
        doc.get_document().get_csaf_version(),
        &doc.get_document().get_category(),
    ) {
        return Ok(());
    }

    let findings = doc
        .get_vulnerabilities()
        .iter()
        .enumerate()
        .filter(|(_, vulnerability)| vulnerability.get_title().is_none())
        .map(|(vulnerability_idx, _)| create_vulnerability_without_title_error(vulnerability_idx))
        .collect::<Vec<_>>();

    if findings.is_empty() { Ok(()) } else { Err(findings) }
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_2_39_13, test_6_2_39_13_vulnerability_title);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_39_13 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_39_13() {
        let alternating_failing_vulnerability_entries_with_cve_but_missing_title = Err(vec![
            create_vulnerability_without_title_error(0),
            create_vulnerability_without_title_error(2),
        ]);
        TESTS_2_1.test_6_2_39_13.expect(ExpectedResults {
            case_01: Err(vec![create_vulnerability_without_title_error(0)]),
            case_02: Err(vec![create_vulnerability_without_title_error(0)]),
            case_s01: alternating_failing_vulnerability_entries_with_cve_but_missing_title,
            // Successfully passes this test, but is missing the product tree which is required for
            // a vulnerability report document
            case_11: Ok(()),
        });
    }
}
