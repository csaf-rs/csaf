use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::ValidationError;
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;

fn create_product_tree_exists_error(document_category: &CsafDocumentCategory) -> ValidationError {
    ValidationError {
        message: format!(
            "The document contains a product tree which is prohibited for documents with category {document_category}"
        ),
        instance_path: "/product_tree".to_string(),
    }
}

/// 6.1.27.15 Product tree
///
/// This test only applies to documents with `/document/category` with value `csaf_withdrawn` or `csaf_superseded`.
///
/// An item `/product_tree` shall not exist.
pub fn test_6_1_27_15_product_tree(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(()); // ToDo generate skipped https://github.com/csaf-rs/csaf/issues/409
    }
    if doc.get_product_tree().is_some() {
        return Err(vec![create_product_tree_exists_error(&doc_category)]);
    }

    Ok(())
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig = DocumentCategoryTestConfig::new().csaf21(&[
    CsafDocumentCategory::CsafWithdrawn,
    CsafDocumentCategory::CsafSuperseded,
]);

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_27_15, test_6_1_27_15_product_tree);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_15() {
        let fail_withdrawn = Err(vec![create_product_tree_exists_error(
            &CsafDocumentCategory::CsafWithdrawn,
        )]);
        let fail_superseded = Err(vec![create_product_tree_exists_error(
            &CsafDocumentCategory::CsafSuperseded,
        )]);
        // Case 11: category csaf_withdrawn, no product tree
        // Case 12: category csaf_superseded, no product tree
        TESTS_2_1
            .test_6_1_27_15
            .expect(fail_withdrawn, fail_superseded, Ok(()), Ok(()));
    }
}
