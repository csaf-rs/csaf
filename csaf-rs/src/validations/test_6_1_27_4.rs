use crate::csaf_traits::{CsafTrait, CsafVersion, DocumentCategory, DocumentTrait};
use crate::validation::ValidationError;

/// 6.1.27.4 Product Tree
///
/// This test only applies to documents with `/document/category` with value `csaf_security_advisory` and `csaf_vex` for
/// `/document/csaf_version` `2.0` and additionally to documents with `/document/category` with
/// value `csaf_deprecated_security_advisory` for `/document/csaf_version` `2.1`.
///
/// Documents with this category must have a `/product_tree` element.
pub fn test_6_1_27_4_product_tree(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    // check if document is relevant document category in csaf 2.0
    if *doc.get_document().get_csaf_version() == CsafVersion::X20 {
        if doc_category != DocumentCategory::CsafSecurityAdvisory && doc_category != DocumentCategory::CsafVex {
            return Ok(());
        }
    }

    // check if document is relevant document category in csaf 2.1
    if *doc.get_document().get_csaf_version() == CsafVersion::X21 {
        if doc_category != DocumentCategory::CsafSecurityAdvisory
            && doc_category != DocumentCategory::CsafVex
            && doc_category != DocumentCategory::CsafDeprecatedSecurityAdvisory
        {
            return Ok(());
        }
    }

    // return error if there are there isn't a product tree
    if doc.get_product_tree().is_none() {
        return Err(vec![test_6_1_27_4_err_generator(doc_category)]);
    }

    Ok(())
}

fn test_6_1_27_4_err_generator(document_category: DocumentCategory) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with category '{}' must have a '/product_tree' element",
            document_category
        ),
        instance_path: "/product_tree".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::csaf_traits::DocumentCategory;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validations::test_6_1_27_4::{test_6_1_27_4_err_generator, test_6_1_27_4_product_tree};
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_27_4() {
        let errors = HashMap::from([
            (
                "01",
                vec![test_6_1_27_4_err_generator(DocumentCategory::CsafSecurityAdvisory)],
            ),
            ("02", vec![test_6_1_27_4_err_generator(DocumentCategory::CsafVex)]),
            (
                "03",
                vec![test_6_1_27_4_err_generator(
                    DocumentCategory::CsafDeprecatedSecurityAdvisory,
                )],
            ),
        ]);
        run_csaf20_tests("27-04", test_6_1_27_4_product_tree, errors.clone());
        run_csaf21_tests("27-04", test_6_1_27_4_product_tree, errors);
    }
}
