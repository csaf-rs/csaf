use std::path::Path;

use anyhow::Error;
use csaf_rs::{
    converter::{Diagnostics, Error::ValidationError, Settings, Warning::ValidationWarning},
    csaf::raw::RawDocument,
    json::JsonSource,
    validation::TestResultStatus,
};

#[cfg(feature = "cli")]
pub mod cli;

pub fn convert(
    document: serde_json::Value,
    settings: &Settings,
    diagnostics: &mut dyn Diagnostics,
    allow_invalid: bool,
) -> Result<serde_json::Value, anyhow::Error> {
    let validation_result = csaf_rs::validation::validate_by_preset(
        &RawDocument::<csaf_rs::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>::new(document.clone()),
        "2.0",
        "basic",
    )?;

    validation_result
        .test_results
        .iter()
        .for_each(|result| match &result.status {
            TestResultStatus::Failure {
                errors,
                warnings: _, // warnings are irrelevant here as we are calling the `basic` preset
                infos: _,    // infos are irrelevant here as we are calling the `basic` preset
            } => {
                errors.iter().for_each(|error| {
                    diagnostics.error(
                        error.instance_path.clone().into(),
                        ValidationError(error.message.clone()),
                    );
                });
            },
            TestResultStatus::NotFound => {
                diagnostics.warning(
                    "".into(),
                    ValidationWarning(format!("Test `{0}` not found.", result.test_id)),
                );
            },
            _ => {},
        });

    if diagnostics.has_errors() && !allow_invalid {
        return Err(Error::msg(
            "Conversion aborted due to validation errors in input file. Run with --allow-invalid to convert the file anyway.",
        ));
    }

    let result = csaf_rs::converter::convert(document, settings, diagnostics);
    if diagnostics.has_errors() && !allow_invalid {
        return Err(Error::msg(
            "Conversion failed due to errors. Run with --allow-invalid to output the converted file anyway.",
        ));
    }

    Ok(result.into_raw())
}

pub fn convert_file(
    path: &Path,
    output: &Path,
    settings: &Settings,
    diagnostics: &mut dyn Diagnostics,
    allow_invalid: bool,
) -> Result<(), anyhow::Error> {
    let raw: serde_json::Value = path.parse()?;
    let converted = convert(raw, settings, diagnostics, allow_invalid)?;
    let result = serde_json::to_string_pretty(&converted)?;
    std::fs::write(output, result)?;
    Ok(())
}
