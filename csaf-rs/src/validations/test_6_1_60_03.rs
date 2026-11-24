use crate::csaf::extension_registry::get_extension_metadata;
use crate::csaf_traits::{CsafTrait, ExtensionsTrait};
use crate::validation::{TestFinding, TestFindingData};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

fn extension_metadata_not_supported_warning(
    schema: &str,
    category: Option<&str>,
    instance_path: impl Into<String>,
) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!(
            "The {} extension `{schema}` is not supported by this implementation, its metadata can't be checked",
            category.unwrap_or("uncategorized")
        ),
        instance_path: instance_path.into(),
    })
}

fn extension_not_allowed_error(schema: &str, requirement_path: &str, instance_path: impl Into<String>) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("The extension `{schema}` is not allowed in `{requirement_path}`"),
        instance_path: instance_path.into(),
    })
}

fn extension_used_too_often_error(
    schema: &str,
    requirement_path: &str,
    occurrences: usize,
    max_occurrence: u64,
    instance_path: impl Into<String>,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "The extension `{schema}` is used {occurrences} times in `{requirement_path}`, but at most {max_occurrence} are allowed"
        ),
        instance_path: instance_path.into(),
    })
}

fn incompatible_extension_error(
    schema: &str,
    incompatible_schema: &str,
    incompatible_path: &str,
    instance_path: impl Into<String>,
) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "The extension `{schema}` is not compatible with the extension `{incompatible_schema}` used in `{incompatible_path}`"
        ),
        instance_path: instance_path.into(),
    })
}

/// Maps the path of an `x_extensions` array in the document to the JSONPath that is used as key in the `requirement`
/// of the extension metadata.
fn requirement_path(instance_path: &str) -> Option<&'static str> {
    if instance_path == "/x_extensions" {
        Some("$.x_extensions")
    } else if instance_path == "/document/x_extensions" {
        Some("$.document.x_extensions")
    } else if instance_path.starts_with("/vulnerabilities/") {
        if instance_path.ends_with("/content/x_extensions") {
            Some("$.vulnerabilities[*].metrics[*].content.x_extensions")
        } else {
            Some("$.vulnerabilities[*].x_extensions")
        }
    } else if instance_path.starts_with("/product_tree/branches/") {
        Some("$.product_tree.branches..product.x_extensions")
    } else if instance_path.starts_with("/product_tree/full_product_names/") {
        Some("$.product_tree.full_product_names[*].x_extensions")
    } else if instance_path.starts_with("/product_tree/product_paths/") {
        // named `relationships` in the extension metadata schema
        Some("$.product_tree.relationships[*].full_product_name.x_extensions")
    } else {
        None
    }
}

