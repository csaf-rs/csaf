use anyhow::{bail, Result};
use clap::Parser;
use csaf_rs::csaf::csaf2_0::loader::load_document as load_document_2_0;
use csaf_rs::csaf::csaf2_1::loader::load_document as load_document_2_1;
use csaf_rs::csaf::validation::{
    validate_by_preset, validate_by_tests, ValidationPreset, ValidationResult,
};
use std::str::FromStr;

/// A validator for CSAF documents
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the CSAF document to validate (not used with --web)
    #[arg()]
    path: Option<String>,

    /// Version of CSAF to use
    #[arg(short, long, default_value = "2.0")]
    csaf_version: String,

    /// The validation preset to use
    #[arg(short, long, default_value = "basic")]
    preset: String,

    /// Run only the selected tests, may be specified multiple times
    #[arg(short, long, action = clap::ArgAction::Append)]
    test_id: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let path = args
        .path
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Path argument is required"))?;

    validate_file(path, &args)
}

/// Try to validate a file as a CSAF document based on the specified version.
fn validate_file(path: &str, args: &Args) -> Result<()> {
    match args.csaf_version.as_str() {
        "2.0" => {
            let document = load_document_2_0(path)?;
            validate_document(document, "2.0", args)
        }
        "2.1" => {
            let document = load_document_2_1(path)?;
            validate_document(document, "2.1", args)
        }
        _ => bail!(format!("Invalid CSAF version: {}", args.csaf_version)),
    }
}

/// Validate a CSAF document of the specified version with the provided arguments.
///
/// This prints the results of the tests on stdout.
fn validate_document<T>(document: T, version: &str, args: &Args) -> Result<()>
where
    T: csaf_rs::csaf::validation::Validatable<T>,
{
    let preset = ValidationPreset::from_str(args.preset.as_str())
        .map_err(|_| anyhow::anyhow!("Invalid validation preset: {}", args.preset))?;

    let result = if !args.test_id.is_empty() {
        // Individual test validation
        let test_ids: Vec<&str> = args.test_id.iter().map(|s| s.as_str()).collect();
        validate_by_tests(&document, version, preset, &test_ids)
    } else {
        // Preset validation
        validate_by_preset(&document, version, preset)
    };

    print_validation_result(&result);
    Ok(())
}

/// Print a validation result to stdout (for CLI use)
pub fn print_validation_result(result: &ValidationResult) {
    println!("CSAF Version: {}", result.version);
    println!("Validating document with {:?} preset...\n", result.preset);

    // Print individual test results
    for test_result in &result.test_results {
        if test_result.success {
            println!("Executing Test {}... ✅ Success", test_result.test_id);
        } else if let Some(error) = test_result.errors.first() {
            if error.message.contains("not found") {
                println!(
                    "Executing Test {}... ⚠️ Test not found",
                    test_result.test_id
                );
            } else {
                println!(
                    "Executing Test {}... ❌ Error: {}",
                    test_result.test_id, error.message
                );
            }
        }
    }

    // Print summary
    println!();
    if result.success {
        println!("✅ Validation passed! No errors found.\n");
    } else {
        println!(
            "❌ Validation failed with {} error(s)\n",
            result.errors.len()
        );
    }
}
