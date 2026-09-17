use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::{CsafTrait, CsafVersion, DocumentTrait};
use crate::validation::{TestFinding, TestFindingData};

/// 6.1.26 Prohibited Document Category Name
pub fn test_6_1_26_prohibited_document_category(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let doc_version = doc.get_document().get_csaf_version();
    let doc_category = doc.get_document().get_category();

    validate_document_category(&doc_category, doc_version)
}

#[inline]
fn validate_document_category(
    doc_category: &CsafDocumentCategory,
    doc_version: CsafVersion,
) -> Result<(), Vec<TestFinding>> {
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
    version: CsafVersion,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Document category '{doc_category}' is prohibited. Only the following values starting with 'csaf_' (or similar) are allowed: {}",
            CsafDocumentCategory::known_profile_concat(version)
        ),
        instance_path: "/document/category".to_string(),
    })
}

fn test_6_1_26_err_generator_too_similar(
    doc_category: &CsafDocumentCategory,
    known_category: &CsafDocumentCategory,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Document category '{doc_category}' is prohibited. It is too similar to the known category: {known_category}",
        ),
        instance_path: "/document/category".to_string(),
    })
}

crate::test_validation::impl_validator!(ValidatorForTest6_1_26, test_6_1_26_prohibited_document_category);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_1_26 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_26 as ExpectedResults_2_1;
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
            CsafVersion::X20,
        )]);
        let case_03_csaf20 = Err(vec![test_6_1_26_err_generator_starts_with_csaf(
            &CsafDocumentCategory::from("Csaf_VeX"),
            CsafVersion::X20,
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
            CsafVersion::X21,
        )]);
        let case_08_csaf21 = Err(vec![test_6_1_26_err_generator_starts_with_csaf(
            &CsafDocumentCategory::from("csaf_BASE"),
            CsafVersion::X21,
        )]);
        let case_09_csaf21 = Err(vec![test_6_1_26_err_generator_too_similar(
            &CsafDocumentCategory::from("Vulnerability＿rePORT"),
            &CsafDocumentCategory::CsafVulnerabilityReport,
        )]);
        let case_21_csaf21 = Err(vec![
            test_6_1_26_err_generator_too_similar(
                &CsafDocumentCategory::from("c\u{0009}\u{000B}\u{000C}\u{FEFF}s\u{0020}\u{00A0}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{200A}\u{202F}\u{205F}\u{3000}af_informat\u{0009}\u{000B}\u{000C}\u{FEFF}ional_adv\u{0020}\u{00A0}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{200A}\u{202F}\u{205F}\u{3000}isory"),
                &CsafDocumentCategory::CsafInformationalAdvisory,
            )
        ]);
        let case_22_csaf21 = Err(vec![
            test_6_1_26_err_generator_too_similar(
                &CsafDocumentCategory::from("cscs\u{002D}\u{02D7}\u{05BE}\u{058A}\u{1400}\u{1806}\u{2010}\u{2011}\u{2012}\u{2013}\u{2014}\u{2015}\u{2043}\u{2053}\u{207B}\u{208B}\u{2212}\u{23AF}\u{23BA}\u{23BB}\u{23BC}\u{23E4}\u{2500}\u{2501}\u{254C}\u{254D}\u{2574}\u{2576}\u{2578}\u{257A}\u{2796}\u{29FF}\u{2E3A}\u{2E3B}\u{301C}\u{FE58}\u{FE63}\u{FF0D}\u{1CC86}af_informa\u{002D}\u{02D7}\u{05BE}\u{058A}\u{1400}\u{1806}\u{2010}\u{2011}\u{2012}\u{2013}\u{2014}\u{2015}\u{2043}\u{2053}\u{207B}\u{208B}\u{2212}\u{23AF}\u{23BA}\u{23BB}\u{23BC}\u{23E4}\u{2500}\u{2501}\u{254C}\u{254D}\u{2574}\u{2576}\u{2578}\u{257A}\u{2796}\u{29FF}\u{2E3A}\u{2E3B}\u{301C}\u{FE58}\u{FE63}\u{FF0D}\u{1CC86}tional_advisory"),
                &CsafDocumentCategory::CsafInformationalAdvisory,
            )
        ]);
        let case_23_csaf21 = Err(vec![
            test_6_1_26_err_generator_too_similar(
                &CsafDocumentCategory::from("c\u{005F}\u{02CD}\u{FF3F}\u{0332}\u{0333}\u{2017}\u{203F}\u{2581}\u{23B5}\u{23BD}\u{FE4D}\u{FE4E}\u{FE4F}sa\u{1F003}\u{1F008}\u{1F00D}\u{1F011}\u{1F012}\u{1F013}\u{1F096}\u{1F097}\u{1F099}f_informa\u{005F}\u{02CD}\u{FF3F}\u{0332}\u{0333}\u{2017}\u{203F}\u{2581}\u{23B5}\u{23BD}\u{FE4D}\u{FE4E}\u{FE4F}tional_advi\u{1F003}\u{1F008}\u{1F00D}\u{1F011}\u{1F012}\u{1F013}\u{1F096}\u{1F097}\u{1F099}sory"),
                &CsafDocumentCategory::CsafInformationalAdvisory,
            )
        ]);
        let case_24_csaf21 = Err(vec![
            test_6_1_26_err_generator_too_similar(
                &CsafDocumentCategory::from("cs\u{00AD}\u{034F}\u{180E}\u{200B}\u{200C}\u{200D}\u{2060}\u{2062}\u{2063}\u{2064}\u{FEFF}af_informat\u{00AD}\u{034F}\u{180E}\u{200B}\u{200C}\u{200D}\u{2060}\u{2062}\u{2063}\u{2064}\u{FEFF}ional_advisory"),
                &CsafDocumentCategory::CsafInformationalAdvisory,
            )
        ]);

        // Supplementary test cases
        // CSAF 2.1 categories that should fail on CSAF 2.0
        let deprecated_sec_advisory_csaf20 = Err(vec![test_6_1_26_err_generator_starts_with_csaf(
            &CsafDocumentCategory::from("csaf_deprecated_security_advisory"),
            CsafVersion::X20,
        )]);
        let withdrawn_csaf20 = Err(vec![test_6_1_26_err_generator_starts_with_csaf(
            &CsafDocumentCategory::from("csaf_withdrawn"),
            CsafVersion::X20,
        )]);
        let superseded_csaf20 = Err(vec![test_6_1_26_err_generator_starts_with_csaf(
            &CsafDocumentCategory::from("csaf_superseded"),
            CsafVersion::X20,
        )]);

        TESTS_2_0.test_6_1_26.expect(ExpectedResults_2_0 {
            case_01: case_01_shared.clone(),
            case_02: case_02_csaf20,
            case_03: case_03_csaf20,
            case_04: case_04_csaf20,
            case_s01: deprecated_sec_advisory_csaf20,
            case_s02: withdrawn_csaf20,
            case_s03: superseded_csaf20,
            case_11: Ok(()),
            case_12: Ok(()),
        });
        TESTS_2_1.test_6_1_26.expect(ExpectedResults_2_1 {
            case_01: case_01_shared,
            case_02: case_02_csaf21,
            case_03: case_03_csaf21,
            case_04: case_04_csaf21,
            case_05: case_05_csaf21,
            case_06: case_06_csaf21,
            case_07: case_07_csaf21,
            case_08: case_08_csaf21,
            case_09: case_09_csaf21,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
            case_14: Ok(()),
            case_15: Ok(()),
            case_16: Ok(()),
            case_21: case_21_csaf21,
            case_22: case_22_csaf21,
            case_23: case_23_csaf21,
            case_24: case_24_csaf21,
        });
    }

    // Additional unit tests from the editorial version of CSAF 2.1
    #[test]
    fn test_validate_document_category_v21() {
        let version = CsafVersion::X21;
        assert!(validate_document_category(&CsafDocumentCategory::from("Csaf_a"), version).is_err());
        assert!(validate_document_category(&CsafDocumentCategory::from("csafvex"), version).is_err());
        assert!(validate_document_category(&CsafDocumentCategory::from("Informational Advisory"), version).is_err());
        assert!(validate_document_category(&CsafDocumentCategory::from("Security      Advisory"), version).is_err());
        assert!(
            validate_document_category(&CsafDocumentCategory::from("security-incident-response"), version).is_err()
        );
        assert!(validate_document_category(&CsafDocumentCategory::from("Superseded"), version).is_err());
        assert!(validate_document_category(&CsafDocumentCategory::from("V_eX"), version).is_err());
        assert!(validate_document_category(&CsafDocumentCategory::from("veX"), version).is_err());
    }
}
