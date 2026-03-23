use crate::csaf::types::version_number::CsafVersionNumber;
use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionHistorySortable, TrackingTrait};
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

    // We can safely unwrap here, as there has to be at least one item in rev_history_tuples
    let first_tuple = rev_history_tuples.first().unwrap();
    let first_version = &first_tuple.number;
    let first_number = first_version.get_major();

    // Throw error if first version is not 0 or 1
    if first_number > 1 {
        return Err(vec![test_6_1_21_err_wrong_first_version_generator(
            first_version,
            &first_tuple.path_index,
        )]);
    }

    // let expected_last_number = rev_history_tuples.len() as u64 + first_number - 1;
    // let previous_number = first_number;
    // for revision_history_item in rev_history_tuples.iter().skip(1) {
    //     let current_number = revision_history_item.number.get_major();
    //     let expected_number = previous_number + 1;
    //     if current_number != expected_number {
    //         errors
    //             .get_or_insert_with(Vec::new)
    //             .push(test_6_1_21_err_missing_version_in_range(
    //                 &revision_history_item.number,
    //                 &expected_number,
    //             ));
    //     }
    // }

    // old

    // get the maximum version number to find all missing versions in between
    let max_number = rev_history_tuples
        .iter()
        .map(|item| item.number.get_major())
        .max()
        .unwrap();

    for expected_number in first_number + 1..max_number {
        let mut found = false;
        // search for the expected version in the revision history
        // this ignores ordering problems, because they are tested by 6.1.14
        for revision_history_item in rev_history_tuples.iter() {
            if revision_history_item.number.clone().get_major() == expected_number {
                found = true;
                break;
            }
        }
        if !found {
            // We can just take the first tuple here, they are the same anyway (or violate 6.1.30)
            errors
                .get_or_insert_with(Vec::new)
                .push(test_6_1_21_err_missing_version_in_range(
                    &first_version.get_next_major_version(),
                ));
        }
    }
    errors.map_or(Ok(()), Err)
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
    version: &CsafVersionNumber,
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

fn test_6_1_21_err_missing_version_in_range(expected_version: &CsafVersionNumber) -> ValidationError {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_21() {
        let case_intver_1_3_missing_2 = Err(vec![test_6_1_21_err_missing_version_in_range(
            &CsafVersionNumber::from("2"),
        )]);
        let case_intver_2_3_missing_1 = Err(vec![test_6_1_21_err_wrong_first_version_generator(
            &CsafVersionNumber::from("2"),
            &0,
        )]);
        let case_semver_1_3_missing_2 = Err(vec![test_6_1_21_err_missing_version_in_range(
            &CsafVersionNumber::from("2.0.0"),
        )]);
        let case_semver_2_3_missing_1 = Err(vec![test_6_1_21_err_wrong_first_version_generator(
            &CsafVersionNumber::from("2.0.0"),
            &0,
        )]);
        let case_semver_missing_2 = Err(vec![test_6_1_21_err_missing_version_in_range(
            &CsafVersionNumber::from("2.0.0"),
        )]);
        let case_semver_multiple_single_versions_missing = Err(vec![
            test_6_1_21_err_missing_version_in_range(&CsafVersionNumber::from("2.0.0")),
            test_6_1_21_err_missing_version_in_range(&CsafVersionNumber::from("4.0.0")),
            test_6_1_21_err_missing_version_in_range(&CsafVersionNumber::from("6.0.0")),
        ]);

        TESTS_2_0.test_6_1_21.expect(
            case_intver_1_3_missing_2.clone(),
            case_intver_2_3_missing_1.clone(),
            case_semver_1_3_missing_2.clone(),
            case_semver_2_3_missing_1.clone(),
            case_semver_missing_2.clone(),
            case_semver_multiple_single_versions_missing.clone(),
            Ok(()),
            Ok(()), // valid intver final start with 1
            Ok(()), // valid intver draft star with 0
            Ok(()), // valid semver final start with 1.0.0
        );

        let case_intver_1_3_4_with_timezone_missing_2 = Err(vec![test_6_1_21_err_missing_version_in_range(
            &CsafVersionNumber::from("2"),
        )]);

        TESTS_2_1.test_6_1_21.expect(
            case_intver_1_3_missing_2,
            case_intver_2_3_missing_1,
            case_intver_1_3_4_with_timezone_missing_2,
            case_semver_1_3_missing_2,
            case_semver_2_3_missing_1,
            Ok(()), // valid intver final start with 1
            Ok(()), // valid intver draft star with 0
            Ok(()), // valid semver final start with 1.0.0
            Ok(()), // valid intver with timezones
        );
    }
}
