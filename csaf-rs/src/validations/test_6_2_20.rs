use std::sync::LazyLock;

use crate::{csaf::raw::RawDocument, validation::ValidationError};
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

const CSAF_2_0_SCHEMA_URL: &str = "https://docs.oasis-open.org/csaf/csaf/v2.0/csaf_json_schema.json";
static CSAF_2_0_SCHEMA: LazyLock<Resource> = LazyLock::new(|| {
    let schema_str = include_str!("../../assets/csaf_2.0_json_schema.json");
    let schema_json: Value = serde_json::from_str(schema_str).unwrap();
    Resource::from_contents(make_strict(schema_json))
});

const CSAF_2_1_SCHEMA_URL: &str = "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/csaf.json";
static CSAF_2_1_SCHEMA: LazyLock<Resource> = LazyLock::new(|| {
    let schema_str = include_str!("../../assets/csaf_2.1_json_schema.json");
    let schema_json: Value = serde_json::from_str(schema_str).unwrap();
    Resource::from_contents(make_strict(schema_json))
});

const CVSS_V2_SCHEMA_URL: &str = "https://www.first.org/cvss/cvss-v2.0.json";
static CVSS_V2_SCHEMA: LazyLock<Resource> = LazyLock::new(|| {
    let schema_str = include_str!("../../assets/cvss-v2.0.json");
    let schema_json: Value = serde_json::from_str(schema_str).unwrap();
    Resource::from_contents(make_strict(schema_json))
});

const CVSS_V3_0_SCHEMA_URL: &str = "https://www.first.org/cvss/cvss-v3.0.json";
static CVSS_V3_0_SCHEMA: LazyLock<Resource> = LazyLock::new(|| {
    let schema_str = include_str!("../../assets/cvss-v3.0.json");
    let schema_json: Value = serde_json::from_str(schema_str).unwrap();
    // we may not make this strict, otherwise the oneOf does not match
    Resource::from_contents(schema_json)
});

const CVSS_V3_1_SCHEMA_URL: &str = "https://www.first.org/cvss/cvss-v3.1.json";
static CVSS_V3_1_SCHEMA: LazyLock<Resource> = LazyLock::new(|| {
    let schema_str = include_str!("../../assets/cvss-v3.1.json");
    let schema_json: Value = serde_json::from_str(schema_str).unwrap();
    // we may not make this strict, otherwise the oneOf does not match
    Resource::from_contents(schema_json)
});

const CVSS_V4_0_1_SCHEMA_URL: &str = "https://www.first.org/cvss/cvss-v4.0.1.json";
static CVSS_V4_0_1_SCHEMA: LazyLock<Resource> = LazyLock::new(|| {
    let schema_str = include_str!("../../assets/cvss-v4.0.rev.json");
    let schema_json: Value = serde_json::from_str(schema_str).unwrap();
    Resource::from_contents(make_strict(schema_json))
});

const SSVC_2_SCHEMA_URL: &str = "https://certcc.github.io/SSVC/data/schema/v2/SelectionList_2_0_0.schema.json";
static SSVC_2_SCHEMA: LazyLock<Resource> = LazyLock::new(|| {
    let schema_str = include_str!("../../assets/decision_point_selection_list_json_schema.json");
    let schema_json: Value = serde_json::from_str(schema_str).unwrap();
    Resource::from_contents(schema_json)
});

/// 6.2.20 Additional Properties
///
/// There is no additional property in the CSAF document that was not defined in the CSAF JSON schema.
pub fn test_6_2_20_additional_properties(json: &Value, schema: Resource) -> Result<(), Vec<ValidationError>> {
    let validator = jsonschema::options()
        .with_resource(CSAF_2_0_SCHEMA_URL, CSAF_2_0_SCHEMA.clone())
        .with_resource(CSAF_2_1_SCHEMA_URL, CSAF_2_1_SCHEMA.clone())
        .with_resource(CVSS_V2_SCHEMA_URL, CVSS_V2_SCHEMA.clone())
        .with_resource(CVSS_V3_0_SCHEMA_URL, CVSS_V3_0_SCHEMA.clone())
        .with_resource(CVSS_V3_1_SCHEMA_URL, CVSS_V3_1_SCHEMA.clone())
        .with_resource(CVSS_V4_0_1_SCHEMA_URL, CVSS_V4_0_1_SCHEMA.clone())
        .with_resource(SSVC_2_SCHEMA_URL, SSVC_2_SCHEMA.clone())
        .build(schema.contents())
        .unwrap();

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
        test_6_2_20_additional_properties(document.get_json(), CSAF_2_0_SCHEMA.clone())
    }
}

impl crate::test_validation::TestValidator<RawDocument<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>>
    for crate::csaf2_1::testcases::ValidatorForTest6_2_20
{
    fn validate(
        &self,
        document: &RawDocument<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_2_20_additional_properties(document.get_json(), CSAF_2_1_SCHEMA.clone())
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
