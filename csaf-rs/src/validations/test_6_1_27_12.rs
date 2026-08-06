use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait, ProductStatusTrait, VulnerabilityTrait};
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;

fn create_missing_affected_products_error(
    document_category: &CsafDocumentCategory,
    vulnerability_index: usize,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Document with category '{document_category}' must have a '/vulnerabilities[]/product_status/known_affected' element"
        ),
        instance_path: format!("/vulnerabilities[{vulnerability_index}]/product_status/known_affected"),
    })
}

/// 6.1.27.12 Affected Products
///
/// This test only applies to documents with `/document/category` with value `csaf_security_advisory`.
///
/// For each item in `/vulnerabilities[]` it MUST be tested that the element `product_status/known_affected` exists.
pub fn test_6_1_27_12_affected_products(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(()); // ToDo generate skipped https://github.com/csaf-rs/csaf/issues/409
    }

    let mut errors: Option<Vec<TestFinding>> = None;
    let vulnerabilities = doc.get_vulnerabilities();
    for (v_i, vulnerability) in vulnerabilities.iter().enumerate() {
        if vulnerability
            .get_product_status()
            .and_then(|ps| ps.get_known_affected())
            .is_none()
        {
            errors
                .get_or_insert_default()
                .push(create_missing_affected_products_error(&doc_category, v_i));
        }
    }

    errors.map_or(Ok(()), Err)
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().shared(&[CsafDocumentCategory::CsafSecurityAdvisory]);

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_27_12, test_6_1_27_12_affected_products);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_27_12 as ExpectedResults;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_12() {
        let case_security_advisory_missing_affected = Err(vec![create_missing_affected_products_error(
            &CsafDocumentCategory::CsafSecurityAdvisory,
            0,
        )]);
        let case_security_advisory_two_vulnerabilities_missing_affected = Err(vec![
            create_missing_affected_products_error(&CsafDocumentCategory::CsafSecurityAdvisory, 0),
            create_missing_affected_products_error(&CsafDocumentCategory::CsafSecurityAdvisory, 1),
        ]);

        TESTS_2_1.test_6_1_27_12.expect(ExpectedResults {
            case_01: case_security_advisory_missing_affected.clone(),
            case_s01: case_security_advisory_missing_affected,
            case_s02: case_security_advisory_two_vulnerabilities_missing_affected,
            case_11: Ok(()),
            case_s11: Ok(()),
        });
    }
}
