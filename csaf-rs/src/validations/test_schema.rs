use std::sync::LazyLock;

use jsonschema::Resource;
use serde_json::Value;

use crate::{
    csaf::raw::RawDocument,
    helpers::{
        CSAF_2_0_SCHEMA, CSAF_2_1_SCHEMA, CVSS_V2_SCHEMA, CVSS_V2_SCHEMA_URL, CVSS_V3_0_SCHEMA, CVSS_V3_0_SCHEMA_URL,
        CVSS_V3_1_SCHEMA, CVSS_V3_1_SCHEMA_URL, CVSS_V4_0_1_SCHEMA, CVSS_V4_0_1_SCHEMA_URL, SSVC_2_SCHEMA,
        SSVC_2_SCHEMA_URL,
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
        .with_resource(CVSS_V2_SCHEMA_URL, Resource::from_contents(CVSS_V2_SCHEMA.clone()))
        .with_resource(CVSS_V3_0_SCHEMA_URL, Resource::from_contents(CVSS_V3_0_SCHEMA.clone()))
        .with_resource(CVSS_V3_1_SCHEMA_URL, Resource::from_contents(CVSS_V3_1_SCHEMA.clone()))
        .with_resource(
            CVSS_V4_0_1_SCHEMA_URL,
            Resource::from_contents(use_draft_schema(CVSS_V4_0_1_SCHEMA.clone())),
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

fn validate_schema<T>(
    document: &RawDocument<T>,
    validator: &jsonschema::Validator,
) -> Result<(), Vec<ValidationError>> {
    let errors: Vec<_> = validator
        .iter_errors(document.get_json())
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
    validate_schema(document, &VALIDATOR_2_0)
}

pub fn validate_schema_csaf_2_1(
    document: &RawDocument<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>,
) -> Result<(), Vec<ValidationError>> {
    validate_schema(document, &VALIDATOR_2_1)
}
