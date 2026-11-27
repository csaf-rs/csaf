use crate::csaf::extension_registry::get_extension_validator;
use crate::csaf_traits::{CsafTrait, ExtensionsTrait};
use crate::validation::{TestFinding, TestFindingData};
use serde_json::Value;

fn extension_not_supported_warning(
    schema: &str,
    category: Option<&str>,
    instance_path: impl Into<String>,
) -> TestFinding {
    TestFinding::Warning(TestFindingData {
        message: format!(
            "The {} extension `{schema}` is not supported by this implementation, its extension schema can't be checked",
            category.unwrap_or("uncategorized")
        ),
        instance_path: instance_path.into(),
    })
}

fn extension_invalid_error(schema: &str, message: String, instance_path: impl Into<String>) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!("The extension is not valid against its declared extension schema `{schema}`: {message}"),
        instance_path: instance_path.into(),
    })
}

/// 6.1.60.2 Extension Schema
///
/// For each item in an element of type $['$defs'].extensions_t it SHALL be tested that the item is valid against the declared CSAF Extension Schema.
/// CSAF Extensions not supported by the implementation SHALL result in a warning which SHALL include the value of $schema of the extension.
/// Such warning SHALL differentiate between the different classes of extensions.
///
/// An extension is supported if its extension schema was registered with
/// [`crate::csaf::extension_registry`] before the validation. The schema is never fetched from the network.
pub fn test_6_1_60_2_extension_schema(doc: &impl CsafTrait) -> Result<(), Vec<TestFinding>> {
    let mut findings: Option<Vec<TestFinding>> = None;
    doc.visit_all_extensions(&mut |extensions, base_path| {
        for (extension_index, extension) in extensions.get_raw().iter().enumerate() {
            // a missing or malformed `$schema` is reported by 6.1.60.1
            let Some(schema) = extension.get("$schema").and_then(Value::as_str) else {
                continue;
            };
            let instance_path = format!("{base_path}/{extension_index}");

            match get_extension_validator(schema) {
                Some(validator) => {
                    let instance = Value::Object(extension.clone());
                    for error in validator.iter_errors(&instance) {
                        findings.get_or_insert_default().push(extension_invalid_error(
                            schema,
                            error.to_string(),
                            format!("{instance_path}{}", error.instance_path()),
                        ));
                    }
                },
                None => {
                    let category = extension.get("category").and_then(Value::as_str);
                    findings.get_or_insert_default().push(extension_not_supported_warning(
                        schema,
                        category,
                        instance_path,
                    ));
                },
            }
        }
    });

    findings.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_60_2, test_6_1_60_2_extension_schema);

#[cfg(test)]
pub(crate) mod tests {
    use super::{extension_invalid_error, extension_not_supported_warning};
    use crate::csaf::extension_registry::register_extensions_from_dir;
    use crate::csaf2_1::testcases::{ExpectedResults_6_1_60_2 as ExpectedResults, TESTS_2_1};
    use std::path::Path;
    use std::sync::Once;

    /// Registers the extensions for documentation purposes shipped with the OASIS test data.
    pub(crate) fn register_oasis_test_extensions() {
        static REGISTER: Once = Once::new();
        REGISTER.call_once(|| {
            let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../oasis-csaf/csaf_2.1/test/extension/data/valid");
            assert_eq!(register_extensions_from_dir(&dir).unwrap(), 6);
        });
    }

    #[test]
    fn test_6_1_60_2() {
        register_oasis_test_extensions();

        TESTS_2_1.test_6_1_60_2.expect(ExpectedResults {
            case_01: Err(vec![extension_invalid_error(
                "https://raw.githubusercontent.com/oasis-tcs/csaf/refs/heads/master/csaf_2.1/test/extension/data/valid/documentation-12/documentation-12-content_1.0.0.json",
                "\"documentation\" is a required property".to_string(),
                "/x_extensions/0/content",
            )]),
            case_02: Err(vec![extension_invalid_error(
                "https://raw.githubusercontent.com/oasis-tcs/csaf/refs/heads/master/csaf_2.1/test/extension/data/valid/documentation-11/documentation-11-content_1.0.0.json",
                "Unevaluated properties are not allowed ('notes' was unexpected)".to_string(),
                "/x_extensions/0/content",
            )]),
            case_11: Ok(()),
            case_12: Ok(()),
            // Supplementary case: extensions without registered extension schema result in a warning that names the
            // extension and its category
            case_s11: Err(vec![
                extension_not_supported_warning(
                    "https://example.com/csaf/extension/unknown-essential_1.0.0.json",
                    Some("essential"),
                    "/x_extensions/0",
                ),
                extension_not_supported_warning(
                    "https://example.com/csaf/extension/unknown-supplementary_1.0.0.json",
                    Some("supplementary"),
                    "/x_extensions/1",
                ),
            ]),
        });
    }
}
