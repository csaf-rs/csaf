use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DistributionTrait, DocumentTrait, TlpTrait};
use crate::schema::csaf2_1::schema::LabelOfTlp;
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;
use std::sync::LazyLock;

/// 6.2.39.9 TLP-Label for Vulnerability Report
///
/// This test only applies to documents with `/document/category` with value
/// `csaf_vulnerability_report`.
///
/// It SHALL be tested that `/document/distribution/tlp/label` is set to `AMBER`.
pub fn test_6_2_39_9_tlp_label_for_vulnerability_report(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let document = doc.get_document();
    let document_category = document.get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(document.get_csaf_version(), &document_category) {
        return Ok(());
    }

    let distribution = document.get_distribution_21().map_err(|e| vec![e])?;

    if distribution.get_tlp_21().map_err(|e| vec![e])?.get_label() != LabelOfTlp::Amber {
        return Err(vec![NON_AMBER_TLP_LABEL_WARNING.clone()]);
    }

    Ok(())
}

static NON_AMBER_TLP_LABEL_WARNING: LazyLock<TestFinding> = LazyLock::new(|| {
    TestFinding::Warning(TestFindingData {
        message: "The vulnerability report uses a TLP label other than the recommended value `AMBER`.".to_string(),
        instance_path: "/document/distribution/tlp/label".to_string(),
    })
});

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().csaf21(&[CsafDocumentCategory::CsafVulnerabilityReport]);

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_2_39_9,
    test_6_2_39_9_tlp_label_for_vulnerability_report
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_39_9 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_39_9() {
        let non_amber_tlp_label = Err(vec![NON_AMBER_TLP_LABEL_WARNING.clone()]);

        // Case 11: TLP label is AMBER
        TESTS_2_1.test_6_2_39_9.expect(ExpectedResults {
            case_01: non_amber_tlp_label,
            case_11: Ok(()),
        });
    }
}
