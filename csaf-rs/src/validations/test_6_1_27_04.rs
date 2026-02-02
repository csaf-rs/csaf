use crate::csaf_traits::{CsafDocumentCategory, CsafTrait, DocumentTrait};
use crate::document_category_test_helper::DocumentCategoryTestConfig;
use crate::validation::ValidationError;

/// 6.1.27.4 Product Tree
///
/// This test only applies to documents with `/document/category` with value `csaf_security_advisory` and `csaf_vex` for
/// `/document/csaf_version` `2.0` and additionally to documents with `/document/category` with
/// value `csaf_deprecated_security_advisory` for `/document/csaf_version` `2.1`.
///
/// Documents with this category must have a `/product_tree` element.
pub fn test_6_1_27_04_product_tree(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(());
    }

    // return error if there are there isn't a product tree
    if doc.get_product_tree().is_none() {
        return Err(vec![test_6_1_27_04_err_generator(doc_category)]);
    }

    Ok(())
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig = DocumentCategoryTestConfig::new()
    .shared(&[
        CsafDocumentCategory::CsafSecurityAdvisory,
        CsafDocumentCategory::CsafVex,
    ])
    .csaf21(&[CsafDocumentCategory::CsafDeprecatedSecurityAdvisory]);

fn test_6_1_27_04_err_generator(document_category: CsafDocumentCategory) -> ValidationError {
    ValidationError {
        message: format!("Document with category '{document_category}' must have a '/product_tree' element"),
        instance_path: "/product_tree".to_string(),
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_27_4
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_04_product_tree(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_27_4
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_04_product_tree(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_04() {
        let case_01 = Err(vec![test_6_1_27_04_err_generator(
            CsafDocumentCategory::CsafSecurityAdvisory,
        )]);

        TESTS_2_0.test_6_1_27_4.expect(case_01.clone());
        TESTS_2_1.test_6_1_27_4.expect(
            case_01,
            Err(vec![test_6_1_27_04_err_generator(CsafDocumentCategory::CsafVex)]),
            Err(vec![test_6_1_27_04_err_generator(
                CsafDocumentCategory::CsafDeprecatedSecurityAdvisory,
            )]),
        );
    }
}
