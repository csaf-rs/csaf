use crate::csaf::types::version_number::CsafVersionNumber;
use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionHistorySortable, TrackingTrait};
use crate::schema::csaf2_1::schema::DocumentStatus;
use crate::validation::ValidationError;

/// 6.1.21 Missing Item in Revision History
///
/// When ordered by their `date` field, all `/document/tracking/revision_history[]` items need to contain
/// all integers in the range between the `number` of first revision history and the last revision history.
/// Also, it has to be ensured that the first item has either a version 0 or 1.
/// This applies to the version number for integer versioning and to the major version for semantic versioning.
pub fn test_6_1_21_missing_item_in_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    // Generate and sort the revision history tuples by date first and by number second
    let mut rev_history_tuples = doc.get_document().get_tracking().get_revision_history_tuples();
    rev_history_tuples.inplace_sort_by_date_then_number();

    let first_tuple = if let Some(first) = rev_history_tuples.first() {
        first
    } else {
        return Ok(()); // ToDo #409 this should be Skipped: Precondition failed
    };

    // Throw error if first version is not 0 or 1
    if first_tuple.number.get_major() > 1 {
        errors
            .get_or_insert_default()
            .push(test_6_1_21_err_wrong_first_version_generator(
                first_tuple.number.clone(),
                &first_tuple.path_index,
            ));
    }

    let mut start_of_sequence = get_first_expected_version(
        doc.get_document().get_tracking().get_status(),
        doc.get_document().get_tracking().get_version(),
    );
    // get the maximum version number to find all missing versions in between
    let max_number = rev_history_tuples
        .iter()
        .map(|item| item.number.get_major())
        .max()
        .unwrap();
    let mut expected_version = start_of_sequence.clone();
    while start_of_sequence.get_major() < max_number {
        // expected_version = expected_version.clone().get_next_major_version();

        // search for the expected version in the revision history
        // this ignores ordering problems, because they are tested by 6.1.14
        let found_expected_version = rev_history_tuples
            .iter()
            .map(|item| &item.number)
            .find(|number| number.get_major() == expected_version.get_major())
            .is_some();

        if found_expected_version {
            if start_of_sequence == expected_version {
                // no version missing, continue with next expected version
            } else {
                let is_only_one_version_missing = start_of_sequence.get_next_major_version() == expected_version;
                if is_only_one_version_missing {
                    // only 1 version missing
                    errors
                        .get_or_insert_default()
                        .push(test_6_1_21_err_missing_version(start_of_sequence.clone()));
                } else {
                    // multiple versions missing
                    errors
                        .get_or_insert_default()
                        .push(test_6_1_21_err_missing_version_range(
                            start_of_sequence.clone(),
                            expected_version.get_previous_major_version(),
                        ));
                }
            }
            expected_version = expected_version.clone().get_next_major_version();
            start_of_sequence = expected_version.clone();
        } else {
            expected_version = expected_version.get_next_major_version();
        }
    }

    errors.map_or(Ok(()), Err)
}

/// Get the first expected version number for a document
/// - 0 for draft and interim documents
/// - 1 for final documents
///
/// while respecting the versioning scheme from document version.
fn get_first_expected_version(status: DocumentStatus, document_version: CsafVersionNumber) -> CsafVersionNumber {
    match (status, document_version) {
        (DocumentStatus::Draft | DocumentStatus::Interim, CsafVersionNumber::IntVer(_)) => CsafVersionNumber::from("0"),
        (DocumentStatus::Final, CsafVersionNumber::IntVer(_)) => CsafVersionNumber::from("1"),
        (DocumentStatus::Draft | DocumentStatus::Interim, CsafVersionNumber::SemVer(_)) => {
            CsafVersionNumber::from("0.0.0")
        },
        (DocumentStatus::Final, CsafVersionNumber::SemVer(_)) => CsafVersionNumber::from("1.0.0"),
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_0::testcases::ValidatorForTest6_1_21
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_21_missing_item_in_revision_history(doc)
    }
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_21
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_21_missing_item_in_revision_history(doc)
    }
}

fn test_6_1_21_err_wrong_first_version_generator(
    version: CsafVersionNumber,
    revision_index: &usize,
) -> ValidationError {
    let version_error = match version {
        CsafVersionNumber::IntVer(_) => "integer version of 0 or 1",
        CsafVersionNumber::SemVer(_) => "semver version of 0.y.z or 1.y.z",
    }
    .to_string();
    ValidationError {
        message: format!("The first revision history item should have {version_error}, but was {version}"),
        instance_path: format!("/document/tracking/revision_history/{revision_index}"),
    }
}

