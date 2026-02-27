use std::sync::LazyLock;

use crate::{
    csaf::raw::RawDocument,
    helpers::{
        CSAF_2_0_SCHEMA, CSAF_2_1_SCHEMA, CVSS_V2_SCHEMA, CVSS_V2_SCHEMA_URL, CVSS_V3_0_SCHEMA, CVSS_V3_0_SCHEMA_URL,
        CVSS_V3_1_SCHEMA, CVSS_V3_1_SCHEMA_URL, CVSS_V4_0_1_SCHEMA, CVSS_V4_0_1_SCHEMA_URL, SSVC_2_SCHEMA,
        SSVC_2_SCHEMA_URL,
    },
    validation::ValidationError,
};
use jsonschema::{Resource, error::ValidationErrorKind};
use serde_json::Value;

fn make_strict(schema_value: Value) -> Value {
    let mut schema_value = schema_value;
    make_strict_inplace(&mut schema_value);
    schema_value.as_object_mut().unwrap().insert(
        "$schema".to_string(),
        Value::String("https://json-schema.org/draft/2020-12/schema".to_string()),
    );
    schema_value
}

fn make_strict_inplace(schema_value: &mut Value) {
    if let Some(obj) = schema_value.as_object_mut() {
        for value in obj.values_mut() {
            make_strict_inplace(value);
        }
        if obj.get("type").and_then(|t| t.as_str()) == Some("object") {
            obj.insert("unevaluatedProperties".to_string(), Value::Bool(false));
        }
        if obj.contains_key("oneOf") {
            obj.insert("unevaluatedProperties".to_string(), Value::Bool(false));
        }
    } else if let Some(array) = schema_value.as_array_mut() {
        for item in array {
            make_strict_inplace(item);
        }
    }
}

static STRICT_VALIDATOR_2_0: LazyLock<jsonschema::Validator> = LazyLock::new(|| {
    jsonschema::options()
        .with_resource(CVSS_V2_SCHEMA_URL, Resource::from_contents(make_strict(CVSS_V2_SCHEMA.clone())))
        .with_resource(CVSS_V3_0_SCHEMA_URL, Resource::from_contents(CVSS_V3_0_SCHEMA.clone()))     // we may not make this strict, otherwise the oneOf does not match
        .with_resource(CVSS_V3_1_SCHEMA_URL, Resource::from_contents(CVSS_V3_1_SCHEMA.clone()))     // we may not make this strict, otherwise the oneOf does not match
        .build(&make_strict(CSAF_2_0_SCHEMA.clone()))
        .unwrap()
});

static STRICT_VALIDATOR_2_1: LazyLock<jsonschema::Validator> = LazyLock::new(|| {
    jsonschema::options()
        .with_resource(CVSS_V2_SCHEMA_URL, Resource::from_contents(make_strict(CVSS_V2_SCHEMA.clone())))
        .with_resource(CVSS_V3_0_SCHEMA_URL, Resource::from_contents(CVSS_V3_0_SCHEMA.clone()))     // we may not make this strict, otherwise the oneOf does not match
        .with_resource(CVSS_V3_1_SCHEMA_URL, Resource::from_contents(CVSS_V3_1_SCHEMA.clone()))     // we may not make this strict, otherwise the oneOf does not match
        .with_resource(CVSS_V4_0_1_SCHEMA_URL, Resource::from_contents(make_strict(CVSS_V4_0_1_SCHEMA.clone())))
        .with_resource(SSVC_2_SCHEMA_URL, Resource::from_contents(make_strict(SSVC_2_SCHEMA.clone())))
        .build(&make_strict(CSAF_2_1_SCHEMA.clone()))
        .unwrap()
});

/// 6.2.20 Additional Properties
///
/// There is no additional property in the CSAF document that was not defined in the CSAF JSON schema.
pub fn test_6_2_20_additional_properties(
    json: &Value,
    validator: &jsonschema::Validator,
) -> Result<(), Vec<ValidationError>> {
    let results: Vec<_> = validator
        .iter_errors(json)
        .flat_map(|error| match error.kind() {
            ValidationErrorKind::UnevaluatedProperties { unexpected } => unexpected
                .iter()
                .map(|property| create_additional_properties_error(property, error.instance_path().as_str()))
                .collect(),
            _ => vec![],
        })
        .collect();

    if results.is_empty() { Ok(()) } else { Err(results) }
}

fn create_additional_properties_error(key: &str, path: &str) -> ValidationError {
    ValidationError {
        message: format!("The key '{key}' is not defined in the JSON schema."),
        instance_path: path.to_string(),
    }
}

impl crate::test_validation::TestValidator<RawDocument<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>>
    for crate::csaf2_0::testcases::ValidatorForTest6_2_20
{
    fn validate(
        &self,
        document: &RawDocument<crate::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_20_additional_properties(document.get_json(), &STRICT_VALIDATOR_2_0)
    }
}

impl crate::test_validation::TestValidator<RawDocument<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_20
{
    fn validate(
        &self,
        document: &RawDocument<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_20_additional_properties(document.get_json(), &STRICT_VALIDATOR_2_1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_2_20() {
        // Both CSAF 2.0 and 2.1 have 1 test cases
        TESTS_2_0
            .test_6_2_20
            .expect(Err(vec![create_additional_properties_error(
                "custom_property",
                "/document",
            )]));
        TESTS_2_1.test_6_2_20.expect(
            Err(vec![create_additional_properties_error(
                "custom_property",
                "/vulnerabilities/0/metrics/0/content/cvss_v3",
            )]),
            Err(vec![create_additional_properties_error(
                "custom_property",
                "/vulnerabilities/0/metrics/0/content/cvss_v4",
            )]),
            Ok(()),
            Ok(()),
        );
    }
}
