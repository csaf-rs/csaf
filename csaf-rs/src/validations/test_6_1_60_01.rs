use serde_json::Value;

use crate::csaf_traits::{CsafTrait, ExtensionsTrait};
use crate::validation::{TestFinding, TestFindingData};

#[jsonschema::validator(
    path = "assets/extension-content.json",
    validate_formats = true,
    resources = {
        "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/meta.json" => { path = "assets/meta.json" },
        "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/extension-metaschema.json" => { path = "assets/extension-metaschema.json" },
    }
)]
struct ValidatorExtensionContent2_1;

/// 6.1.60.1 Content Schema
///
/// For each item in an element of type $['$defs'].extensions_t it SHALL be tested that the item is valid against the Extension Content Schema.
pub fn test_6_1_60_1_extension_content_schema(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut errors: Option<Vec<TestFinding>> = None;
    doc.visit_all_extensions(&mut |extensions, base_path| {
        // Iterate over all the individual extensions and check their validity
        for (extension_index, extension) in extensions.get_raw().iter().enumerate() {
            for err in ValidatorExtensionContent2_1::iter_errors(&Value::Object(extension.clone())) {
                errors.get_or_insert_default().push(TestFinding::Error(TestFindingData {
                    message: err.to_string(),
                    instance_path: format!("{base_path}/{extension_index}"),
                }));
            }
        }
    });

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_60_1,
    test_6_1_60_1_extension_content_schema
);

#[cfg(test)]
mod tests {
    use crate::{
        csaf2_1::testcases::{ExpectedResults_6_1_60_1 as ExpectedResults, TESTS_2_1},
        validation::{TestFinding, TestFindingData},
    };

    fn extension_invalid_content_schema_missing_property_error(
        instance_path: impl Into<String>,
        name: &str,
    ) -> TestFinding {
        TestFinding::Error(TestFindingData {
            message: format!("\"{name}\" is a required property"),
            instance_path: instance_path.into(),
        })
    }

    fn extension_invalid_content_schema_unknown_property_error(
        instance_path: impl Into<String>,
        name: &str,
    ) -> TestFinding {
        TestFinding::Error(TestFindingData {
            message: format!("Additional properties are not allowed ('{name}' was unexpected)"),
            instance_path: instance_path.into(),
        })
    }

    #[test]
    fn test_6_1_60_1() {
        let missing_prop = Err(vec![extension_invalid_content_schema_missing_property_error(
            "/x_extensions/0",
            "critical",
        )]);

        let additional_prop = Err(vec![extension_invalid_content_schema_unknown_property_error(
            "/x_extensions/0",
            "some_additional_property",
        )]);

        let all_paths_with_missing_root_with_multiple_findings = Err(vec![
            extension_invalid_content_schema_missing_property_error("/x_extensions/0", "critical"),
            extension_invalid_content_schema_unknown_property_error("/x_extensions/1", "some_additional_property"),
            extension_invalid_content_schema_missing_property_error("/document/x_extensions/0", "critical"),
            extension_invalid_content_schema_missing_property_error("/vulnerabilities/0/x_extensions/0", "critical"),
            extension_invalid_content_schema_missing_property_error(
                "/vulnerabilities/0/metrics/0/content/x_extensions/0",
                "critical",
            ),
            extension_invalid_content_schema_missing_property_error(
                "/product_tree/branches/0/product/x_extensions/0",
                "critical",
            ),
            extension_invalid_content_schema_missing_property_error(
                "/product_tree/branches/0/branches/0/product/x_extensions/0",
                "critical",
            ),
            extension_invalid_content_schema_missing_property_error(
                "/product_tree/full_product_names/0/x_extensions/0",
                "critical",
            ),
            extension_invalid_content_schema_missing_property_error(
                "/product_tree/product_paths/0/full_product_name/x_extensions/0",
                "critical",
            ),
        ]);

        // Case 11: Case 01, without the missing critical property
        // Case 12: Case 02, without additional props and with notes array

        TESTS_2_1.test_6_1_60_1.expect(ExpectedResults {
            case_01: missing_prop,
            case_02: additional_prop,
            case_s01: all_paths_with_missing_root_with_multiple_findings,
            case_11: Ok(()),
            case_12: Ok(()),
            case_s11: Ok(()),
        });
    }
}
