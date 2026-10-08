use serde_json::Value;

use crate::csaf_traits::{
    BranchTrait, ContentTrait, CsafTrait, DocumentTrait, ExtensionsTrait, MetricTrait, ProductPathTrait, ProductTrait,
    ProductTreeTrait, VulnerabilityTrait,
};
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

fn validate_schema_extension_content(json: &Value, path: &str) -> Vec<TestFinding> {
    ValidatorExtensionContent2_1::iter_errors(json)
        .map(|err| {
            TestFinding::Error(TestFindingData {
                message: err.to_string(),
                instance_path: path.to_string(),
            })
        })
        .collect()
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

            let extension_errors =
                validate_schema_extension_content(&serde_json::Value::Object(extension.clone()), &path);

            if !extension_errors.is_empty() {
                errors.get_or_insert_default().extend(extension_errors);
            }
        }
    }

    errors.map_or(Ok(()), Err)
}

fn collect_extensions(doc: &impl CsafTrait) -> Vec<(&dyn ExtensionsTrait, String)> {
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
            case_s01: Err(vec![
                extension_invalid_content_schema_missing_property_error("/x_extensions/0", "$schema"),
                extension_invalid_content_schema_missing_property_error("/x_extensions/0", "category"),
                extension_invalid_content_schema_missing_property_error("/x_extensions/0", "critical"),
                extension_invalid_content_schema_missing_property_error("/x_extensions/0", "content"),
                extension_invalid_content_schema_missing_property_error("/document/x_extensions/0", "$schema"),
                extension_invalid_content_schema_missing_property_error("/document/x_extensions/0", "category"),
                extension_invalid_content_schema_missing_property_error("/document/x_extensions/0", "critical"),
                extension_invalid_content_schema_missing_property_error("/document/x_extensions/0", "content"),
                extension_invalid_content_schema_missing_property_error("/vulnerabilities/0/x_extensions/0", "$schema"),
                extension_invalid_content_schema_missing_property_error(
                    "/vulnerabilities/0/x_extensions/0",
                    "category",
                ),
                extension_invalid_content_schema_missing_property_error(
                    "/vulnerabilities/0/x_extensions/0",
                    "critical",
                ),
                extension_invalid_content_schema_missing_property_error("/vulnerabilities/0/x_extensions/0", "content"),
                extension_invalid_content_schema_missing_property_error(
                    "/vulnerabilities/0/metrics/0/content/x_extensions/0",
                    "$schema",
                ),
                extension_invalid_content_schema_missing_property_error(
                    "/vulnerabilities/0/metrics/0/content/x_extensions/0",
                    "category",
                ),
                extension_invalid_content_schema_missing_property_error(
                    "/vulnerabilities/0/metrics/0/content/x_extensions/0",
                    "critical",
                ),
                extension_invalid_content_schema_missing_property_error(
                    "/vulnerabilities/0/metrics/0/content/x_extensions/0",
                    "content",
                ),
                extension_invalid_content_schema_missing_property_error(
                    "/product_tree/branches/0/product/x_extensions/0",
                    "$schema",
                ),
                extension_invalid_content_schema_missing_property_error(
                    "/product_tree/branches/0/product/x_extensions/0",
                    "category",
                ),
                extension_invalid_content_schema_missing_property_error(
                    "/product_tree/branches/0/product/x_extensions/0",
                    "critical",
                ),
                extension_invalid_content_schema_missing_property_error(
                    "/product_tree/branches/0/product/x_extensions/0",
                    "content",
                ),
                extension_invalid_content_schema_missing_property_error(
                    "/product_tree/full_product_names/0/x_extensions/0",
                    "$schema",
                ),
                extension_invalid_content_schema_missing_property_error(
                    "/product_tree/full_product_names/0/x_extensions/0",
                    "category",
                ),
                extension_invalid_content_schema_missing_property_error(
                    "/product_tree/full_product_names/0/x_extensions/0",
                    "critical",
                ),
                extension_invalid_content_schema_missing_property_error(
                    "/product_tree/full_product_names/0/x_extensions/0",
                    "content",
                ),
                extension_invalid_content_schema_missing_property_error(
                    "/product_tree/product_paths/0/full_product_name/x_extensions/0",
                    "$schema",
                ),
                extension_invalid_content_schema_missing_property_error(
                    "/product_tree/product_paths/0/full_product_name/x_extensions/0",
                    "category",
                ),
                extension_invalid_content_schema_missing_property_error(
                    "/product_tree/product_paths/0/full_product_name/x_extensions/0",
                    "critical",
                ),
                extension_invalid_content_schema_missing_property_error(
                    "/product_tree/product_paths/0/full_product_name/x_extensions/0",
                    "content",
                ),
            ]),
        });
    }
}
