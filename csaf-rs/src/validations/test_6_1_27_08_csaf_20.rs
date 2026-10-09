use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait, VulnerabilityTrait};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;

fn create_cve_and_ids_are_not_present_error(
    document_category: &CsafDocumentCategory,
    vuln_path_index: usize,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Document with category '{document_category}' must provide at at least one of `cve` or `ids` for each vulnerability"
        ),
        instance_path: format!("/vulnerabilities/{vuln_path_index}/product_status"),
    })
}

/// 6.1.27.8 Vulnerability ID
///
/// This test only applies to documents with `/document/category` with value `csaf_vex`.
///
/// In documents with this category each `/vulnerabilities[]` item must have at least one of the elements `cve` or `ids` present.
/// element.
///
/// The implementation of this test has significantly diverged between CSAF 2.0 and CSAF 2.1, so we have two separate implementations for each version.
/// This is the CSAF 2.0 implementation.
pub fn test_6_1_27_08_vulnerability_id_csaf_20(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category(&doc_category) {
        return Ok(()); // ToDo generate skipped https://github.com/csaf-rs/csaf/issues/409
    }

    let mut errors: Option<Vec<TestFinding>> = None;
    // for each vulnerability
    for (v_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        // check if the "cve" and "ids" fields are both None, i.e. not present
        // if so, add an error
        if vulnerability.get_cve().is_none() && vulnerability.get_ids().is_none() {
            errors
                .get_or_insert_default()
                .push(create_cve_and_ids_are_not_present_error(&doc_category, v_i));
        }
    }

    errors.map_or(Ok(()), Err)
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().csaf20(&[CsafDocumentCategory::CsafVex]);

crate::test_validation::impl_validator!(
    csaf2_0,
    ValidatorForTest6_1_27_8,
    test_6_1_27_08_vulnerability_id_csaf_20
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_1_27_8 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;

    #[test]
    fn test_test_6_1_27_08() {
        let case_vex_without_cve_or_id = Err(vec![create_cve_and_ids_are_not_present_error(
            &CsafDocumentCategory::CsafVex,
            0,
        )]);

        TESTS_2_0.test_6_1_27_8.expect(ExpectedResults_2_0 {
            case_01: case_vex_without_cve_or_id.clone(),
            case_s11: Ok(()),
        });
    }
}
