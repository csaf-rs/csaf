use crate::csaf_traits::{CsafTrait, VersionNumber};
use crate::validation::ValidationError;
use crate::version_helpers::{generate_revision_history_tuples, sort_revision_history_tuples_by_date_by_number};

/// 6.1.21 Missing Item in Revision History
///
/// When ordered by their `date` field, all `/document/tracking/revision_history[]` items need to contain
/// all integers in the range between the `number` of first revision history and the last revision history.
/// Also, it has to be ensured that the first item has either a version 0 or 1.
/// This applies to the version number for integer versioning and to the major version for semantic versioning.
pub fn test_6_1_21_missing_item_in_revision_history(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    // Generate and sort the revision history tuples by date first and by number second
    let mut rev_history_tuples = generate_revision_history_tuples(doc);
    sort_revision_history_tuples_by_date_by_number(&mut rev_history_tuples);

    if rev_history_tuples.is_empty() {
        // This should not be able to happen as revision history is a required property with 1..* items
        panic!("Revision history is empty, document is malformed.");
    }

    // We can safely unwrap here, as there has to be at least one item in rev_history_tuples
    let first_tuple = rev_history_tuples.first().unwrap();
    let first_version = first_tuple.2.clone();
    let first_number = first_version.get_major();

    // Throw error if first version is not 0 or 1
    if first_number > 1 {
        return Err(vec![test_6_1_21_err_wrong_first_version_generator(
            first_version,
            first_tuple.0.to_string(),
        )]);
    }

    let last_number = rev_history_tuples.last().unwrap().2.clone().get_major();

    println!("First number: {}", first_number);
    println!("Last number: {}", last_number);
    for expected_number in first_number + 1..last_number {
        println!("Checking for expected number: {}", expected_number);
        let mut found = false;
        for (_, _, number) in rev_history_tuples.iter() {
            if number.clone().get_major() == expected_number {
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
    use crate::csaf_traits::VersionNumber;
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validations::test_6_1_21::{
        test_6_1_21_err_missing_version_in_range, test_6_1_21_err_wrong_first_version_generator,
        test_6_1_21_missing_item_in_revision_history,
    };

    #[test]
    fn test_test_6_1_21() {
        let errors = std::collections::HashMap::from([
            (
                "01",
                vec![test_6_1_21_err_missing_version_in_range(
                    VersionNumber::from_number("1"),
                    2,
                    1,
                    3,
                )],
            ),
            (
                "02",
                vec![test_6_1_21_err_wrong_first_version_generator(
                    VersionNumber::from_number("2"),
                    "0".to_string(),
                )],
            ),
            (
                "03",
                vec![test_6_1_21_err_missing_version_in_range(
                    VersionNumber::from_number("1"),
                    2,
                    1,
                    4,
                )],
            ),
        ]);
        run_csaf20_tests("21", test_6_1_21_missing_item_in_revision_history, errors.clone());
        run_csaf21_tests("21", test_6_1_21_missing_item_in_revision_history, errors);
    }
}
