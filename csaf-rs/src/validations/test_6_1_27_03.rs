use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::ValidationError;
use crate::validations::utils::document_category_test_config::DocumentCategoryTestConfig;

fn create_must_not_have_vuln_element_error(doc_category: &CsafDocumentCategory) -> ValidationError {
    ValidationError {
        message: format!("Document with category '{doc_category}' must not have a '/vulnerabilities' element"),
        instance_path: "/vulnerabilities".to_string(),
    }
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig = DocumentCategoryTestConfig::new()
    .shared(&[CsafDocumentCategory::CsafInformationalAdvisory])
    .csaf21(&[
        CsafDocumentCategory::CsafWithdrawn,
        CsafDocumentCategory::CsafSuperseded,
    ]);

/// 6.1.27.3 Vulnerabilities
///
/// This test only applies to documents with `/document/category` with value `csaf_informational_advisory` for
/// `/document/csaf_version` `2.0` and additionally to documents with `/document/category` with
/// value `csaf_withdrawn` and `csaf_superseded` for `/document/csaf_version` `2.1`.
///
/// Documents with this category must not have a `/vulnerabilities` element.
pub fn test_6_1_27_03_vulnerability(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();
    // check if document has a relevant category for this test
    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(()); // ToDo generate skipped https://github.com/csaf-rs/csaf/issues/409
    }

    // return error if there are elements in /vulnerabilities
    if !doc.get_vulnerabilities().is_empty() {
        return Err(vec![create_must_not_have_vuln_element_error(&doc_category)]);
    }

    Ok(())
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_27_3, test_6_1_27_03_vulnerability);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_03() {
        let case_informational_advisory = Err(vec![create_must_not_have_vuln_element_error(
            &CsafDocumentCategory::CsafInformationalAdvisory,
        )]);
        let case_withdrawn = Err(vec![create_must_not_have_vuln_element_error(
            &CsafDocumentCategory::CsafWithdrawn,
        )]);
        let case_superseded = Err(vec![create_must_not_have_vuln_element_error(
            &CsafDocumentCategory::CsafSuperseded,
        )]);
        TESTS_2_0.test_6_1_27_3.expect(case_informational_advisory.clone());
        TESTS_2_1.test_6_1_27_3.expect(
            case_informational_advisory,
            case_withdrawn,
            case_superseded,
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}
