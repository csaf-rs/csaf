use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait, ProductStatusTrait, VulnerabilityTrait};
use crate::document_category_test_helper::DocumentCategoryTestConfig;
use crate::validation::ValidationError;

fn create_missing_affected_products_error(
    document_category: &CsafDocumentCategory,
    vulnerability_index: usize,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with category '{document_category}' must have a '/vulnerabilities[]/product_status/known_affected' element"
        ),
        instance_path: format!("/vulnerabilities[{vulnerability_index}]/product_status"),
    }
}

/// 6.1.27.12 Affected Products
///
/// This test only applies to documents with `/document/category` with value `csaf_security_advisory`.
///
/// For each item in /vulnerabilities it MUST be tested that the element product_status/known_affected exists.
pub fn test_6_1_27_12_affected_products(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(());
    }

    let vulnerabilities = doc.get_vulnerabilities();
    for (v_i, vulnerability) in vulnerabilities.iter().enumerate() {
        match vulnerability.get_product_status() {
            None => {
                return Err(vec![create_missing_affected_products_error(&doc_category, v_i)]);
            },
            Some(ps) => {
                if ps.get_known_affected().is_none() {
                    return Err(vec![create_missing_affected_products_error(&doc_category, v_i)]);
                }
            },
        }
    }

    Ok(())
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().shared(&[CsafDocumentCategory::CsafSecurityAdvisory]);

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_27_12
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_12_affected_products(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_12() {
        let case_security_advisory = Err(vec![create_missing_affected_products_error(
            &CsafDocumentCategory::CsafSecurityAdvisory,
            0,
        )]);

        TESTS_2_1.test_6_1_27_12.expect(case_security_advisory, Ok(()));
    }
}
