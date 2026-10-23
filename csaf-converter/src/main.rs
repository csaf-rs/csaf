mod args;

use std::borrow::Cow;

use anyhow::bail;
use args::Args;
use clap::Parser;
use csaf::{
    converter::{
        Diagnostics,
        Error::{self, ValidationError},
        Warning::{self, ValidationWarning},
    },
    csaf::raw::RawDocument,
    json::JsonSource,
    validation::TestResultStatus,
};

pub struct VerboseDiagnostics {
    has_errors: bool,
}
impl Diagnostics for VerboseDiagnostics {
    fn error(&mut self, path: Cow<'static, str>, error: Error) {
        self.has_errors = true;
        eprintln!("Error: {error} ({path})");
    }

    fn warning(&mut self, path: Cow<'static, str>, warning: Warning) {
        eprintln!("Warning: {warning} ({path})");
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let raw: serde_json::Value = args.path.as_path().parse()?;
    let mut diag = VerboseDiagnostics { has_errors: false };

    let validation_result = csaf::validation::validate_by_preset(
        &RawDocument::<csaf::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework>::new(raw.clone()),
        "2.0",
        "basic",
    )?;

    validation_result
        .test_results
        .iter()
        .for_each(|result| match &result.status {
            TestResultStatus::Failure {
                errors,
                warnings,
                infos: _,
            } => {
                errors.iter().for_each(|error| {
                    diag.error(
                        error.instance_path.clone().into(),
                        ValidationError(error.message.clone()),
                    );
                });
                warnings.iter().for_each(|warning| {
                    diag.warning(
                        warning.instance_path.clone().into(),
                        ValidationWarning(warning.message.clone()),
                    );
                });
            },
            TestResultStatus::NotFound => {
                diag.error("".into(), ValidationError("Validation preset not found.".into()));
            },
            _ => {},
        });

    if diag.has_errors && !args.allow_invalid {
        bail!(
            "Conversion aborted due to validation errors in input file. Run with --allow-invalid to convert the file anyway."
        );
    }

    let settings = csaf::converter::Settings {
        publisher_category_other: csaf::converter::PublisherCategoryOtherSettings {
            suppress_warning: args.publisher_category_other.suppress_warning,
            convert_to_multiplier: args.publisher_category_other.convert_to_multiplier,
        },
    };

    let result = csaf::converter::convert(raw, &settings, &mut diag);
    if diag.has_errors {
        if args.allow_invalid {
            eprintln!("Conversion completed with errors, but allowed due to --allow-invalid.");
        } else {
            bail!("Conversion failed due to errors. Run with --allow-invalid to output the converted file anyway.");
        }
    }

    let result = serde_json::to_string_pretty(&result.into_raw())?;
    if let Some(path) = args.output {
        std::fs::write(path.as_path(), result)?;
        println!("Wrote converted file to {}.", path.display());
    } else {
        println!("{result}");
    }
    Ok(())
}
