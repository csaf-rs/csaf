use anyhow::{Result, bail};

use csaf_rs::csaf2_0::loader::load_document as load_document_2_0;
use csaf_rs::csaf2_1::loader::load_document as load_document_2_1;
use csaf_rs::{
    csaf::loader::detect_version_with,
    json::JsonSource,
    validation::{Validatable, ValidationResult, validate_by_tests},
};

#[cfg(feature = "cli")]
pub mod cli;

/// Validate a CSAF document of the specified version with the provided arguments.
///
/// This prints the results of the tests on stdout.
pub fn validate_document<T>(document: T, csaf_version: &str, tests: &[String]) -> ValidationResult
where
    T: Validatable,
{
    let test_ids: Vec<_> = tests
        .iter()
        .flat_map(|test_or_preset| {
            let presets = T::get_presets();
            // Try to find a matching preset for the document version
            let matched_preset = presets
                .iter()
                .find(|p| p.to_string().eq_ignore_ascii_case(test_or_preset));
            match matched_preset {
                Some(preset) => T::tests_in_preset(preset).unwrap_or_default(),
                // no matching preset found, treat the argument as a test ID
                None => vec![test_or_preset.as_str()],
            }
        })
        .collect();

    validate_by_tests(&document, csaf_version, &test_ids)
}

/// Try to validate the content as a CSAF document based on the specified version.
/// The content can be anything which resembles a `JsonSource`.
pub fn validate<S: JsonSource>(content: S, csaf_version: &str, tests: &[String]) -> Result<ValidationResult> {
    match csaf_version {
        "auto" => {
            let detected = detect_version_with(content)?;
            load_and_validate(detected.data, &detected.version, tests)
        },
        other => load_and_validate(content, other, tests),
    }
}

/// Load a document of the given version from any JSON source and validate it.
fn load_and_validate<S: JsonSource>(source: S, version: &str, tests: &[String]) -> Result<ValidationResult> {
    match version {
        "2.0" => {
            let document = load_document_2_0(source)?;
            let result = validate_document(document, "2.0", tests);
            Ok(result)
        },
        "2.1" => {
            let document = load_document_2_1(source)?;
            let result = validate_document(document, "2.1", tests);
            Ok(result)
        },
        _ => bail!("Invalid CSAF version: {version}"),
    }
}
