use crate::csaf_traits::{CsafTrait, DocumentCategory, DocumentTrait, VulnerabilityTrait};
use crate::profile_test_helper::ProfileTestConfig;
use crate::validation::ValidationError;

/// 6.1.27.6 Product Status
///
/// This test only applies to documents with `/document/category` with value `csaf_security_advisory` for
/// `/document/csaf_version` `2.0` and additionally to documents with `/document/category` with
/// value `csaf_deprecated_security_advisory` for `/document/csaf_version` `2.1`.
///
/// Documents with these categories must have a `/vulnerabilities[]/product_status` element.
pub fn test_6_1_27_06_product_status(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_category = doc.get_document().get_category();

    if PROFILE_TEST_CONFIG.is_ignored_for_on_csaf_version(doc.get_document().get_csaf_version(), &doc_category) {
        return Ok(());
    }

    let mut errors: Option<Vec<ValidationError>> = None;
    // return error if there are vulnerabilities without product_status
    for (v_i, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        if vulnerability.get_product_status().is_none() {
            errors
                .get_or_insert_with(Vec::new)
                .push(test_6_1_27_06_err_generator(&doc_category, &v_i));
        }
    }

    errors.map_or(Ok(()), Err)
}

const PROFILE_TEST_CONFIG: ProfileTestConfig = ProfileTestConfig::new()
    .shared(&[DocumentCategory::CsafSecurityAdvisory])
    .csaf21(&[DocumentCategory::CsafDeprecatedSecurityAdvisory]);

fn test_6_1_27_06_err_generator(document_category: &DocumentCategory, vuln_path_index: &usize) -> ValidationError {
    ValidationError {
        message: format!(
            "Document with category '{}' must have a product_status element in each vulnerability",
            document_category
        ),
        instance_path: format!("/vulnerabilities/{}/product_status", vuln_path_index),
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_27_6
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_06_product_status(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_27_6
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_27_06_product_status(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_06() {
        let case_01 = Err(vec![test_6_1_27_06_err_generator(
            &DocumentCategory::CsafSecurityAdvisory,
            &0,
        )]);

        TESTS_2_0.test_6_1_27_6.expect(case_01.clone());
        TESTS_2_1.test_6_1_27_6.expect(
            case_01,
            Err(vec![test_6_1_27_06_err_generator(
                &DocumentCategory::CsafSecurityAdvisory,
                &0,
            )]),
            Err(vec![test_6_1_27_06_err_generator(
                &DocumentCategory::CsafDeprecatedSecurityAdvisory,
                &0,
            )]),
        );
    }
}
