use crate::csaf_traits::{CsafTrait, DocumentCategory, DocumentTrait, ProductStatusTrait, VulnerabilityTrait};
use crate::document_category_test_helper::ProfileTestConfig;
use crate::validation::ValidationError;

/// 6.1.27.7 VEX Product Status
///
/// This test only applies to documents with `/document/category` with value `csaf_vex`.
///
/// In documents with this category each `/vulnerabilities[]/product_status` must have at least one
/// of the elements: `fixed`, `known_affected`, `known_not_affected` or `under_investigation`
pub fn test_6_1_27_07_vex_product_status(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category(&doc_category) {
        return Ok(());
    }

    let mut errors: Option<Vec<ValidationError>> = None;
    // return error if there are vulnerabilities without fixed, known_affected, known_not_affected or under_investigation in product_status
    for (v_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if let Some(product_status) = vulnerability.get_product_status() {
            if !(product_status.get_fixed().is_some()
                || product_status.get_known_affected().is_some()
                || product_status.get_known_not_affected().is_some()
                || product_status.get_under_investigation().is_some())
            {
                errors
                    .get_or_insert_with(Vec::new)
                    .push(test_6_1_27_07_err_generator(&doc_category, &v_i));
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().shared(&[DocumentCategory::CsafVex]);

fn test_6_1_27_07_err_generator(document_category: &DocumentCategory, vuln_path_index: &usize) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with category '{}' must provide at least one fixed, known_affected, known_unaffected or under_investigation product_status in each vulnerability",
            document_category
        ),
        instance_path: format!("/vulnerabilities/{}/product_status", vuln_path_index),
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_27_7
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_07_vex_product_status(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_27_7
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_07_vex_product_status(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_07() {
        let case_01 = Err(vec![test_6_1_27_07_err_generator(&DocumentCategory::CsafVex, &0)]);

        TESTS_2_0.test_6_1_27_7.expect(case_01.clone());
        TESTS_2_1.test_6_1_27_7.expect(case_01);
    }
}