/// 6.1.60.3 Extension Metadata
///
/// For each element of type $['$defs'].extensions_t it SHALL be tested that the requirements provided through its metadata are fulfilled
/// for each CSAF Extension used.
/// CSAF Extensions not supported by the implementation SHALL result in a warning which SHALL include the value of $schema of the extension.
/// Such warning SHALL differentiate between the different classes of extensions.
///
/// This includes that the extension is allowed at the path at all, that it is not used more often than allowed
/// and that it is not used together with incompatible extensions anywhere in the document.
///
/// An extension is supported if its extension metadata was registered with
/// [`crate::csaf::extension_registry`] before the validation. The metadata is never fetched from the network.
pub fn test_6_1_60_3_extension_metadata(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut findings: Option<Vec<TestFinding>> = None;
    // all used extensions in the document, as (`$schema`, instance path), to check for incompatible extensions
    let mut used_extensions: Vec<(String, String)> = Vec::new();

    doc.visit_all_extensions(&mut |extensions, base_path| {
        // number of occurrences of each extension in this location
        let mut occurrences: BTreeMap<&str, Vec<usize>> = BTreeMap::new();

        for (extension_index, extension) in extensions.get_raw().iter().enumerate() {
            // a missing or malformed `$schema` is reported by 6.1.60.1
            let Some(schema) = extension.get("$schema").and_then(Value::as_str) else {
                continue;
            };
            occurrences.entry(schema).or_default().push(extension_index);
            used_extensions.push((schema.to_string(), format!("{base_path}/{extension_index}")));
        }

        for (schema, indices) in occurrences {
            let Some(metadata) = get_extension_metadata(schema) else {
                for index in indices {
                    let category = extensions.get_raw()[index].get("category").and_then(Value::as_str);
                    findings
                        .get_or_insert_default()
                        .push(extension_metadata_not_supported_warning(
                            schema,
                            category,
                            format!("{base_path}/{index}"),
                        ));
                }
                continue;
            };

            let Some(requirement_path) = requirement_path(base_path) else {
                continue;
            };
            // the metadata schema requires a requirement for each path, nothing is allowed if there is none
            let max_occurrence = metadata.max_occurrence.get(requirement_path).copied().unwrap_or(0);
            if max_occurrence == 0 {
                findings
                    .get_or_insert_default()
                    .push(extension_not_allowed_error(schema, requirement_path, base_path));
            } else if indices.len() as u64 > max_occurrence {
                findings.get_or_insert_default().push(extension_used_too_often_error(
                    schema,
                    requirement_path,
                    indices.len(),
                    max_occurrence,
                    base_path,
                ));
            }
        }
    });

    // incompatible extensions are checked in the whole document, every pair is reported once
    let mut reported_pairs: BTreeSet<(usize, usize)> = BTreeSet::new();
    for (index, (schema, instance_path)) in used_extensions.iter().enumerate() {
        let Some(metadata) = get_extension_metadata(schema) else {
            continue;
        };
        for (other_index, (other_schema, other_path)) in used_extensions.iter().enumerate() {
            if other_index != index
                && metadata.incompatible_extensions.contains(other_schema)
                && reported_pairs.insert((index.min(other_index), index.max(other_index)))
            {
                findings.get_or_insert_default().push(incompatible_extension_error(
                    schema,
                    other_schema,
                    other_path,
                    instance_path,
                ));
            }
        }
    }

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_60_3, test_6_1_60_3_extension_metadata);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::{ExpectedResults_6_1_60_3 as ExpectedResults, TESTS_2_1};
    use crate::validations::test_6_1_60_02::tests::register_oasis_test_extensions;

    const DOCUMENTATION_11: &str = "https://raw.githubusercontent.com/oasis-tcs/csaf/refs/heads/master/csaf_2.1/test/extension/data/valid/documentation-11/documentation-11-content_1.0.0.json";
    const DOCUMENTATION_12: &str = "https://raw.githubusercontent.com/oasis-tcs/csaf/refs/heads/master/csaf_2.1/test/extension/data/valid/documentation-12/documentation-12-content_1.0.0.json";

    #[test]
    fn test_6_1_60_3() {
        register_oasis_test_extensions();

        TESTS_2_1.test_6_1_60_3.expect(ExpectedResults {
            case_01: Err(vec![extension_used_too_often_error(
                DOCUMENTATION_11,
                "$.x_extensions",
                2,
                1,
                "/x_extensions",
            )]),
            case_02: Err(vec![incompatible_extension_error(
                DOCUMENTATION_12,
                DOCUMENTATION_11,
                "/x_extensions/1",
                "/x_extensions/0",
            )]),
            // Supplementary case: `documentation-12` is not allowed in the content of a metric
            case_s01: Err(vec![extension_not_allowed_error(
                DOCUMENTATION_12,
                "$.vulnerabilities[*].metrics[*].content.x_extensions",
                "/vulnerabilities/0/metrics/0/content/x_extensions",
            )]),
            case_11: Ok(()),
            case_12: Ok(()),
            // Supplementary case: an extension without registered metadata results in a warning
            case_s11: Err(vec![extension_metadata_not_supported_warning(
                "https://example.com/csaf/extension/unknown_1.0.0.json",
                Some("significant"),
                "/document/x_extensions/0",
            )]),
        });
    }
}
