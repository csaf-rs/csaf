use crate::validation::{TestFinding, TestFindingData};
use jsonschema::error::ValidationErrorKind;
use serde_json::Value;

#[jsonschema::validator(
    path = "assets/csaf_2.0_json_schema.strict.json",
    validate_formats = true,
    resources = {
        "https://www.first.org/cvss/cvss-v2.0.json" => { path = "assets/cvss-v2.0.strict.json" },
        "https://www.first.org/cvss/cvss-v3.0.json" => { path = "assets/cvss-v3.0.json"},
        "https://www.first.org/cvss/cvss-v3.1.json" => { path = "assets/cvss-v3.1.json"}
    }
)]
struct StrictValidator2_0;

#[jsonschema::validator(
    path = "assets/csaf_2.1_json_schema.strict.json",
    validate_formats = true,
    draft = Draft202012,
    resources = {
        "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/extension-metaschema.json" => { path = "assets/extension-metaschema.strict.json" },
        "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/extension-content.json" => { path = "assets/extension-content.strict.json" },
        "https://www.first.org/cvss/cvss-v2.0.json" => { path = "assets/cvss-v2.0.strict.json" },
        "https://www.first.org/cvss/cvss-v3.0.json" => { path = "assets/cvss-v3.0.json"}, // we may not make this strict, otherwise the oneOf does not match
        "https://www.first.org/cvss/cvss-v3.1.json" => { path = "assets/cvss-v3.1.json"}, // we may not make this strict, otherwise the oneOf does not match
        "https://www.first.org/cvss/cvss-v4.0.json" => { path = "assets/cvss-v4.0.strict.json" },
        "https://certcc.github.io/SSVC/data/schema/v2/SelectionList_2_0_0.schema.json" => { path = "assets/SelectionList_2_0_0.schema.strict.json" }
    }
)]
struct StrictValidator2_1;

/// 6.2.20 Additional Properties
///
/// There is no additional property in the CSAF document that was not defined in the CSAF JSON schema.
pub fn test_6_2_20_additional_properties(
    json: &Value,
    iter_errors: impl Fn(&serde_json::Value) -> jsonschema::ErrorIterator,
) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;
    for error in iter_errors(json) {
        if let ValidationErrorKind::UnevaluatedProperties { unexpected } = error.kind() {
            for property in unexpected {
                errors.get_or_insert_default().push(create_additional_properties_error(
                    property,
                    error.instance_path().as_str(),
                ));
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

fn create_additional_properties_error(key: &str, path: &str) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!("The key '{key}' is not defined in the JSON schema."),
        instance_path: path.to_string(),
    })
}

fn test_6_2_20_validate_2_0(json: &Value) -> Result<(), Vec<TestFinding>> {
    test_6_2_20_additional_properties(json, StrictValidator2_0::iter_errors)
}

fn test_6_2_20_validate_2_1(json: &Value) -> Result<(), Vec<TestFinding>> {
    test_6_2_20_additional_properties(json, StrictValidator2_1::iter_errors)
}

crate::test_validation::impl_raw_json_validator!(csaf2_0, ValidatorForTest6_2_20, test_6_2_20_validate_2_0);
crate::test_validation::impl_raw_json_validator!(csaf2_1, ValidatorForTest6_2_20, test_6_2_20_validate_2_1);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_2_20 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_2_20 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_20() {
        // Both CSAF 2.0 and 2.1 have 1 test cases
        TESTS_2_0.test_6_2_20.expect(ExpectedResults_2_0 {
            case_01: Err(vec![create_additional_properties_error("custom_property", "/document")]),
        });
        TESTS_2_1.test_6_2_20.expect(ExpectedResults_2_1 {
            case_01: Err(vec![create_additional_properties_error(
                "custom_property",
                "/vulnerabilities/0/metrics/0/content/cvss_v3",
            )]),
            case_02: Err(vec![create_additional_properties_error(
                "custom_property",
                "/vulnerabilities/0/metrics/0/content/cvss_v4",
            )]),
            case_11: Ok(()),
            case_12: Ok(()),
        });
    }
}
