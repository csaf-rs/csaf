use serde_json::Value;

use crate::{
    csaf::raw::RawDocument,
    validation::{TestFinding, TestFindingData},
};

#[jsonschema::validator(
    path = "assets/csaf_2.0_json_schema.json",
    validate_formats = true,
    resources = {
        "https://www.first.org/cvss/cvss-v2.0.json" => { path = "assets/cvss-v2.0.json" },
        "https://www.first.org/cvss/cvss-v3.0.json" => { path = "assets/cvss-v3.0.json"},
        "https://www.first.org/cvss/cvss-v3.1.json" => { path = "assets/cvss-v3.1.json"}
    }
)]
struct Validator2_0;

#[jsonschema::validator(
    path = "assets/csaf_2.1_json_schema.json",
    validate_formats = true,
    draft = Draft202012,
    resources = {
        "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/extension-metaschema.json" => { path = "assets/extension-metaschema.json" },
        "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/extension-content.json" => { path = "assets/extension-content.json" },
        "https://www.first.org/cvss/cvss-v2.0.json" => { path = "assets/cvss-v2.0.json" },
        "https://www.first.org/cvss/cvss-v3.0.json" => { path = "assets/cvss-v3.0.json"},
        "https://www.first.org/cvss/cvss-v3.1.json" => { path = "assets/cvss-v3.1.json"},
        "https://www.first.org/cvss/cvss-v4.0.json" => { path = "assets/cvss-v4.0.json" },
        "https://certcc.github.io/SSVC/data/schema/v2/SelectionList_2_0_0.schema.json" => { path = "assets/SelectionList_2_0_0.schema.json" }
    }
)]
struct Validator2_1;

fn create_schema_error(err: String, path: &str) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: err,
        instance_path: match path.len() {
            0 => "/".to_string(),
            _ => path.to_string(),
        },
    })
}

fn validate_schema(
    document: &Value,
    iter_errors: impl Fn(&serde_json::Value) -> jsonschema::ErrorIterator,
) -> Result<(), Vec<TestFinding>> {
    let errors: Vec<_> = iter_errors(document)
        .map(|error| create_schema_error(format!("{error}"), error.instance_path().as_str()))
        .collect();
    match errors.len() {
        0 => Ok(()),
        _ => Err(errors),
    }
}

pub fn validate_schema_csaf_2_0(
    document: &RawDocument<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>,
) -> Result<(), Vec<TestFinding>> {
    validate_schema(document.get_json(), Validator2_0::iter_errors)
}

pub fn validate_schema_csaf_2_1(
    document: &RawDocument<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>,
) -> Result<(), Vec<TestFinding>> {
    validate_schema(document.get_json(), Validator2_1::iter_errors)
    // TODO: validate extensions
}

#[cfg(test)]
mod tests {
    use super::*;

    // we cannot use the generated files here now because the testcases schema does not allow the test id "schema".
    macro_rules! check_file {
        ($csaf_major: expr, $csaf_minor: expr, $case:expr, $validator:expr, $expected:expr) => {
            let file_content = include_str!(concat!(
                "../../../type-generator/assets/tests/",
                concat!("csaf_", $csaf_major, ".", $csaf_minor),
                "/schema/",
                concat!("csaf-rs_csaf-csaf_", $csaf_major, "_", $csaf_minor),
                concat!("-schema-", $case, ".json")
            ));
            let actual = validate_schema(&serde_json::from_str(file_content).unwrap(), $validator);
            crate::test_result_comparison::compare_test_results(
                &actual,
                &$expected,
                concat!("V", $csaf_major, "_", $csaf_minor),
                "schema",
                $case,
            )
            .unwrap_or_else(|e| panic!("{}", e));
        };
    }

    #[test]
    fn test_validate_schema() {
        let min_properties = Err(vec![create_schema_error(
            r#"{} has less than 1 property"#.to_string(),
            "/vulnerabilities/0",
        )]);
        let pattern = Err(vec![create_schema_error(
            r#""does-not-match-regex" does not match "^CVE-[0-9]{4}-[0-9]{4,}$""#.to_string(),
            "/vulnerabilities/0/cve",
        )]);
        let min_items = Err(vec![create_schema_error(
            r#"[] has less than 1 item"#.to_string(),
            "/vulnerabilities",
        )]);
        let min_length = Err(vec![create_schema_error(
            r#""" is shorter than 1 character"#.to_string(),
            "/vulnerabilities/0/ids/0/text",
        )]);
        let non_unique = Err(vec![create_schema_error(
            r#"[{"system_name":"GitHub Issue","text":"oasis-tcs/csaf#210"},{"system_name":"GitHub Issue","text":"oasis-tcs/csaf#210"}] has non-unique elements"#.to_string(),
            "/vulnerabilities/0/ids",
        )]);
        let format = Err(vec![create_schema_error(
            r#""2025-01-01T01:01:01" is not a "date-time""#.to_string(),
            "/vulnerabilities/0/disclosure_date",
        )]);

        // checks for CSAF 2.0
        check_file!(2, 0, "s01", &Validator2_0::iter_errors, min_properties);
        check_file!(2, 0, "s02", &Validator2_0::iter_errors, pattern);
        check_file!(2, 0, "s03", &Validator2_0::iter_errors, min_items);
        check_file!(2, 0, "s04", &Validator2_0::iter_errors, min_length);
        check_file!(2, 0, "s05", &Validator2_0::iter_errors, non_unique);

        // checks for CSAF 2.1
        check_file!(2, 1, "s01", &Validator2_1::iter_errors, min_properties);
        check_file!(2, 1, "s02", &Validator2_1::iter_errors, pattern);
        check_file!(2, 1, "s03", &Validator2_1::iter_errors, min_items);
        check_file!(2, 1, "s04", &Validator2_1::iter_errors, min_length);
        check_file!(2, 1, "s05", &Validator2_1::iter_errors, non_unique);
        check_file!(2, 1, "s06", &Validator2_1::iter_errors, format);
    }
}
