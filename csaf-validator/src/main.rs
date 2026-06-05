use anstream::println;
use anyhow::{Result, bail};
use clap::{CommandFactory, Parser};
use csaf::csaf::loader::detect_version;
use csaf::csaf2_0::loader::load_document as load_document_2_0;
use csaf::csaf2_1::loader::load_document as load_document_2_1;
use csaf::schema::csaf2_1::testresult_schema::{
    JsonSchema, NumberOfTheTest, ResultT, TestResultForASingleCsafTestFile, ValidationMessageT, ValidationMessagesT,
};
use csaf::validation::ValidationError;
use csaf::validation::{
    TestResult,
    TestResultStatus::{Failure, NotFound, Skipped, Success},
    Validatable, ValidationResult, validate_by_tests,
};
use std::ops::Deref;

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

    /// Enables verbose output, showing additional information about the validation process and results
    #[arg(short = 'v', long)]
    verbose: bool,

    /// Create a test result JSON for the given test ID as primary result, with other test failed results as secondary results
    #[arg(long, value_name = "TEST_ID")]
    create_test_result: Option<String>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    if args.path.is_empty() {
        Args::command().print_help()?;
        return Ok(());
    }

    if args.create_test_result.is_some() && args.path.len() != 1 {
        bail!("--create-test-result requires exactly one input file");
    }

    if args
        .path
        .iter()
        .map(|file| validate_file(file.deref(), &args))
        .filter(|result| match result {
            Ok(result) => !result.success,
            Err(err) => {
                eprintln!("{err}");
                true
            },
        })
        .count()
        > 0
        && args.create_test_result.is_none()
    {
        bail!("One or more files failed validation");
    }
    Ok(())
}

/// Try to validate a file as a CSAF document based on the specified version.
fn validate_file(path: &str, args: &Args) -> Result<ValidationResult> {
    if args.create_test_result.is_none() {
        let file_color = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Cyan.into()));
        println!("Validating file: {file_color}{path}{file_color:#}");
    }
    let version = match args.csaf_version.as_str() {
        "auto" => detect_version(path)?,
        other => other.to_string(),
    };
    let result = match version.as_str() {
        "2.0" => {
            let document = load_document_2_0(path)?;
            validate_document(document, "2.0", args)
        },
        "2.1" => {
            let document = load_document_2_1(path)?;
            validate_document(document, "2.1", args)
        },
        _ => bail!(format!("Invalid CSAF version: {version}")),
    };
    if let Some(primary_id) = &args.create_test_result {
        let testresult_json = build_testresult_json(&result, primary_id)?;
        println!("{}", serde_json::to_string_pretty(&testresult_json)?);
    }
    Ok(result)
}

/// Validate a CSAF document of the specified version with the provided arguments.
///
/// This prints the results of the tests on stdout.
fn validate_document<T>(document: T, version: &str, args: &Args) -> ValidationResult
where
    T: Validatable,
{
    let mut test_ids: Vec<&str> = args
        .test
        .iter()
        .flat_map(|test_or_preset| match T::tests_in_preset(test_or_preset) {
            Some(test_ids) => test_ids,
            None => vec![test_or_preset.as_str()],
        })
        .collect();
    if let Some(primary_id) = &args.create_test_result
        && !test_ids.contains(&primary_id.as_str())
    {
        test_ids.push(primary_id.as_str());
    }

    let result = validate_by_tests(&document, version, &test_ids);

    if args.create_test_result.is_none() {
        print_validation_result(&result, args.verbose);
    }
    result
}

/// Print a validation result to stdout (for CLI use)
pub fn print_validation_result(result: &ValidationResult, verbose: bool) {
    if verbose {
        println!("CSAF Version: {}", result.version);
    }

    // Print individual test results
    for test_result in &result.test_results {
        if verbose {
            print_individual_test_result(test_result);
        }
        print_individual_test_failures(test_result);
    }

    // Print summary
    match (result.num_errors, result.num_warnings, result.num_infos) {
        (0, 0, 0) => println!("✅  Validation passed! No errors found."),
        (0, 0, infos) => println!("💡  Validation passed with {infos} info(s)."),
        (0, warnings, infos) => println!("⚠️  Validation passed with {warnings} warning(s) and {infos} info(s)."),
        (errors, warnings, infos) => {
            println!("❌  Validation failed with {errors} error(s), {warnings} warning(s) and {infos} info(s).")
        },
    }

    if result.num_not_found > 0 {
        let bold = anstyle::Style::new().underline();
        println!(
            "{bold}Note:{bold:#} {} test(s) were not found during validation.",
            result.num_not_found
        );
    }
    println!();
}

