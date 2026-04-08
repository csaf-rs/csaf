use std::fmt::{Display, Formatter};
use crate::csaf::types::language::CsafLanguage;
use crate::csaf::types::language::valid_language::PrivateUseReason;
use crate::csaf_traits::{CsafTrait, DocumentTrait};
use crate::validation::ValidationError;

/// 6.2.14 Use of Private Language
///
/// For each element of type `/$defs/lang_t` it MUST be tested that the language code does not
/// contain subtags reserved for private use.
pub fn test_6_2_14_use_of_private_language(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();

    if document.get_lang().is_none() && document.get_source_lang().is_none() {
        return Ok(()); // This should be a wasSkipped later (see #409)
    }

    let mut errors: Option<Vec<ValidationError>> = None;

    validate_private_language(document.get_lang(), "/document/lang", &mut errors);
    validate_private_language(document.get_source_lang(), "/document/source_lang", &mut errors);

    errors.map_or(Ok(()), Err)
}

/// Helper function to validate a `lang` tag and check if it contains subtags reserved for private use.
///
/// If the optional language tag is `Some`, is a valid tag, and contains private use subtags,
/// an error will be added to `errors` vector.
///
/// # Arguments
/// - `lang`: The (optional) language tag to validate
/// - `json_path`: The JSON path to the language tag
/// - `errors`: A mutable reference to the errors vector
fn validate_private_language(lang: Option<CsafLanguage>, json_path: &str, errors: &mut Option<Vec<ValidationError>>) {
    if let Some(CsafLanguage::Valid(valid_lang)) = lang
        && let Some(private_use_reasons) = valid_lang.get_private_use()
    {
        errors.get_or_insert_default().push(create_private_language_error_from_reasons(
            valid_lang.as_str().to_string(),
            &private_use_reasons,
            json_path,
        ));
    }
}

/// Keeping this in, if we ever want less "detailed" error messages.
#[allow(dead_code)]
fn create_private_language_error(lang_tag: String, instance_path: &str) -> ValidationError {
    ValidationError {
        message: format!("The language tag '{lang_tag}' contains subtags reserved for private use"),
        instance_path: instance_path.to_string(),
    }
}

// Display specific to this test
impl Display for PrivateUseReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrivateUseReason::PrivateUseSubtag(subtag) => write!(f, "Subtag '{subtag}' is a private use subtag"),
            PrivateUseReason::PrivateUsePrimaryLangSubtag(primary_lang) => write!(f, "Primary Language Subtag '{primary_lang}' is private use"),
            PrivateUseReason::PrivateUseScriptSubtag(script) => write!(f, "Script subtag '{script}' is private use"),
            PrivateUseReason::PrivateUseRegionSubtag(region) => write!(f, "Region subtag '{region}' is private use"),
        }
    }
}

fn create_private_language_error_from_reasons(lang_tag: String, reasons: &Vec<PrivateUseReason>, instance_path: &str) -> ValidationError {
    // Reasons are constructed in a sorted order, so no sorting is necessary here
    let reasons_str = reasons.iter().map(|reason| reason.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    ValidationError {
        message: format!("The language tag '{lang_tag}' contains subtags reserved for private use: {reasons_str}"),
        instance_path: instance_path.to_string()
    }
}

crate::test_validation::impl_validator!(ValidatorForTest6_2_14, test_6_2_14_use_of_private_language);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_14() {
        let case_01_private_primary_lang = Err(vec![create_private_language_error_from_reasons(
            "qtx".to_string(),
            &vec![PrivateUseReason::PrivateUsePrimaryLangSubtag("qtx".to_string())],
            "/document/lang",
        )]);
        let case_02_private_primary_source_lang = Err(vec![create_private_language_error_from_reasons(
            "qcb".to_string(),
            &vec![PrivateUseReason::PrivateUsePrimaryLangSubtag("qcb".to_string())],
            "/document/source_lang",
        )]);
        let case_03_both_private_primary_lang = Err(vec![
            create_private_language_error_from_reasons(
                "qdq".to_string(),
                &vec![PrivateUseReason::PrivateUsePrimaryLangSubtag("qdq".to_string())],
                "/document/lang",
            ),
            create_private_language_error_from_reasons(
                "qcb".to_string(),
                &vec![PrivateUseReason::PrivateUsePrimaryLangSubtag("qcb".to_string())],
                "/document/source_lang",
            ),
        ]);

        let case_04_private_region_qm = Err(vec![create_private_language_error_from_reasons(
            "en-QM".to_string(),
            &vec![PrivateUseReason::PrivateUseRegionSubtag("QM".to_string())],
            "/document/lang",
        )]);

        let case_05_private_region_xp = Err(vec![create_private_language_error_from_reasons(
            "en-XP".to_string(),
            &vec![PrivateUseReason::PrivateUseRegionSubtag("XP".to_string())],
            "/document/lang",
        )]);

        let case_06_private_script_qabc = Err(vec![create_private_language_error_from_reasons(
            "en-Qabc".to_string(),
            &vec![PrivateUseReason::PrivateUseScriptSubtag("Qabc".to_string())],
            "/document/lang",
        )]);

        let case_07_private_region_aa = Err(vec![create_private_language_error_from_reasons(
            "en-AA".to_string(),
            &vec![PrivateUseReason::PrivateUseRegionSubtag("AA".to_string())],
            "/document/lang",
        )]);

        let case_08_private_region_zz = Err(vec![create_private_language_error_from_reasons(
            "fr-ZZ".to_string(),
            &vec![PrivateUseReason::PrivateUseRegionSubtag("ZZ".to_string())],
            "/document/lang",
        )]);

        let case_s01_private_use_tag = Err(vec![create_private_language_error_from_reasons(
            "en-x-this-is-private".to_string(),
            &vec![PrivateUseReason::PrivateUseSubtag("x-this-is-private".to_string())],
            "/document/lang",
        )]);

        let case_s02_multiple_reasons = Err(vec![create_private_language_error_from_reasons(
            "qtx-Qabc-XP-x-this-is-private".to_string(),
            &vec![
                PrivateUseReason::PrivateUsePrimaryLangSubtag("qtx".to_string()),
                PrivateUseReason::PrivateUseScriptSubtag("Qabc".to_string()),
                PrivateUseReason::PrivateUseRegionSubtag("XP".to_string()),
                PrivateUseReason::PrivateUseSubtag("x-this-is-private".to_string()),
            ],
            "/document/lang",
        )]);

        // Case 11: /document/lang is set to a non-private language
        // Case 12: Both are set to non-private languages

        TESTS_2_0.test_6_2_14.expect(
            case_01_private_primary_lang.clone(),
            case_02_private_primary_source_lang.clone(),
            case_03_both_private_primary_lang.clone(),
            case_04_private_region_qm.clone(),
            case_05_private_region_xp.clone(),
            case_06_private_script_qabc.clone(),
            case_07_private_region_aa.clone(),
            case_08_private_region_zz.clone(),
            case_s01_private_use_tag.clone(),
            case_s02_multiple_reasons.clone(),
            Ok(()),
            Ok(()),
        );
        TESTS_2_1.test_6_2_14.expect(
            case_01_private_primary_lang,
            case_02_private_primary_source_lang,
            case_03_both_private_primary_lang,
            case_04_private_region_qm,
            case_05_private_region_xp,
            case_06_private_script_qabc,
            case_07_private_region_aa,
            case_08_private_region_zz,
            case_s01_private_use_tag,
            case_s02_multiple_reasons,
            Ok(()),
            Ok(()),
        );
    }
}
