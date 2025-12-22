use crate::csaf_traits::{CsafTrait, CsafVersion, DocumentCategory, DocumentTrait};
use crate::validation::ValidationError;
use crate::validations::test_6_1_27_03::test_6_1_27_03_vulnerability;

/// 6.1.27.4 Product Tree
///
/// This test only applies to documents with `/document/category` with value `csaf_security_advisory` and `csaf_vex` for
/// `/document/csaf_version` `2.0` and additionally to documents with `/document/category` with
/// value `csaf_deprecated_security_advisory` for `/document/csaf_version` `2.1`.
///
/// Documents with this category must have a `/product_tree` element.
pub fn test_6_1_27_04_product_tree(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    // check if document is relevant document category in csaf 2.0
    if *doc.get_document().get_csaf_version() == CsafVersion::X20
        && doc_category != DocumentCategory::CsafSecurityAdvisory
        && doc_category != DocumentCategory::CsafVex
    {
        return Ok(());
    }

    // check if document is relevant document category in csaf 2.1
    if *doc.get_document().get_csaf_version() == CsafVersion::X21
        && doc_category != DocumentCategory::CsafSecurityAdvisory
        && doc_category != DocumentCategory::CsafVex
        && doc_category != DocumentCategory::CsafDeprecatedSecurityAdvisory
    {
        return Ok(());
    }

    // return error if there are there isn't a product tree
    if doc.get_product_tree().is_none() {
        return Err(vec![test_6_1_27_04_err_generator(doc_category)]);
    }

    Ok(())
}

fn test_6_1_27_04_err_generator(document_category: DocumentCategory) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with category '{}' must have a '/product_tree' element",
            document_category
        ),
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
        let case_01 = Err(vec![test_6_1_27_04_err_generator(DocumentCategory::CsafSecurityAdvisory)]);
        let case_02 = Err(vec![test_6_1_27_04_err_generator(DocumentCategory::CsafVex)]);
        let case_03 = Err(vec![test_6_1_27_04_err_generator(
            DocumentCategory::CsafDeprecatedSecurityAdvisory,
        )]);

        TESTS_2_0.test_6_1_27_4.expect(case_01.clone());
        TESTS_2_1.test_6_1_27_4.expect(case_01, case_02, case_03);
    }
}