fn test_6_1_21_err_missing_version(expected_version: CsafVersionNumber) -> ValidationError {
    let expected_number = expected_version.get_major();
    let version_error = match expected_version {
        CsafVersionNumber::IntVer(_) => format!("{expected_number}"),
        CsafVersionNumber::SemVer(_) => format!("{expected_number}.y.z"),
    }
    .to_string();
    ValidationError {
        message: format!("Missing revision history item {version_error}"),
        instance_path: "/document/tracking/revision_history".to_string(),
    }
}

fn test_6_1_21_err_missing_version_range(from: CsafVersionNumber, to: CsafVersionNumber) -> ValidationError {
    ValidationError {
        message: format!("Missing revision history items {from} to {to}."),
        instance_path: "/document/tracking/revision_history".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_21() {
        let case_intver_1_3_missing_2 = Err(vec![test_6_1_21_err_missing_version(CsafVersionNumber::from("2"))]);
        let case_intver_2_3_missing_1 = Err(vec![
            test_6_1_21_err_wrong_first_version_generator(CsafVersionNumber::from("2"), &0),
            test_6_1_21_err_missing_version(CsafVersionNumber::from("1")),
        ]);
        let case_semver_1_3_missing_2 = Err(vec![test_6_1_21_err_missing_version(CsafVersionNumber::from("2.0.0"))]);
        let case_semver_2_3_missing_1 = Err(vec![
            test_6_1_21_err_wrong_first_version_generator(CsafVersionNumber::from("2.0.0"), &0),
            test_6_1_21_err_missing_version(CsafVersionNumber::from("1.0.0")),
        ]);
        let case_semver_missing_2 = Err(vec![test_6_1_21_err_missing_version(CsafVersionNumber::from("2.0.0"))]);
        let case_semver_multiple_single_versions_and_range_missing = Err(vec![
            test_6_1_21_err_missing_version(CsafVersionNumber::from("2.0.0")),
            test_6_1_21_err_missing_version(CsafVersionNumber::from("4.0.0")),
            test_6_1_21_err_missing_version_range(CsafVersionNumber::from("6.0.0"), CsafVersionNumber::from("7.0.0")),
        ]);
        let case_big_range_missing = Err(vec![test_6_1_21_err_missing_version_range(
            CsafVersionNumber::from("2.0.0"),
            CsafVersionNumber::from("99.0.0"),
        )]);
        let case_semver_first_version_mismatch_multiple_versions_missing = Err(vec![
            test_6_1_21_err_wrong_first_version_generator(CsafVersionNumber::from("3.0.0"), &1),
            test_6_1_21_err_missing_version(CsafVersionNumber::from("2.0.0")),
            test_6_1_21_err_missing_version_range(CsafVersionNumber::from("4.0.0"), CsafVersionNumber::from("7.0.0")),
        ]);
        let case_intver_first_version_mismatch_range_missing = Err(vec![
            test_6_1_21_err_wrong_first_version_generator(CsafVersionNumber::from("4"), &0),
            test_6_1_21_err_missing_version_range(CsafVersionNumber::from("1"), CsafVersionNumber::from("3")),
        ]);

        // Valid cases for both 2.0 and 2.1
        // case 11: valid intver final start with 1
        // case 12: valid intver draft start with 0
        // case 13: valid semver final start with 1.0.0

        TESTS_2_0.test_6_1_21.expect(
            case_intver_1_3_missing_2.clone(),
            case_intver_2_3_missing_1.clone(),
            case_semver_1_3_missing_2.clone(),
            case_semver_2_3_missing_1.clone(),
            case_semver_missing_2.clone(),
            case_semver_multiple_single_versions_and_range_missing.clone(),
            case_big_range_missing.clone(),
            case_semver_first_version_mismatch_multiple_versions_missing.clone(),
            case_intver_first_version_mismatch_range_missing.clone(),
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()),
        );

        let case_intver_1_3_4_with_timezone_missing_2 =
            Err(vec![test_6_1_21_err_missing_version(CsafVersionNumber::from("2"))]);

        TESTS_2_1.test_6_1_21.expect(
            case_intver_1_3_missing_2,
            case_intver_2_3_missing_1,
            case_intver_1_3_4_with_timezone_missing_2,
            case_semver_1_3_missing_2,
            case_semver_2_3_missing_1,
            case_semver_missing_2,
            case_semver_multiple_single_versions_and_range_missing,
            case_big_range_missing,
            case_semver_first_version_mismatch_multiple_versions_missing,
            case_intver_first_version_mismatch_range_missing,
            Ok(()),
            Ok(()),
            Ok(()),
            Ok(()), // valid intver with timezones
            Ok(()),
        );
    }
}
