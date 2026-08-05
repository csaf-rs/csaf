use crate::validation::ValidationError;
use serde_json::Value;

/// 6.2.13 Sorting
///
/// All keys in a CSAF document must be sorted alphabetically.
pub fn test_6_2_13_sorting(json: &Value) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;
    check_sorted_recursive(json, "", &mut errors);
    errors.map_or(Ok(()), Err)
}

fn check_sorted_recursive(value: &Value, path: &str, errors: &mut Option<Vec<ValidationError>>) {
    match value {
        Value::Object(map) => {
            // object -> check if keys are sorted
            for (a, b) in map.keys().zip(map.keys().skip(1)) {
                if a > b {
                    errors
                        .get_or_insert_default()
                        .push(create_unsorted_keys_error(format!("{path}/{a}").as_str()));
                }
            }

            // check all children recursively
            for (key, value) in map {
                check_sorted_recursive(value, format!("{path}/{key}").as_str(), errors);
            }
        },
        Value::Array(arr) => {
            // array -> check for each item
            for (key, value) in arr.iter().enumerate() {
                check_sorted_recursive(value, format!("{path}/{key}").as_str(), errors);
            }
        },
        // primitive types are always sorted
        _ => {},
    }
}

fn create_unsorted_keys_error(path: &str) -> ValidationError {
    ValidationError {
        message: "The keys in the CSAF document are not sorted alphabetically".to_string(),
        instance_path: path.to_string(),
    }
}

crate::test_validation::impl_raw_json_validator!(ValidatorForTest6_2_13, test_6_2_13_sorting);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_2_13 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_13 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_13() {
        let err = Err(vec![create_unsorted_keys_error("/document/csaf_version")]);

        // Both CSAF 2.0 and 2.1 have 1 test cases
        TESTS_2_0
            .test_6_2_13
            .expect(ExpectedResults_2_0 { case_01: err.clone() });
        TESTS_2_1.test_6_2_13.expect(ExpectedResults_2_1 { case_01: err });
    }
}
