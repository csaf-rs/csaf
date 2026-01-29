use crate::csaf::types::csaf_version_number::{CsafVersionNumber, ValidVersionNumber};
use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionHistorySortable, TrackingTrait};
use crate::schema::csaf2_1::schema::DocumentStatus;
use crate::validation::ValidationError;

/// 6.1.16 Latest Document Version
///
/// `/document/tracking/version` must be equal to the last `/document/tracking/revision_history[]/number` when
/// sorting the revision history ascending by `date`. Build metadata is ignored. Pre-release parts are ignored
/// if `/document/status` is "draft".
pub fn test_6_1_16_latest_document_version(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let tracking = doc.get_document().get_tracking();

    // Check if doc version is valid, if not return an error and skip this test
    let doc_version = match tracking.get_version() {
        CsafVersionNumber::Valid(version_number) => version_number,
        CsafVersionNumber::Invalid(err) => return Err(vec![err.get_validation_error("/document/version")]),
    };

    let mut revision_history = tracking.get_revision_history_tuples();
    revision_history.inplace_sort_by_date_then_number();

    let mut errors: Option<Vec<ValidationError>> = None;
    if let Some(latest_revision_history_item) = revision_history.last() {
        let latest_number = latest_revision_history_item.number.clone();
        // TODO also add validation errors for invalid revision history numbers here
        let doc_status = tracking.get_status();
        match (&latest_number, &doc_version) {
            (ValidVersionNumber::IntVer(last_number), ValidVersionNumber::IntVer(doc_version)) => {
                if doc_version == last_number {
                    return Ok(());
                }
            },
            (ValidVersionNumber::SemVer(last_number), ValidVersionNumber::SemVer(doc_version)) => {
                // Manually compare the semver objs according to test req
                let mut equal = true;
                equal &= equal && doc_version.get_major() == last_number.get_major();
                equal &= equal && doc_version.get_minor() == last_number.get_minor();
                equal &= equal && doc_version.get_patch() == last_number.get_patch();
                if doc_status != DocumentStatus::Draft {
                    equal &= equal && doc_version.get_prerelease() == last_number.get_prerelease();
                }
                if equal {
                    return Ok(());
                }
            },
            // Mixed version number types cannot be equal
            _ => {},
        };
        errors
            .get_or_insert_default()
            .push(test_6_1_16_err_generator(&doc_version, &latest_number, &doc_status));
    }
    errors.map_or(Ok(()), Err)
}

fn test_6_1_16_err_generator(
    doc_version: &ValidVersionNumber,
    latest_number: &ValidVersionNumber,
    doc_status: &DocumentStatus,
) -> ValidationError {
    ValidationError {
        message: format!(
            "The document version '{doc_version}' is not equal to the latest revision history number '{latest_number}' in document with status '{doc_status}'"
        ),
        instance_path: "/document/tracking/version".to_string(),
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_16
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_16_latest_document_version(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_16
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_16_latest_document_version(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;
    use std::str::FromStr;

    #[test]
    fn test_test_6_1_16() {
        // Error cases
        let case_01 = Err(vec![test_6_1_16_err_generator(
            &ValidVersionNumber::from_str("1").unwrap(),
            &ValidVersionNumber::from_str("2").unwrap(),
            &DocumentStatus::Final,
        )]);
        let case_02 = Err(vec![test_6_1_16_err_generator(
            &ValidVersionNumber::from_str("1").unwrap(),
            &ValidVersionNumber::from_str("2").unwrap(),
            &DocumentStatus::Final,
        )]);
        let case_03 = Err(vec![test_6_1_16_err_generator(
            &ValidVersionNumber::from_str("1").unwrap(),
            &ValidVersionNumber::from_str("2").unwrap(),
            &DocumentStatus::Final,
        )]);
        let case_04 = Err(vec![test_6_1_16_err_generator(
            &ValidVersionNumber::from_str("1.0.0").unwrap(),
            &ValidVersionNumber::from_str("2.0.0").unwrap(),
            &DocumentStatus::Final,
        )]);
        let case_05 = Err(vec![test_6_1_16_err_generator(
            &ValidVersionNumber::from_str("1.0.0").unwrap(),
            &ValidVersionNumber::from_str("2.0.0").unwrap(),
            &DocumentStatus::Final,
        )]);
        let case_06 = Err(vec![test_6_1_16_err_generator(
            &ValidVersionNumber::from_str("9").unwrap(),
            &ValidVersionNumber::from_str("10").unwrap(),
            &DocumentStatus::Final,
        )]);
        let case_07 = Err(vec![test_6_1_16_err_generator(
            &ValidVersionNumber::from_str("1.9.0").unwrap(),
            &ValidVersionNumber::from_str("1.10.0").unwrap(),
            &DocumentStatus::Final,
        )]);
        let case_08 = Err(vec![test_6_1_16_err_generator(
            &ValidVersionNumber::from_str("1").unwrap(),
            &ValidVersionNumber::from_str("2").unwrap(),
            &DocumentStatus::Final,
        )]);

        // CSAF 2.0 has 18 test cases (01-08, 11-19, 31)
        TESTS_2_0.test_6_1_16.expect(
            case_01.clone(),
            case_02.clone(),
            case_03.clone(),
            case_04.clone(),
            case_05.clone(),
            case_06.clone(),
            case_07.clone(),
            case_08.clone(),
            Ok(()), // case_11
            Ok(()), // case_12
            Ok(()), // case_13
            Ok(()), // case_14
            Ok(()), // case_15
            Ok(()), // case_16
            Ok(()), // case_17
            Ok(()), // case_18
            Ok(()), // case_19
            Ok(()), // case_31
        );

        // CSAF 2.1 has 20 test cases (01-09, 11-19, 31-32)
        TESTS_2_1.test_6_1_16.expect(
            case_01,
            case_02,
            case_03,
            case_04,
            case_05,
            case_06,
            case_07,
            case_08,
            Err(vec![test_6_1_16_err_generator(
                &ValidVersionNumber::from_str("2").unwrap(),
                &ValidVersionNumber::from_str("1").unwrap(),
                &DocumentStatus::Final,
            )]),
            Ok(()), // case_11
            Ok(()), // case_12
            Ok(()), // case_13
            Ok(()), // case_14
            Ok(()), // case_15
            Ok(()), // case_16
            Ok(()), // case_17
            Ok(()), // case_18
            Ok(()), // case_19
            Ok(()), // case_31
            Ok(()), // case_32
        );
    }
}
