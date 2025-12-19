use crate::csaf_traits::{CsafTrait, DocumentTrait, RevisionHistorySortable, TrackingTrait, VersionNumber};
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

    if rev_history_tuples.is_empty() {
        // This should not be able to happen as revision history is a required property with 1..* items
        panic!("Revision history is empty, document is malformed.");
    }

    // We can safely unwrap here, as there has to be at least one item in rev_history_tuples
    let first_tuple = rev_history_tuples.first().unwrap();
    let first_version = first_tuple.number.clone();
    let first_number = first_version.get_major();

    // Throw error if first version is not 0 or 1
    if first_number > 1 {
        return Err(vec![test_6_1_21_err_wrong_first_version_generator(
            first_version,
            first_tuple.path_index.to_string(),
        )]);
    }

    let last_number = rev_history_tuples.last().unwrap().number.clone().get_major();

    for expected_number in first_number + 1..last_number {
        let mut found = false;
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
                    first_version.clone(),
                    expected_number,
                    first_number,
                    last_number,
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

fn test_6_1_21_err_wrong_first_version_generator(version: VersionNumber, path: String) -> ValidationError {
    let version_error = match version {
        VersionNumber::Integer(_) => "integer version of 0 or 1",
        VersionNumber::Semver(_) => "semver version of 0.y.z or 1.y.z",
    }
    .to_string();
    ValidationError {
        message: format!(
            "The first revision history item should have {}, but was {}",
            version_error, version
        ),
        instance_path: format!("/document/tracking/revision_history/{}", path),
    }
}

fn test_6_1_21_err_missing_version_in_range(
    version: VersionNumber,
    expected_number: u64,
    first_number: u64,
    last_number: u64,
) -> ValidationError {
    let version_error = match version {
        VersionNumber::Integer(_) => format!("integer version {}", expected_number),
        VersionNumber::Semver(_) => format!("semver version {}.y.z", expected_number),
    }
    .to_string();
    let version_error_range = match version {
        VersionNumber::Integer(_) => format!("integer version range {} to {}", first_number, last_number),
        VersionNumber::Semver(_) => format!("semver version range {}.y.z to {}.y.z", first_number, last_number),
    }
    .to_string();
    ValidationError {
        message: format!(
            "Missing revision history item with {} number {}",
            version_error, version_error_range
        ),
        instance_path: "/document/tracking/revision_history".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf_traits::VersionNumber;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_21() {
        // Error cases
        let case_01 = Err(vec![test_6_1_21_err_missing_version_in_range(
            VersionNumber::from_number("1"),
            2,
            1,
            3,
        )]);
        let case_02 = Err(vec![test_6_1_21_err_wrong_first_version_generator(
            VersionNumber::from_number("2"),
            "0".to_string(),
        )]);
        let case_03 = Err(vec![test_6_1_21_err_missing_version_in_range(
            VersionNumber::from_number("1"),
            2,
            1,
            4,
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
            case_03,
            Ok(()), // case_11
            Ok(()), // case_12
            Ok(()), // case_13
            Ok(()), // case_14
        );
    }
}
