use std::{borrow::Cow, path::PathBuf};

use anyhow::bail;
use clap::Parser;
use csaf::{
    converter::{
        Diagnostics,
        Error::{self, InputValidationError, OutputValidationError},
        Settings,
        Warning::{self, ValidationWarning},
    },
    csaf::raw::RawDocument,
    json::JsonSource,
    schema::csaf2_1::schema::EntityCategory,
    validation::TestResultStatus,
    validations::test_schema::validate_schema_csaf_2_1,
};

/// A converter for CSAF 2.0 to 2.1 documents
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to CSAF document to convert
    #[arg()]
    path: PathBuf,

    /// Path to the output file, stdout when omitted
    #[arg(short = 'o')]
    output: Option<PathBuf>,

    /// Allow (partial) processing and output of invalid CSAF documents
    #[arg(long)]
    allow_invalid: bool,

    /// User-provided involvement element
    #[arg(long)]
    involvement: Option<serde_json::Value>,

    /// Disable warning for involvement category 'other' usage
    #[arg(long)]
    disable_involvement_category_other_warning: bool,

    /// User-provided entity category for `other` parties
    #[arg(long)]
    involvement_category_other: Option<EntityCategory>,

    /// User-provided entity mapping
    #[arg(long)]
    entities: Option<serde_json::Value>,

    /// Create incomplete actions (e.g. no date) if necessary
    #[arg(long)]
    create_incomplete_actions: bool,

    /// Extract further entity details from involvement elements
    #[arg(long)]
    involvement_extract_entity_details: bool,

    /// Collapse entities during conversion
    #[arg(long)]
    collapse_entities: bool,

    /// Collapse actions during conversion
    #[arg(long)]
    collapse_actions: bool,
}

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
                        InputValidationError(error.message.clone()),
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
                diag.error("".into(), InputValidationError("Validation preset not found.".into()));
            },
            _ => {},
        });

    if diag.has_errors && !args.allow_invalid {
        bail!(
            "Conversion aborted due to validation errors in input file. Run with --allow-invalid to convert the file anyway."
        );
    }

    let settings = Settings {
        collapse_actions: args.collapse_actions,
        collapse_entities: args.collapse_entities,
        create_incomplete_actions: args.create_incomplete_actions,
        involvement_extract_entity_details: args.involvement_extract_entity_details,
        disable_involvement_category_other_warning: args.disable_involvement_category_other_warning,
        involvement_category_other: args.involvement_category_other,
        entities: args.entities,
        involvement: args.involvement,
    };

    let result = csaf::converter::convert(raw, &settings, &mut diag).into_raw();

    if let Err(errors) = validate_schema_csaf_2_1(&RawDocument::<
        csaf::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    >::new(result.clone()))
    {
        errors.iter().for_each(|error| {
            diag.error(
                error.get_data().instance_path.clone().into(),
                OutputValidationError(error.get_data().message.clone()),
            );
        });
    }

    if diag.has_errors {
        if args.allow_invalid {
            eprintln!("Conversion completed with errors, but allowed due to --allow-invalid.");
        } else {
            bail!("Conversion failed due to errors. Run with --allow-invalid to output the converted file anyway.");
        }
    }

    let result = serde_json::to_string_pretty(&result)?;
    if let Some(path) = args.output {
        std::fs::write(path.as_path(), result)?;
        println!("Wrote converted file to {}.", path.display());
    } else {
        println!("{result}");
    }
    Ok(())
}
