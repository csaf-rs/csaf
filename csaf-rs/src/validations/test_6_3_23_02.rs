use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait, VulnerabilityTrait};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;

fn create_missing_disclosure_date_info(vulnerability_index: usize, category: CsafDocumentCategory) -> TestFinding {
    TestFinding::Information(TestFindingData {
        message: format!(
            "The vulnerability does not contain disclosure_date, which is recommended for document category `{category}`",
        ),
        instance_path: format!("/vulnerabilities/{vulnerability_index}"),
    })
}

/// 6.3.23.2 Disclosure Date
///
/// This test only applies to documents with `/document/category` with value
/// `csaf_vulnerability_report`.
///
/// It SHALL be tested that `$.vulnerabilities[*].disclosure_date` exists.
pub fn test_6_3_23_2_disclosure_date(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();
    let document_category = document.get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(document.get_csaf_version(), &document_category) {
        return Ok(());
    }

    let mut findings: Option<Vec<TestFinding>> = None;

    for (vulnerability_index, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if vulnerability.get_disclosure_date().is_none() {
            findings
                .get_or_insert_default()
                .push(create_missing_disclosure_date_info(
                    vulnerability_index,
                    document_category.clone(),
                ));
        }
    }

    findings.map_or(Ok(()), Err)
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().csaf21(&[CsafDocumentCategory::CsafVulnerabilityReport]);

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_3_23_2, test_6_3_23_2_disclosure_date);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_3_23_2 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_3_23_2() {
        let missing_disclosure_date = Err(vec![create_missing_disclosure_date_info(
            0,
            CsafDocumentCategory::CsafVulnerabilityReport,
        )]);

        let multiple_missing_disclosure_dates = Err(vec![
            create_missing_disclosure_date_info(0, CsafDocumentCategory::CsafVulnerabilityReport),
            create_missing_disclosure_date_info(2, CsafDocumentCategory::CsafVulnerabilityReport),
        ]);

        // Case 01: minimal vulnerability, with missing disclosure_date
        // Case 02: more complex vulnerability with missing disclosure_date
        // Case s01: alternating vulns with and without disclosure dates
        // Case 11: Case 02, but with a disclosure_date present
        // Case S11: missing disclosure_date, but test is skipped because of unrelated document category

        // Note: S01 is invalid, as it's missing the notes required to pass 6.1.27.20
        // This simplification is acceptable here.
        TESTS_2_1.test_6_3_23_2.expect(ExpectedResults {
            case_01: missing_disclosure_date.clone(),
            case_02: missing_disclosure_date,
            case_s01: multiple_missing_disclosure_dates,
            case_11: Ok(()),
            case_s11: Ok(()),
        });
    }
}
