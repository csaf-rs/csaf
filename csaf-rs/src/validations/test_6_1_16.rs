use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionHistorySortable, TrackingTrait, VersionNumber};
use crate::schema::csaf2_1::schema::DocumentStatus;
use crate::validation::ValidationError;

/// 6.1.16 Latest Document Version
///
/// `/document/tracking/version` must be equal to the last `/document/tracking/revision_history[]/number` when
/// sorting the revision history ascending by `date`. Build metadata is ignored. Pre-release parts are ignored
/// if `/document/status` is "draft".
pub fn test_6_1_16_latest_document_version(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let document = doc.get_document();

    let mut revision_history = doc.get_document().get_tracking().get_revision_history_tuples();
    revision_history.inplace_sort_by_date_then_number();

    if let Some(latest_revision_history_item) = revision_history.last() {
        let latest_number = &latest_revision_history_item.number;
        let doc_version = document.get_tracking().get_version();
        let doc_status = document.get_tracking().get_status();
        match latest_number {
            VersionNumber::Integer(_) => {
                // We can use the default eq here, as intver has no pre-release or build metadata
                // and the eq will return false if comparing intver with semver
                if doc_version == *latest_number {
                    return Ok(());
                }
            },
            VersionNumber::Semver(latest) => {
                // Manually check if comparing with intver
                if let VersionNumber::Semver(ref version) = doc_version {
                    // Manually compare the semver objs according to test req
                    let mut equal = true;
                    equal &= equal && version.major == latest.major;
                    equal &= equal && version.minor == latest.minor;
                    equal &= equal && version.patch == latest.patch;
                    if doc_status != DocumentStatus::Draft {
                        equal &= equal && version.pre == latest.pre;
                    }
                    if equal {
                        return Ok(());
                    }
                }
            },
        };

        return Err(vec![test_6_1_16_err_generator(
            doc_version.to_string(),
            latest_number.to_string(),
            doc_status.to_string(),
        )]);
    }

    // This should not be able to happen as revision history is a required property with 1..* items
    panic!("Revision history is empty, document is malformed.");
}

fn test_6_1_16_err_generator(doc_version: String, latest_number: String, doc_status: String) -> ValidationError {
    ValidationError {
        message: format!(
            "The document version '{}' is not equal to the latest revision history number '{}' in document with status '{}'",
            doc_version, latest_number, doc_status
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

    #[test]
    fn test_test_6_1_16() {
        // Error cases
        let case_01 = Err(vec![test_6_1_16_err_generator(
            "1".to_string(),
            "2".to_string(),
            "final".to_string(),
        )]);
        let case_02 = Err(vec![test_6_1_16_err_generator(
            "1".to_string(),
            "2".to_string(),
            "final".to_string(),
        )]);
        let case_03 = Err(vec![test_6_1_16_err_generator(
            "1".to_string(),
            "2".to_string(),
            "final".to_string(),
        )]);
        let case_04 = Err(vec![test_6_1_16_err_generator(
            "1.0.0".to_string(),
            "2.0.0".to_string(),
            "final".to_string(),
        )]);
        let case_05 = Err(vec![test_6_1_16_err_generator(
            "1.0.0".to_string(),
            "2.0.0".to_string(),
            "final".to_string(),
        )]);
        let case_06 = Err(vec![test_6_1_16_err_generator(
            "9".to_string(),
            "10".to_string(),
            "final".to_string(),
        )]);
        let case_07 = Err(vec![test_6_1_16_err_generator(
            "1.9.0".to_string(),
            "1.10.0".to_string(),
            "final".to_string(),
        )]);
        let case_08 = Err(vec![test_6_1_16_err_generator(
            "1".to_string(),
            "2".to_string(),
            "final".to_string(),
        )]);
        let case_09 = Err(vec![test_6_1_16_err_generator(
            "2".to_string(),
            "1".to_string(),
            "final".to_string(),
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
            case_09,
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
