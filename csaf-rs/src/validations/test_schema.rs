use std::sync::LazyLock;

use jsonschema::Resource;
use serde_json::Value;

use crate::{
    csaf::raw::RawDocument,
    helpers::{
        CSAF_2_0_SCHEMA, CSAF_2_1_SCHEMA, CVSS_V2_SCHEMA, CVSS_V2_SCHEMA_URL, CVSS_V3_0_SCHEMA, CVSS_V3_0_SCHEMA_URL,
        CVSS_V3_1_SCHEMA, CVSS_V3_1_SCHEMA_URL, CVSS_V4_0_2_SCHEMA, CVSS_V4_0_2_SCHEMA_URL, EXTENSION_METASCHEMA,
        EXTENSION_METASCHEMA_URL, EXTENSION_SCHEMA, EXTENSION_SCHEMA_URL, SSVC_2_SCHEMA, SSVC_2_SCHEMA_URL,
    },
    validation::ValidationError,
};

fn use_draft_schema(mut schema_value: Value) -> Value {
    schema_value.as_object_mut().unwrap().insert(
        "$schema".to_string(),
        Value::String("https://json-schema.org/draft/2020-12/schema".to_string()),
    );
    schema_value
}

static VALIDATOR_2_0: LazyLock<jsonschema::Validator> = LazyLock::new(|| {
    jsonschema::options()
        .should_validate_formats(true)
        .with_resource(CVSS_V2_SCHEMA_URL, Resource::from_contents(CVSS_V2_SCHEMA.clone()))
        .with_resource(CVSS_V3_0_SCHEMA_URL, Resource::from_contents(CVSS_V3_0_SCHEMA.clone()))
        .with_resource(CVSS_V3_1_SCHEMA_URL, Resource::from_contents(CVSS_V3_1_SCHEMA.clone()))
        .build(&CSAF_2_0_SCHEMA.clone())
        .unwrap()
});

static VALIDATOR_2_1: LazyLock<jsonschema::Validator> = LazyLock::new(|| {
    jsonschema::options()
        .should_validate_formats(true)
        .with_resource(
            EXTENSION_METASCHEMA_URL,
            Resource::from_contents(use_draft_schema(EXTENSION_METASCHEMA.clone())),
        )
        .with_resource(
            EXTENSION_SCHEMA_URL,
            Resource::from_contents(use_draft_schema(EXTENSION_SCHEMA.clone())),
        )
        .with_resource(CVSS_V2_SCHEMA_URL, Resource::from_contents(CVSS_V2_SCHEMA.clone()))
        .with_resource(CVSS_V3_0_SCHEMA_URL, Resource::from_contents(CVSS_V3_0_SCHEMA.clone()))
        .with_resource(CVSS_V3_1_SCHEMA_URL, Resource::from_contents(CVSS_V3_1_SCHEMA.clone()))
        .with_resource(
            CVSS_V4_0_2_SCHEMA_URL,
            Resource::from_contents(use_draft_schema(CVSS_V4_0_2_SCHEMA.clone())),
        )
        .with_resource(SSVC_2_SCHEMA_URL, Resource::from_contents(SSVC_2_SCHEMA.clone()))
        .build(&use_draft_schema(CSAF_2_1_SCHEMA.clone()))
        .unwrap()
});

fn create_schema_error(err: String, path: &str) -> ValidationError {
    ValidationError {
        message: err.to_string(),
        instance_path: match path.len() {
            0 => "/".to_string(),
            _ => path.to_string(),
        },
    }
}

fn validate_schema(document: &Value, validator: &jsonschema::Validator) -> Result<(), Vec<ValidationError>> {
    let errors: Vec<_> = validator
        .iter_errors(document)
        .map(|error| create_schema_error(format!("{error}"), error.instance_path().as_str()))
        .collect();
    match errors.len() {
        0 => Ok(()),
        _ => Err(errors),
    }
}

pub fn validate_schema_csaf_2_0(
    document: &RawDocument<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>,
) -> Result<(), Vec<ValidationError>> {
    validate_schema(document.get_json(), &VALIDATOR_2_0)
}

pub fn validate_schema_csaf_2_1(
    document: &RawDocument<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>,
) -> Result<(), Vec<ValidationError>> {
    validate_schema(document.get_json(), &VALIDATOR_2_1)
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
        check_file!(2, 0, "s01", &VALIDATOR_2_0, min_properties.clone());
        check_file!(2, 0, "s02", &VALIDATOR_2_0, pattern.clone());
        check_file!(2, 0, "s03", &VALIDATOR_2_0, min_items.clone());
        check_file!(2, 0, "s04", &VALIDATOR_2_0, min_length.clone());
        check_file!(2, 0, "s05", &VALIDATOR_2_0, non_unique.clone());

        // checks for CSAF 2.1
        check_file!(2, 1, "s01", &VALIDATOR_2_1, min_properties);
        check_file!(2, 1, "s02", &VALIDATOR_2_1, pattern);
        check_file!(2, 1, "s03", &VALIDATOR_2_1, min_items);
        check_file!(2, 1, "s04", &VALIDATOR_2_1, min_length);
        check_file!(2, 1, "s05", &VALIDATOR_2_1, non_unique);
        check_file!(2, 1, "s06", &VALIDATOR_2_1, format);
    }
}
