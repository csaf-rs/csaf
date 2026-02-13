use anyhow::{Result, bail};
use clap::{CommandFactory, Parser};
use csaf::csaf::loader::detect_version;
use csaf::csaf2_0::loader::load_document as load_document_2_0;
use csaf::csaf2_1::loader::load_document as load_document_2_1;
use csaf::validation::{
    TestResult,
    TestResultStatus::{Failure, NotFound, Skipped, Success},
    Validatable, ValidationPreset, ValidationResult, validate_by_tests,
};
use std::ops::Deref;
use std::str::FromStr;

/// A validator for CSAF documents
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the CSAF document(s) to validate
    #[arg(action = clap::ArgAction::Append)]
    path: Vec<String>,

    /// Version of CSAF to use
    #[arg(short = 'C', long, default_value = "auto")]
    csaf_version: String,

    /// The validation preset or tests to use
    #[arg(short = 'T', long, default_value = "basic", action = clap::ArgAction::Append)]
    test: Vec<String>,

    #[arg(short = 'v', long)]
    verbose: bool
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    if args.path.is_empty() {
        Args::command().print_help()?;
        println!();
        bail!("PATH is required");
    }

    if args
        .path
        .iter()
        .map(|file| validate_file(file.deref(), &args))
        .filter(|result| match result {
            Ok(_) => false,
            Err(err) => {
                println!("{err}\n");
                true
            },
        })
        .count()
        > 0
    {
        bail!("One or more files failed validation");
    }
    Ok(())
}

/// Try to validate a file as a CSAF document based on the specified version.
fn validate_file(path: &str, args: &Args) -> Result<()> {
    println!("Validating file: {path}");
    let version = match args.csaf_version.as_str() {
        "auto" => detect_version(path)?,
        other => other.to_string(),
    };
    match version.as_str()
    {
        "2.0" => {
            let document = load_document_2_0(path)?;
            validate_document(document, "2.0", args)
        },
        "2.1" => {
            let document = load_document_2_1(path)?;
            validate_document(document, "2.1", args)
        },
        _ => bail!(format!("Invalid CSAF version: {}", args.csaf_version)),
    }
}

/// Validate a CSAF document of the specified version with the provided arguments.
///
/// This prints the results of the tests on stdout.
fn validate_document<T>(document: T, version: &str, args: &Args) -> Result<()>
where
    T: Validatable,
{
    let test_ids: Vec<_> = args
        .test
        .iter()
        .flat_map(|test_or_preset| {
            ValidationPreset::from_str(test_or_preset.as_str())
                .map_or(vec![test_or_preset.as_str()], |preset| T::tests_in_preset(&preset))
        })
        .collect();

    let result = validate_by_tests(&document, version, &test_ids);

    print_validation_result(&result, args.verbose);
    match result.num_errors {
        0 => Ok(()),
        _ => Err(anyhow::anyhow!("Validation failed with {} error(s)", result.num_errors)),
    }
}

/// Print a validation result to stdout (for CLI use)
pub fn print_validation_result(result: &ValidationResult, verbose: bool) {
    if verbose {
        println!("CSAF Version: {}", result.version);
        println!("Validating document...\n");

        // Print individual test results
        for test_result in &result.test_results {
            print_test_result(test_result);
        }

        // Print summary
        println!();
        println!();
    }
    match (result.num_errors, result.num_warnings, result.num_infos) {
        (0, 0, 0) => println!("✅  Validation passed! No errors found.\n"),
        (0, 0, infos) => println!("💡  Validation passed with {infos} info(s)\n"),
        (0, warnings, infos) => println!("⚠️  Validation passed with {warnings} warning(s) and {infos} info(s)\n"),
        (errors, warnings, infos) => {
            println!("❌  Validation failed with {errors} error(s), {warnings} warning(s) and {infos} info(s)\n")
        },
    }

    if verbose && result.num_not_found > 0 {
        println!(
            "Note: {} test(s) were not found during validation.\n",
            result.num_not_found
        );
    }
}

/// Print individual test result to stdout.
fn print_test_result(test_result: &TestResult) {
    // Common prefix for all test statuses
    let prefix = format!("Executing Test {:10} ... ", test_result.test_id);
    print!("{prefix}");

    match &test_result.status {
        Success => {
            // Yay, success!
            println!("✅  Success");
        },
        Failure {
            errors,
            warnings,
            infos,
        } => {
            if !errors.is_empty() {
                println!("❌ {} error(s) found", errors.len());
            } else if !warnings.is_empty() {
                println!("⚠️  {} warning(s) found", warnings.len());
            } else {
                println!("💡  {} info(s) found", infos.len());
            };
            for error in errors {
                println!(
                    "❌  {}: {} [{}]",
                    test_result.test_id, error.message, error.instance_path
                );
            }
            for warning in warnings {
                println!(
                    "⚠️  {}: {} [{}]",
                    test_result.test_id, warning.message, warning.instance_path
                );
            }
            for info in infos {
                println!("💡  {}: {} [{}]", test_result.test_id, info.message, info.instance_path);
            }
        },
        NotFound => {
            // Test not found
            println!("❓  Test not found");
        },
        Skipped => {
            // Test skipped
            println!("⏭️  Test skipped");
        },
    }
}
