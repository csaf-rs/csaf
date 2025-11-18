use anyhow::{Result, bail};
use clap::Parser;
use csaf::csaf2_0::loader::load_document as load_document_2_0;
use csaf::csaf2_1::loader::load_document as load_document_2_1;
use csaf::validation::{
    TestResult,
    TestResultStatus::{Failure, NotFound, Success}, ValidationPreset, ValidationResult, validate_by_preset, validate_by_tests,
};
use std::str::FromStr;

#[cfg(feature = "web")]
mod web;

/// A validator for CSAF documents
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the CSAF document to validate (not used with --web)
    #[arg()]
    path: Option<String>,

    /// Start the web server instead of validating a file
    #[cfg(feature = "web")]
    #[arg(long)]
    web: bool,

    /// Host to bind the web server to (only with --web)
    #[cfg(feature = "web")]
    #[arg(long, default_value = "127.0.0.1", requires = "web")]
    host: String,

    /// Port to bind the web server to (only with --web)
    #[cfg(feature = "web")]
    #[arg(long, default_value = "8080", requires = "web")]
    port: u16,

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

#[cfg(feature = "web")]
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // If web mode is enabled, start the web server
    if args.web {
        return web::start_server(&args.host, args.port).await;
    }

    // Otherwise, validate a file
    let path = args
        .path
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Path argument is required when not using --web"))?;

    validate_file(path, &args)
}

#[cfg(not(feature = "web"))]
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
    T: csaf::validation::Validatable<T>,
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
        print_test_result(test_result);
    }

    // Print summary
    println!();
    if result.success {
        println!("✅ Validation passed! No errors found.\n");
    } else {
        println!("❌ Validation failed with {} error(s)\n", result.num_errors,);
    }
}

/// Print individual test result to stdout.
fn print_test_result(test_result: &TestResult) {
    // Common prefix for all test statuses
    let prefix = format!("Executing Test {} ... ", test_result.test_id);

    match &test_result.status {
        Success => {
            // Yay, success!
            println!("{}✅ Success", prefix);
        },
        Failure { errors } => {
            // We want to print multiple errors nicely indented
            let error_msg = "❌ ";
            print!("{}{}", prefix, error_msg);
            let indent = " ".repeat(prefix.len() + error_msg.len());
            for (i, error) in errors.iter().enumerate() {
                if i > 0 {
                    print!("{}", indent);
                }
                println!("Error: {}", error.message);
            }
        },
        NotFound => {
            // Test not found
            println!("{}⚠️  Test not found", prefix);
        },
    }
}
