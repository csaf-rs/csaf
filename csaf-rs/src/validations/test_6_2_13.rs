use crate::{csaf::raw::RawDocument, validation::ValidationError};
use serde_json::Value;

/// 6.2.13 Sorting
///
/// All keys in a CSAF document must be sorted alphabetically.
pub fn test_6_2_13_sorting(json: &Value) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    match check_sorted_recursive(json, "", &mut errors) {
        true => Ok(()),
        false => Err(errors),
    }
}

fn check_sorted_recursive(value: &Value, path: &str, errors: &mut Vec<ValidationError>) -> bool {
    match value {
        Value::Object(map) => {
            // object -> check if keys are sorted
            let keys_ok = map.keys().zip(map.keys().skip(1)).all(|(a, b)| {
                if a > b {
                    errors.push(create_unsorted_keys_error(format!("{path}/{a}").as_str()));
                    false
                } else {
                    true
                }
            });

            // check all children recursively
            let children_ok = map
                .iter()
                .filter(|(key, value)| !check_sorted_recursive(value, format!("{path}/{key}").as_str(), errors))
                .count()
                == 0;

            keys_ok && children_ok
        },
        Value::Array(arr) => {
            // array -> check for each item
            arr.iter()
                .enumerate()
                .filter(|(key, value)| !check_sorted_recursive(value, format!("{path}/{key}").as_str(), errors))
                .count()
                == 0
        },
        // primitive types are always sorted
        _ => true,
    }
}

fn create_unsorted_keys_error(path: &str) -> ValidationError {
    ValidationError {
        message: "The keys in the CSAF document are not sorted alphabetically".to_string(),
        instance_path: path.to_string(),
    }
}

impl crate::test_validation::TestValidator<RawDocument<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>>
    for crate::csaf2_0::testcases::ValidatorForTest6_2_13
{
    fn validate(
        &self,
        document: &RawDocument<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_13_sorting(document.get_json())
    }
}

impl crate::test_validation::TestValidator<RawDocument<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_13
{
    fn validate(
        &self,
        document: &RawDocument<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_13_sorting(document.get_json())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_13() {
        let err = Err(vec![create_unsorted_keys_error("/document/csaf_version")]);

        // Both CSAF 2.0 and 2.1 have 1 test cases
        TESTS_2_0.test_6_2_13.expect(err.clone());
        TESTS_2_1.test_6_2_13.expect(err);
    }
}
