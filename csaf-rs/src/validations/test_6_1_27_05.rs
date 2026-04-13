use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, DocumentTrait, VulnerabilityTrait};
use crate::document_category_test_helper::DocumentCategoryTestConfig;
use crate::validation::ValidationError;

/// 6.1.27.5 Vulnerability Notes
///
/// This test only applies to documents with `/document/category` with value `csaf_security_advisory` and `csaf_vex` for
/// `/document/csaf_version` `2.0` and additionally to documents with `/document/category` with
/// value `csaf_deprecated_security_advisory` for `/document/csaf_version` `2.1`.
///
/// Documents with these categories must have a `/vulnerabilities[]/notes` element.
pub fn test_6_1_27_05_vulnerability_notes(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    if !PROFILE_TEST_CONFIG.matches_category_with_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(()); // ToDo generate skipped https://github.com/csaf-rs/csaf/issues/409
    }

    let mut errors: Option<Vec<ValidationError>> = None;
    // return error if there are vulnerabilities without notes
    for (v_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if vulnerability.get_notes().is_none() {
            errors
                .get_or_insert_default()
                .push(test_6_1_27_05_err_generator(&doc_category, &v_i));
        }
    }

    errors.map_or(Ok(()), Err)
}

const PROFILE_TEST_CONFIG: DocumentCategoryTestConfig = DocumentCategoryTestConfig::new()
    .shared(&[
        CsafDocumentCategory::CsafSecurityAdvisory,
        CsafDocumentCategory::CsafVex,
    ])
    .csaf21(&[CsafDocumentCategory::CsafDeprecatedSecurityAdvisory]);

fn test_6_1_27_05_err_generator(document_category: &CsafDocumentCategory, vuln_path_index: &usize) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with category '{document_category}' must have a notes element in each vulnerability"
        ),
        instance_path: format!("/vulnerabilities/{vuln_path_index}/notes"),
    }
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_27_5, test_6_1_27_05_vulnerability_notes);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_05() {
        let case_security_advisory = Err(vec![test_6_1_27_05_err_generator(
            &CsafDocumentCategory::CsafSecurityAdvisory,
            &0,
        )]);
        let case_vex = Err(vec![test_6_1_27_05_err_generator(&CsafDocumentCategory::CsafVex, &0)]);
        let case_deprecated_security_advisory: Result<(), Vec<ValidationError>> =
            Err(vec![test_6_1_27_05_err_generator(
                &CsafDocumentCategory::CsafDeprecatedSecurityAdvisory,
                &0,
            )]);

        TESTS_2_0
            .test_6_1_27_5
            .expect(case_security_advisory.clone(), case_vex.clone());
        TESTS_2_1
            .test_6_1_27_5
            .expect(case_security_advisory, case_vex, case_deprecated_security_advisory);
    }
}
