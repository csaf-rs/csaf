use crate::csaf_traits::{CsafTrait, DocumentCategory, DocumentTrait, ProductTrait, ProductTreeTrait};
use crate::document_category_test_helper::DocumentCategoryTestConfig;
use crate::validation::ValidationError;

fn create_unused_product_id_error(product_id: &str, path: &str) -> ValidationError {
    ValidationError {
        message: format!(
            "Product ID '{}' is defined but not referenced in the document",
            product_id
        ),
        instance_path: format!("{}/product_id", path),
    }
}

const SKIP_TEST_CONFIG: DocumentCategoryTestConfig =
    DocumentCategoryTestConfig::new().shared(&[DocumentCategory::CsafInformationalAdvisory]);

/// 6.2.1 Unused Definition of Product ID
///
/// All defined product IDs need to be referenced at least once in the document.
pub fn test_6_2_01_unused_definition_of_product_id(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    // Skips the test for profile "Informational Advisory"
    if SKIP_TEST_CONFIG.matches_category(&doc.get_document().get_category()) {
        return Ok(());
    }

    // Get all references to product IDs in the document
    let references = doc.get_all_product_references_ids();

    // Visit all product id definitions and check if they are referenced
    if let Some(tree) = doc.get_product_tree().as_ref() {
        tree.visit_all_products(&mut |fpn, path| {
            if !references.contains(fpn.get_product_id()) {
                errors
                    .get_or_insert_with(Vec::new)
                    .push(create_unused_product_id_error(fpn.get_product_id(), path));
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_2_1
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_01_unused_definition_of_product_id(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_1
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_01_unused_definition_of_product_id(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_01() {
        let case_01 = Err(vec![create_unused_product_id_error(
            "CSAFPID-9080700",
            "/product_tree/full_product_names/0",
        )]);

        // Both CSAF 2.0 and 2.1 have 2 test cases
        TESTS_2_0.test_6_2_1.expect(case_01.clone(), Ok(()));
        TESTS_2_1.test_6_2_1.expect(case_01, Ok(()));
    }
}