/// Print individual test result to stdout.
fn print_individual_test_result(test_result: &TestResult) {
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

/// Print individual information about test failures, warnings, and infos.
fn print_individual_test_failures(test_result: &TestResult) {
    if let Failure {
        errors,
        warnings,
        infos,
    } = &test_result.status
    {
        let test_id = &test_result.test_id;
        let path_color = anstyle::Style::new().dimmed();
        for ValidationError { message, instance_path } in errors {
            let color = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Red.into()));
            println!(" {path_color}{instance_path}{path_color:#}: {color}{message} [Error {test_id}]{color:#}");
        }
        for ValidationError { message, instance_path } in warnings {
            let color = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Yellow.into()));
            println!(" {path_color}{instance_path}{path_color:#}: {color}{message} [Warning {test_id}]{color:#}");
        }
        for ValidationError { message, instance_path } in infos {
            let color = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Blue.into()));
            println!(" {path_color}{instance_path}{path_color:#}: {color}{message} [Info {test_id}]{color:#}");
        }
    }
}

/// Convert a slice of [`ValidationError`]s to a [`ValidationMessagesT`], or `None` if empty.
fn testresult_create_validationmessages(errors: &[ValidationError]) -> Option<ValidationMessagesT> {
    if errors.is_empty() {
        return None;
    }
    let messages: Vec<ValidationMessageT> = errors
        .iter()
        .filter_map(|e| {
            Some(ValidationMessageT {
                instance_path: e.instance_path.clone(),
                message: e.message.parse().ok()?,
            })
        })
        .collect();
    if messages.is_empty() {
        None
    } else {
        Some(ValidationMessagesT(messages))
    }
}

/// Convert a [`TestResult`] to a [`ResultT`], returning `None` for tests that were not found or skipped,
/// or whose test ID does not match the required pattern.
fn testresult_create_result(test_result: &TestResult) -> Option<ResultT> {
    let id: NumberOfTheTest = test_result.test_id.parse().ok()?;
    match &test_result.status {
        Success => Some(ResultT {
            id,
            passed: true,
            errors: None,
            warnings: None,
            infos: None,
        }),
        Failure {
            errors,
            warnings,
            infos,
        } => Some(ResultT {
            id,
            passed: false,
            errors: testresult_create_validationmessages(errors),
            warnings: testresult_create_validationmessages(warnings),
            infos: testresult_create_validationmessages(infos),
        }),
        NotFound | Skipped => None,
    }
}

/// Build a [`TestResultForASingleCsafTestFile`] from a [`ValidationResult`] with the given primary test ID.
fn build_testresult_json(result: &ValidationResult, primary_test_id: &str) -> Result<TestResultForASingleCsafTestFile> {
    let primary_test = result
        .test_results
        .iter()
        .find(|r| r.test_id == primary_test_id)
        .ok_or_else(|| anyhow::anyhow!("Test '{}' not found in validation results", primary_test_id))?;

    let primary_result = testresult_create_result(primary_test).ok_or_else(|| {
        anyhow::anyhow!(
            "Cannot create result for test '{}': test ID does not match the required pattern, or test was not found/skipped",
            primary_test_id
        )
    })?;

    let secondary_results: Vec<ResultT> = result
        .test_results
        .iter()
        .filter(|r| r.test_id != primary_test_id)
        .filter_map(testresult_create_result)
        .filter(|r| !r.passed)
        .collect();

    Ok(TestResultForASingleCsafTestFile {
        schema: JsonSchema::HttpsRawGithubusercontentComOasisTcsCsafMasterCsaf21TestValidatorTestresultJsonSchemaJson,
        overall_valid: result.success,
        primary_result,
        resultschema_version: "2.1".to_string(),
        secondary_results: if secondary_results.is_empty() {
            None
        } else {
            Some(secondary_results)
        },
    })
}
