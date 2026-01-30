use std::fmt::Display;
use crate::csaf::aggregation::csaf_revision_history::validated_revision_history::{TypedValidCsafRevisionHistory, ValidRevisionHistory, ValidatedRevisionHistory, VersionNumberKind};
use crate::csaf_traits::{CsafTrait, DocumentTrait, TrackingTrait};
use crate::validation::ValidationError;

/// 6.1.21 Missing Item in Revision History
///
/// When ordered by their `date` field, all `/document/tracking/revision_history[]` items need to contain
/// all integers in the range between the `number` of first revision history and the last revision history.
/// Also, it has to be ensured that the first item has either a version 0 or 1.
/// This applies to the version number for integer versioning and to the major version for semantic versioning.
pub fn test_6_1_21_missing_item_in_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    // Generate and sort the revision history tuples by date first and by number second
    // Get the revision history
    let revision_history = doc.get_document().get_tracking().get_revision_history();
    let validated = ValidatedRevisionHistory::from(&revision_history);
    // Check if revision history is valid, if not return the errors and skip this test
    let valid = match validated {
        ValidatedRevisionHistory::Valid(valid) => {valid}
        ValidatedRevisionHistory::Invalid(errors) => {return Err(errors.into())}
    };

    match valid {
        ValidRevisionHistory::IntVer(intver) => {
            check_for_missing_versions_numbers_in_revision_history(intver, VersionNumberTypeSpecificEnum::IntVer)
        }
        ValidRevisionHistory::SemVer(semver) => {
            check_for_missing_versions_numbers_in_revision_history(semver, VersionNumberTypeSpecificEnum::SemVer)
        }
    }
}

fn check_for_missing_versions_numbers_in_revision_history<V: VersionNumberKind>(history: TypedValidCsafRevisionHistory<V>, error_message: VersionNumberTypeSpecificEnum) -> Result<(), Vec<ValidationError>> {
    // sort the revision history by first by date, then by number
    let sorted = history.get_sorted_by_date_by_number();

    // get first version in sorted array
    let first = sorted.first();
    let first_major = first.number.get_major();

    // Throw error if first version is not 0 or 1
    if first_major > 1 {
        return Err(vec![test_6_1_21_err_wrong_first_version_generator(
            &error_message,
            first.number,
            &first.path_index,
        )]);
    }

    // get last version
    let last_major = sorted.last().number.get_major();
    // extract all major version numbers from the sorted revision history
    let major_numbers: Vec<u64> = sorted.iter().map(|item| item.number.get_major()).collect();

    let mut errors: Option<Vec<ValidationError>> = None;
    // check if all integers in the range between first and last version are present in the extracted major version numbers
    // if not generate an error
    for expected_number in first_major + 1..last_major {
        if !major_numbers.contains(&expected_number) {
            errors
                .get_or_insert_with(Vec::new)
                .push(test_6_1_21_err_missing_version_in_range(
                    &error_message,
                    &expected_number,
                    &first_major,
                    &last_major,
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

// This enum is used to generate specific error messages for integer versioning and semantic versioning
enum VersionNumberTypeSpecificEnum {
    IntVer,
    SemVer,
}

fn test_6_1_21_err_wrong_first_version_generator(
    version_number_type: &VersionNumberTypeSpecificEnum,
    first_item: impl Display,
    revision_index: &usize,
) -> ValidationError {
    let version_error = match version_number_type {
        VersionNumberTypeSpecificEnum::IntVer => "integer version of 0 or 1",
        VersionNumberTypeSpecificEnum::SemVer => "semver version of 0.y.z or 1.y.z",
    }
    .to_string();
    ValidationError {
        message: format!("The first revision history item should have {version_error}, but was {first_item}"),
        instance_path: format!("/document/tracking/revision_history/{revision_index}"),
    }
}

fn test_6_1_21_err_missing_version_in_range(
    version_number_type: &VersionNumberTypeSpecificEnum,
    expected_number: &u64,
    first_number: &u64,
    last_number: &u64,
) -> ValidationError {
    let version_error = match version_number_type {
        VersionNumberTypeSpecificEnum::IntVer => format!("integer version {expected_number}"),
        VersionNumberTypeSpecificEnum::SemVer => format!("semver version {expected_number}.y.z"),
    };
    let version_error_range = match version_number_type {
        VersionNumberTypeSpecificEnum::IntVer => format!("integer version range {first_number} to {last_number}"),
        VersionNumberTypeSpecificEnum::SemVer => format!("semver version range {first_number}.y.z to {last_number}.y.z"),
    };
    ValidationError {
        message: format!("Missing revision history item with {version_error} number {version_error_range}"),
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
        // TODO: Unit Tests with semver
        // Error cases
        let case_01 = Err(vec![test_6_1_21_err_missing_version_in_range(
            &VersionNumberTypeSpecificEnum::IntVer,
            &2,
            &1,
            &3,
        )]);
        let case_02 = Err(vec![test_6_1_21_err_wrong_first_version_generator(
            &VersionNumberTypeSpecificEnum::IntVer,
            &2,
            &0,
        )]);

        // CSAF 2.0 has 5 test cases (01-02, 11-13)
        TESTS_2_0.test_6_1_21.expect(
            case_01.clone(),
            case_02.clone(),
            Ok(()), // case_11
            Ok(()), // case_12
            Ok(()), // case_13
        );

        // CSAF 2.1 has 7 test cases (01-03, 11-14)
        TESTS_2_1.test_6_1_21.expect(
            case_01,
            case_02,
            Err(vec![test_6_1_21_err_missing_version_in_range(
                &VersionNumberTypeSpecificEnum::IntVer,
                &2,
                &1,
                &4,
            )]),
            Ok(()), // case_11
            Ok(()), // case_12
            Ok(()), // case_13
            Ok(()), // case_14
        );
    }
}
