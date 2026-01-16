use crate::validation::ValidationError;
use serde_json::Value;
use std::sync::LazyLock;

/// 6.2.13 Sorting
///
/// All keys in a CSAF document must be sorted alphabetically.
pub fn test_6_2_13_sorting(raw: &str) -> Result<(), Vec<ValidationError>> {
    let index_map: Value = serde_json::from_str(raw).unwrap();
    if !check_sorted_recursive(&index_map) {
        return Err(vec![KEYS_NOT_SORTED.clone()]);
    }
    Ok(())
}

fn check_sorted_recursive(value: &Value) -> bool {
    match value {
        Value::Object(map) => {
            // object -> check if keys are sorted
            let keys: Vec<&String> = map.keys().collect();
            let mut sorted_keys = keys.clone();
            sorted_keys.sort();

            if keys != sorted_keys {
                return false;
            }

            // recursion
            for v in map.values() {
                if !check_sorted_recursive(v) {
                    return false;
                }
            }
            true
        }
        Value::Array(arr) => {
            // array -> check for each item
            arr.iter().all(|item| check_sorted_recursive(item))
        }
        // primitive types are always sorted
        _ => true,
    }
}

static KEYS_NOT_SORTED: LazyLock<ValidationError> = LazyLock::new(|| ValidationError {
    message: "The keys in the CSAF document are not sorted alphabetically".to_string(),
    instance_path: "/".to_string(),
});

impl crate::test_validation::TestValidatorWithRawString
for crate::csaf2_0::testcases::ValidatorForTest6_2_13
{
    fn validate(
        &self,
        raw: &str,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_13_sorting(raw)
    }
}

impl crate::test_validation::TestValidatorWithRawString
for crate::csaf2_1::testcases::ValidatorForTest6_2_13
{
    fn validate(
        &self,
        raw: &str,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_13_sorting(raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_13() {
        let err = Err(vec![KEYS_NOT_SORTED.clone()]);

        // Both CSAF 2.0 and 2.1 have 1 test cases
        TESTS_2_0.test_6_2_13.expect(err.clone());
        TESTS_2_1.test_6_2_13.expect(err);
    }
}
