use serde_json::Value;

use crate::csaf_traits::{
    BranchTrait, ContentTrait, CsafTrait, DocumentTrait, ExtensionsTrait, MetricTrait, ProductPathTrait, ProductTrait,
    ProductTreeTrait, VulnerabilityTrait,
};
use crate::validation::{TestFinding, TestFindingData};

fn extension_invalid_content_schema_missing_property_error(
    instance_path: impl Into<String>,
    name: &str,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("The extension is missing required property {name}"),
        instance_path: instance_path.into(),
    })
}

fn extension_invalid_content_schema_wrong_type_error(
    instance_path: impl Into<String>,
    name: &str,
    expected: &str,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("The property {name} of the extension has the wrong type. Expected type `{expected}`."),
        instance_path: instance_path.into(),
    })
}

fn extension_invalid_content_schema_unknown_property_error(
    instance_path: impl Into<String>,
    name: &str,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("The extension has an additional unknown property {name}."),
        instance_path: instance_path.into(),
    })
}

/// Content Schema
///
/// For each item in an element of type $['$defs'].extensions_t it SHALL be tested that the item is valid against the Extension Content Schema.
pub fn test_6_1_60_1_extension_content_schema(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    if doc.get_document().get_csaf_version() == crate::csaf_traits::CsafVersion::X20 {
        // CSAF 2.0 does not support extensions
        return Ok(());
    }

    // Collect extensions from all the possible paths within the CSAF document
    let collected_extensions = collect_extensions(doc);

    let mut errors: Option<Vec<TestFinding>> = None;
    for (extensions, path_base) in collected_extensions {
        // Iterate over all the individual extensions and check their validity
        for (idx, extension) in extensions.get_raw().iter().enumerate() {
            let path = format!("{path_base}/{idx}");

            let expected_properties = [
                ("$schema", Value::is_string as fn(&Value) -> bool, "String"),
                ("category", check_category_enum, "String.Enum"),
                ("content", Value::is_object, "Mapping"),
                ("critical", Value::is_boolean, "Boolean"),
            ];
            for (property_name, check_type, expected_type) in expected_properties {
                if let Some(property) = extension.get(property_name) {
                    if !check_type(property) {
                        errors
                            .get_or_insert_default()
                            .push(extension_invalid_content_schema_wrong_type_error(
                                &path,
                                property_name,
                                expected_type,
                            ));
                    }
                } else {
                    errors
                        .get_or_insert_default()
                        .push(extension_invalid_content_schema_missing_property_error(
                            &path,
                            property_name,
                        ));
                }
            }

            for property_name in extension.keys() {
                if expected_properties
                    .iter()
                    .all(|(expected_property, ..)| property_name != expected_property)
                {
                    errors
                        .get_or_insert_default()
                        .push(extension_invalid_content_schema_unknown_property_error(
                            &path,
                            property_name,
                        ));
                }
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

fn collect_extensions<DOC>(doc: &DOC) -> Vec<(&dyn ExtensionsTrait, String)>
where
    DOC: CsafTrait,
{
    let mut collected_extensions: Vec<(&dyn ExtensionsTrait, String)> = Vec::new();

    // $.x_extensions[*]
    if let Some(extensions) = doc.get_extensions() {
        collected_extensions.push((extensions, "/x_extensions".into()));
    }

    // $.document.x_extensions[*]
    if let Some(extensions) = doc.get_document().get_extensions() {
        collected_extensions.push((extensions, "/document/x_extensions".into()));
    }

    let vulnerabilities = doc.get_vulnerabilities();

    for (i, vuln) in vulnerabilities.iter().enumerate() {
        // $.vulnerabilities[*].x_extensions[*]
        if let Some(extensions) = vuln.get_extensions() {
            collected_extensions.push((extensions, format!("/vulnerabilities/{i}/x_extensions")));
        }

        // $.vulnerabilities[*].metrics[*].content.x_extensions[*]
        if let Some(metrics) = vuln.get_metrics() {
            for (m_i, metric) in metrics.iter().enumerate() {
                if let Some(extensions) = metric.get_content().get_extensions() {
                    collected_extensions.push((
                        extensions,
                        format!("/vulnerabilities/{i}/metrics/{m_i}/content/x_extensions"),
                    ));
                }
            }
        }
    }

    if let Some(product_tree) = doc.get_product_tree() {
        // $.product_tree.branches[*].product.x_extensions[*]
        product_tree.get_branches().inspect(|branches| {
            for (i, branch) in branches.iter().enumerate() {
                if let Some(product) = branch.get_product()
                    && let Some(extensions) = product.get_extensions()
                {
                    collected_extensions.push((extensions, format!("/product_tree/branches/{i}/product/x_extensions")));
                }
            }
        });

        // $.product_tree.full_product_names[*].x_extensions[*]
        for (i, full_product_name) in product_tree.get_full_product_names().iter().enumerate() {
            if let Some(extensions) = full_product_name.get_extensions() {
                collected_extensions.push((extensions, format!("/product_tree/full_product_names/{i}/x_extensions")));
            }
        }

        // $.product_tree.product_paths[*].full_product_name.x_extensions[*]
        for (i, product_path) in product_tree.get_product_paths().iter().enumerate() {
            if let Some(extensions) = product_path.get_full_product_name().get_extensions() {
                collected_extensions.push((
                    extensions,
                    format!("/product_tree/product_paths/{i}/full_product_name/x_extensions"),
                ));
            }
        }
    }

    collected_extensions
}

/// Check that category enum is of type String and matches the allowed value,
/// as specified in `2.4.4.2 Content Schema Property - Category`.
fn check_category_enum(v: &Value) -> bool {
    matches!(v.as_str(), Some("essential" | "supplementary" | "significant"))
}

crate::test_validation::impl_validator!(
    csaf2_1,
    ValidatorForTest6_1_60_1,
    test_6_1_60_1_extension_content_schema
);

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{
        csaf::raw::HasParsed,
        csaf2_1::{
            loader::load_document,
            testcases::{ExpectedResults_6_1_60_1 as ExpectedResults, TESTS_2_1},
        },
        validations::test_6_1_60_1::{
            collect_extensions, extension_invalid_content_schema_missing_property_error,
            extension_invalid_content_schema_unknown_property_error,
        },
    };

    #[test]
    fn test_6_1_60_1() {
        TESTS_2_1.test_6_1_60_1.expect(ExpectedResults {
            // Case 01: Missing property "critical"
            case_01: Err(vec![extension_invalid_content_schema_missing_property_error(
                "/x_extensions/0",
                "critical",
            )]),
            // Case 02: Additional unknown property "some_additional_property"
            case_02: Err(vec![extension_invalid_content_schema_unknown_property_error(
                "/x_extensions/0",
                "some_additional_property",
            )]),
            // Case 11:
            case_11: Ok(()),
            // Case 12:
            case_12: Ok(()),
        });
    }

    #[test]
    fn test_extension_collection() {
        let csaf_doc = include_str!("test_6_1_60_1/all_extension_locations.json");
        let raw_doc =
            load_document(csaf_doc).expect("Failed to load CSAF test document containing all extension locations");
        let doc = raw_doc.get_parsed().as_ref().expect("Failed to parse CSAF document");

        let collected_extensions: HashSet<_> = collect_extensions(doc).iter().map(|(_, path)| path.clone()).collect();

        assert_eq!(
            collected_extensions,
            [
                "/x_extensions",
                "/document/x_extensions",
                "/vulnerabilities/0/x_extensions",
                "/vulnerabilities/0/metrics/0/content/x_extensions",
                "/product_tree/branches/0/product/x_extensions",
                "/product_tree/full_product_names/0/x_extensions",
                "/product_tree/product_paths/0/full_product_name/x_extensions",
            ]
            .into_iter()
            .map(|p| p.to_string())
            .collect(),
            "Did not return the expected extension locations"
        )
    }
}
