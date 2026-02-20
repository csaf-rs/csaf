use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, CsafVersion, DocumentTrait};
use crate::validation::ValidationError;

/// 6.1.26 Prohibited Document Category Name
pub fn test_6_1_26_prohibited_document_category(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let doc_version = doc.get_document().get_csaf_version();
    let doc_category = doc.get_document().get_category();

    validate_document_category(&doc_category, doc_version)
}

#[inline]
fn validate_document_category(
    doc_category: &CsafDocumentCategory,
    doc_version: &CsafVersion,
) -> Result<(), Vec<ValidationError>> {
    // skip test for known profiles and categories
    if doc_category.is_known_profile(doc_version) {
        return Ok(());
    }

    // throw error, as only known profiles are allowed to start with "csaf_"
    if doc_category.starts_with_csaf_underscore() {
        return Err(vec![test_6_1_26_err_generator_starts_with_csaf(
            doc_category,
            doc_version,
        )]);
    }

    // normalize and compare against known profiles
    let normalized_doc_category = doc_category.normalize();
    for (normalized_known_category, known_category) in CsafDocumentCategory::known_profiles_normalized(doc_version) {
        if normalized_doc_category == normalized_known_category {
            return Err(vec![test_6_1_26_err_generator_too_similar(
                doc_category,
                &known_category,
            )]);
        }
    }

    Ok(())
}

fn test_6_1_26_err_generator_starts_with_csaf(
    doc_category: &CsafDocumentCategory,
    version: &CsafVersion,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Document category '{doc_category}' is prohibited. Only the following values starting with 'csaf_' are allowed: {}",
            CsafDocumentCategory::known_profile_concat(version)
        ),
        instance_path: "/document/category".to_string(),
    }
}

fn test_6_1_26_err_generator_too_similar(
    doc_category: &CsafDocumentCategory,
    known_category: &CsafDocumentCategory,
) -> ValidationError {
    ValidationError {
        message: format!(
            "Document category '{doc_category}' is prohibited. It is too similar to the known category: {known_category}",
        ),
        instance_path: "/document/category".to_string(),
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_26
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_26_prohibited_document_category(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_26
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_26_prohibited_document_category(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_26() {
        // Shared test cases
        let case_01_shared = Err(vec![test_6_1_26_err_generator_too_similar(
            &CsafDocumentCategory::from("Security_Incident_Response"),
            &CsafDocumentCategory::CsafSecurityIncidentResponse,
        )]);

        // CSAF 2.0 specific test cases
        let case_02_csaf20 = Err(vec![test_6_1_26_err_generator_starts_with_csaf(
            &CsafDocumentCategory::from("csaf_BASE"),
            &CsafVersion::X20,
        )]);
        let case_03_csaf20 = Err(vec![test_6_1_26_err_generator_starts_with_csaf(
            &CsafDocumentCategory::from("Csaf_VeX"),
            &CsafVersion::X20,
        )]);
        let case_04_csaf20 = Err(vec![test_6_1_26_err_generator_too_similar(
            &CsafDocumentCategory::from("csafsecurityadvisory"),
            &CsafDocumentCategory::CsafSecurityAdvisory,
        )]);

        // CSAF 2.1 specific test cases
        let case_02_csaf21 = Err(vec![test_6_1_26_err_generator_too_similar(
            &CsafDocumentCategory::from("Deprecated Security Advisory"),
            &CsafDocumentCategory::CsafDeprecatedSecurityAdvisory,
        )]);
        let case_03_csaf21 = Err(vec![test_6_1_26_err_generator_too_similar(
            &CsafDocumentCategory::from("withdrawn"),
            &CsafDocumentCategory::CsafWithdrawn,
        )]);
        let case_04_csaf21 = Err(vec![test_6_1_26_err_generator_too_similar(
            &CsafDocumentCategory::from("superseded"),
            &CsafDocumentCategory::CsafSuperseded,
        )]);
        let case_05_csaf21 = Err(vec![test_6_1_26_err_generator_too_similar(
            &CsafDocumentCategory::from("csafvex"),
            &CsafDocumentCategory::CsafVex,
        )]);
        let case_06_csaf21 = Err(vec![test_6_1_26_err_generator_too_similar(
            &CsafDocumentCategory::from("CSafDeprecatedSecurity—Advisory"),
            &CsafDocumentCategory::CsafDeprecatedSecurityAdvisory,
        )]);
        let case_07_csaf21 = Err(vec![test_6_1_26_err_generator_starts_with_csaf(
            &CsafDocumentCategory::from("CsaF_VeX"),
            &CsafVersion::X21,
        )]);
        let case_08_csaf21 = Err(vec![test_6_1_26_err_generator_starts_with_csaf(
            &CsafDocumentCategory::from("csaf_BASE"),
            &CsafVersion::X21,
        )]);

        TESTS_2_0.test_6_1_26.expect(
            case_01_shared.clone(),
            case_02_csaf20,
            case_03_csaf20,
            case_04_csaf20,
            Ok(()),
            Ok(()),
        );
        TESTS_2_1.test_6_1_26.expect(
            case_01_shared,
            case_02_csaf21,
            case_03_csaf21,
            case_04_csaf21,
            case_05_csaf21,
            case_06_csaf21,
            case_07_csaf21,
            case_08_csaf21,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }

    // Additional unit tests from the editorial version of CSAF 2.1
    #[test]
    fn test_validate_document_category_v21() {
        let version = CsafVersion::X21;
        assert!(validate_document_category(&CsafDocumentCategory::from("Csaf_a"), &version).is_err());
        assert!(validate_document_category(&CsafDocumentCategory::from("csafvex"), &version).is_err());
        assert!(validate_document_category(&CsafDocumentCategory::from("Informational Advisory"), &version).is_err());
        assert!(validate_document_category(&CsafDocumentCategory::from("Security      Advisory"), &version).is_err());
        assert!(
            validate_document_category(&CsafDocumentCategory::from("security-incident-response"), &version).is_err()
        );
        assert!(validate_document_category(&CsafDocumentCategory::from("Superseded"), &version).is_err());
        assert!(validate_document_category(&CsafDocumentCategory::from("V_eX"), &version).is_err());
        assert!(validate_document_category(&CsafDocumentCategory::from("veX"), &version).is_err());
    }

    #[test]
    fn test_csaf_21_known_categories_fail_on_csaf_20() {
        let version_20 = CsafVersion::X20;
        assert!(
            validate_document_category(&CsafDocumentCategory::CsafDeprecatedSecurityAdvisory, &version_20).is_err()
        );
        assert!(validate_document_category(&CsafDocumentCategory::CsafWithdrawn, &version_20).is_err());
        assert!(validate_document_category(&CsafDocumentCategory::CsafSuperseded, &version_20).is_err());
    }
}
